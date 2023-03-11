extern crate fontdue;
extern crate gl;
extern crate glam;
use crate::gl_wrap::{Bind, Buffer, Drop, Program, Texture, TextureFramebuffer, VertexArray};
use crate::vertices::{bmp_arr, bmp_vert, BitmapVert};
use crate::vertices::{bmp_to_text_vert, TextVert};
use fontdue::{Font, FontSettings};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;

pub const VERT_PER_CHAR: usize = 6; // num vertices per char in output vertex data
static DEFAULT_FONT: &str = "./resources/Ubuntu-Regular.ttf";
static LABEL_MARGIN: f32 = 0.1;
static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.";
static FONT_SIZE: f32 = 30.0;
static FONT_SUPERSAMPLE: f32 = 3.0;
static MAP_SIZE: [f32; 2] = [1024.0, 512.0];
static NUM_VERTEX: i32 = 4;
static VERTICES: [BitmapVert; 4] = bmp_arr![
    [0.5, 1.0, 1.0, 0.0],
    [0.5, 0.0, 1.0, 1.0],
    [-0.5, 1.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 1.0]
];

pub struct FontMapper {
    program: Program,
    vao: VertexArray,
    buffer: Buffer,
    uniforms: FontMapperUniforms,
    chars: Vec<char>,
    window_size: [i32; 2],
}

struct FontMapperUniforms {
    pub char_size: i32,
    pub offset: i32,
}

impl FontMapper {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, FontMapperError> {
        // init gl resources for font bitmap creation
        const VERT: &str = "./shaders/bitmap_vert.glsl";
        const FRAG: &str = "./shaders/bitmap_frag.glsl";
        let program = Program::new_from_files(VERT, FRAG)?;
        let vao = VertexArray::new();
        let buffer = Buffer::new_from(&VERTICES, gl::STATIC_DRAW);
        let pos_loc = program.get_attrib_location("position")?;
        let tcoord_loc = program.get_attrib_location("a_texCoord")?;
        vao.set_attribute::<BitmapVert>(pos_loc, 2, 0);
        vao.set_attribute::<BitmapVert>(tcoord_loc, 2, 2);

        // init program uniforms / get locations
        program.bind();
        let uniforms: FontMapperUniforms;
        let char_size_cname = CString::new("char_size")?;
        let offset_cname = CString::new("offset")?;
        let map_size_cname = CString::new("map_size")?;
        unsafe {
            // store uniform locations set during font map generation
            uniforms = FontMapperUniforms {
                char_size: gl::GetUniformLocation(program.id, char_size_cname.as_ptr()),
                offset: gl::GetUniformLocation(program.id, offset_cname.as_ptr()),
            };
            // don't need to store map size location since value is static and set once
            let map_size_loc = gl::GetUniformLocation(program.id, map_size_cname.as_ptr());
            gl::Uniform2fv(map_size_loc, 1, &MAP_SIZE[0]);
        }

        let chars: Vec<char> = CHAR_SET.chars().collect();
        let window_size = [window_width, window_height];

        Ok(Self {
            program,
            vao,
            buffer,
            chars,
            uniforms,
            window_size,
        })
    }

    // create texture with rasterized chars for single font face
    // return finished font map texture, vector of character vertex data,
    // and hashmap to convert character to index in vertex data
    pub fn gen_font_map(&self, font_file: &str) -> Result<FontMap, FontMapperError> {
        let mut vertices = Vec::<BitmapVert>::new();
        let mut indices = HashMap::<char, usize>::new();
        let font = FontMapper::get_fontdue(font_file)?;
        let framebuffer = self.new_tex_fb()?;

        // bind constant gl resources
        framebuffer.bind();
        self.program.bind();
        self.vao.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // character layout params
        let margin: f32 = FONT_SIZE * 0.5;
        let line_height: f32 = FONT_SIZE * 1.25;
        let char_per_row = (MAP_SIZE[0] / FONT_SIZE).floor() as usize;
        let avg_alignment = margin * 0.5; // offset to correct for vertical char alignment

        // rasterize and draw characters to font map, storing character position info
        for i in 0..self.chars.len() {
            let (metrics, bitmap) = font.rasterize(self.chars[i], FONT_SIZE * FONT_SUPERSAMPLE);

            // calc uniforms for character position
            let grid_x = (i % char_per_row) as f32 * FONT_SIZE + margin;
            let grid_y = (i / char_per_row) as f32 * line_height + margin;
            let char_alignment = metrics.bounds.ymin / FONT_SUPERSAMPLE;
            let offset: [f32; 2] = [grid_x, grid_y + char_alignment];
            let char_size: [f32; 2] = [
                metrics.width as f32 / FONT_SUPERSAMPLE,
                metrics.height as f32 / FONT_SUPERSAMPLE,
            ];

            let rgba = FontMapper::rgba_from_bytes(bitmap);
            let texture = Texture::new(&rgba, metrics.width as i32, metrics.height as i32);
            unsafe {
                gl::Uniform2fv(self.uniforms.char_size, 1, &char_size[0]);
                gl::Uniform2fv(self.uniforms.offset, 1, &offset[0]);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, NUM_VERTEX);
            }
            texture.drop(); // free texture since only one draw needed

            let start_ind = vertices.len();
            indices.insert(self.chars[i], start_ind);

            // quad size
            let w = char_size[0];
            let h = line_height;
            let w2 = w * 0.5;
            let h2 = h * 0.5;
            // font map texture coords for +/- x/y
            let tpx = (grid_x + w2) / MAP_SIZE[0];
            let tnx = (grid_x - w2) / MAP_SIZE[0];
            let tpy = (grid_y + h2 + avg_alignment) / MAP_SIZE[1];
            let tny = (grid_y - h2 + avg_alignment) / MAP_SIZE[1];
            let mut quad = FontMapper::get_quad(w, h, tpx, tnx, tpy, tny);
            vertices.append(&mut quad);
        }
        // free texture framebuffer for finished font map and bind default
        framebuffer.bind_default();
        framebuffer.drop();

        // return finished font map
        let fontmap = FontMap {
            texture: framebuffer.texture,
            verts: vertices,
            inds: indices,
        };
        Ok(fontmap)
    }

    // get fontdue font for rasterization
    fn get_fontdue(font_file: &str) -> Result<Font, FontMapperError> {
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font = Font::from_bytes(font_bytes, FontSettings::default())?;
        Ok(font)
    }

    // get new texture framebuffer of fixed size
    fn new_tex_fb(&self) -> Result<TextureFramebuffer, FontMapperError> {
        let framebuffer = TextureFramebuffer::new(
            MAP_SIZE[0] as i32,
            MAP_SIZE[1] as i32,
            self.window_size[0],
            self.window_size[1],
        )?;
        Ok(framebuffer)
    }

    // get quad vertex data from dims and +/- texture coords
    fn get_quad(w: f32, h: f32, tpx: f32, tnx: f32, tpy: f32, tny: f32) -> Vec<BitmapVert> {
        let w2 = w * 0.5;
        let h2 = h * 0.5;
        bmp_vert![
            [w2, h2, tpx, tpy],
            [-w2, h2, tnx, tpy],
            [-w2, -h2, tnx, tny],
            [-w2, -h2, tnx, tny],
            [w2, -h2, tpx, tny],
            [w2, h2, tpx, tpy]
        ]
    }

    // get rgba data from byte array for compatibility with gl color formats
    fn rgba_from_bytes(bytes: Vec<u8>) -> Vec<u8> {
        let mut rgba: Vec<u8> = vec![0; bytes.len() * 4];
        for i in 0..bytes.len() {
            for j in 0..4 {
                rgba[i * 4 + j] = bytes[i];
            }
        }
        rgba
    }
}

impl Drop for FontMapper {
    fn drop(&self) {
        self.program.drop();
        self.vao.drop();
        self.buffer.drop();
    }
}

pub struct FontMap {
    pub texture: Texture,
    pub verts: Vec<BitmapVert>,
    pub inds: HashMap<char, usize>,
}

impl FontMap {
    pub fn get_verts(
        &self,
        label: &str,
        params: &TextParams,
        position: [f32; 3],
    ) -> Result<Vec<TextVert>, FontMapError> {
        let mut vertices = Vec::<TextVert>::new();
        let mut offset: f32 = 0.0;
        for c in label.chars() {
            if let ' ' = c {
                // add fixed width for space character
                offset += params.size;
                continue;
            }
            // get start index of vertex data if char exists in font texture
            let vert_ind = match self.inds.get(&c) {
                Some(&index) => index,
                None => return Err(FontMapError::Character(c)),
            };
            // character width / 2 from first vertex x coordinate
            let char_spacing = self.verts[vert_ind].position[0] + params.kearning;
            offset += char_spacing;
            for i in 0..VERT_PER_CHAR {
                let mut vert = bmp_to_text_vert!(self.verts[i + vert_ind], position);
                vert.offset[0] += offset; // layout text on x axis
                vertices.push(vert);
            }
            offset += char_spacing;
        }
        // center text about origin
        let mid_width = offset * 0.5;
        for vert in &mut vertices {
            vert.offset[0] -= mid_width;
        }
        Ok(vertices)
    }
}

impl Drop for FontMap {
    fn drop(&self) {
        self.texture.drop();
    }
}

pub struct TextParams {
    pub font: String,
    pub size: f32,
    pub kearning: f32,
}

impl TextParams {
    pub fn new() -> Self {
        Self {
            font: DEFAULT_FONT.to_string(),
            size: FONT_SIZE,
            kearning: 0.0,
        }
    }
}

extern crate thiserror;
use crate::gl_wrap::{FramebufferError, ProgramError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FontMapperError {
    #[error("{0}")]
    Program(#[from] ProgramError),
    #[error("{0}")]
    Framebuffer(#[from] FramebufferError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    String(#[from] std::ffi::NulError),
    #[error("{0}")]
    Font(String),
    #[error("Invalid font file")]
    FontFile,
}

impl From<&str> for FontMapperError {
    fn from(s: &str) -> Self {
        Self::Font(s.to_string())
    }
}

#[derive(Error, Debug)]
pub enum FontMapError {
    #[error("Invalid character '{0}'")]
    Character(char),
}
