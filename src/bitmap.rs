extern crate gl;
extern crate fontdue;
use crate::vertices::{BitmapVert, bmp_vert, bmp_arr};
use crate::gl_wrap::{TextureFramebuffer, Program, Buffer, VertexArray, Texture, Bind, Drop};
use fontdue::{Font, FontSettings};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;

pub struct Bitmap {
    program: Program,
    vao: VertexArray,
    buffer: Buffer,
    chars: Vec<char>,
    uniforms: BitmapUniforms,
    window_size: [i32; 2]
}

struct BitmapUniforms {
    pub char_size: i32,
    pub offset: i32,
}

impl Bitmap {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, BitmapError> {
        let program = Program::new_from_files(
            "./shaders/bitmap_vert.glsl",
            "./shaders/bitmap_frag.glsl"
        )?;
        let pos_loc = program.get_attrib_location("position")?;
        let tcoord_loc = program.get_attrib_location("a_texCoord")?;
        let vao = VertexArray::new();
        let buffer = Buffer::new_from(&VERTICES, gl::STATIC_DRAW);
        vao.set_attribute::<BitmapVert>(pos_loc, 2, 0);
        vao.set_attribute::<BitmapVert>(tcoord_loc, 2, 2);

        program.bind();
        let uniforms: BitmapUniforms;
        let char_size_cname = CString::new("char_size")?;
        let offset_cname = CString::new("offset")?;
        let map_size_cname = CString::new("map_size")?;
        unsafe {
            // store uniform locations set during bitmap generation
            uniforms = BitmapUniforms {
                char_size: gl::GetUniformLocation(program.id, char_size_cname.as_ptr()),
                offset: gl::GetUniformLocation(program.id, offset_cname.as_ptr())
            };
            // don't need to store map size location since value is static and set once
            let map_size_loc = gl::GetUniformLocation(program.id, map_size_cname.as_ptr());
            gl::Uniform2fv(map_size_loc, 1, &MAP_SIZE[0]);
        }

        let chars: Vec<char> = CHAR_SET.chars().collect();
        let window_size = [window_width, window_height];

        Ok(Self { program, vao, buffer, chars, uniforms, window_size })
    }

    // create texture with rasterized chars for single font face
    // return finished texture, vector of character vertex data,
    // and hashmap to convert character to index in vertex data
    pub fn gen_font_map(&self, font_file: &str)
    -> Result<(Texture, Vec<BitmapVert>, HashMap<char, usize>), BitmapError> {
        let mut vertices = Vec::<BitmapVert>::new();
        let mut indices = HashMap::<char, usize>::new();
        let font = Bitmap::get_font(font_file)?;
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

        // rasterize and draw characters to bitmap, storing character position info
        for i in 0..self.chars.len() {
            let (metrics, bitmap) = font.rasterize(self.chars[i], FONT_SIZE * FONT_SUPERSAMPLE);

            // calc uniforms for character position
            let grid_x = (i % char_per_row) as f32 * FONT_SIZE + margin;
            let grid_y = (i / char_per_row) as f32 * line_height + margin;
            let char_alignment = metrics.bounds.ymin / FONT_SUPERSAMPLE;
            let offset: [f32; 2] = [grid_x, grid_y + char_alignment];
            let char_size: [f32; 2] = [
                metrics.width as f32 / FONT_SUPERSAMPLE,
                metrics.height as f32 / FONT_SUPERSAMPLE
            ];

            let rgba = Bitmap::rgba_from_bytes(bitmap);
            let texture = Texture::new(&rgba, metrics.width as i32, metrics.height as i32);
            unsafe {
                gl::Uniform2fv(self.uniforms.char_size, 1, &char_size[0]);
                gl::Uniform2fv(self.uniforms.offset, 1, &offset[0]);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, NUM_VERTEX);
            }
            texture.drop(); // free texture since only one draw needed

            let start_ind = vertices.len();
            indices.insert(self.chars[i], start_ind);

            // quad size / 2
            let w2 = char_size[0] * 0.5;
            let h2 = line_height * 0.5;
            // bitmap texture coords for +/- x/y
            let tpx = (grid_x + w2) / MAP_SIZE[0];
            let tnx = (grid_x - w2) / MAP_SIZE[0];
            let tpy = (grid_y + h2 + avg_alignment) / MAP_SIZE[1];
            let tny = (grid_y - h2 + avg_alignment) / MAP_SIZE[1];
            let mut quad = Bitmap::get_quad(w2, -w2, h2, -h2, tpx, tnx, tpy, tny);
            vertices.append(&mut quad);
        }
        // free texture framebuffer for finished bitmap and bind default
        framebuffer.bind_default();
        framebuffer.drop();

        // return finished font texture with vertex data
        // caller is responsible for freeing bitmap texture once used
        Ok((framebuffer.texture, vertices, indices))
    }

    // get fontdue font for rasterization
    fn get_font(font_file: &str) -> Result<Font, BitmapError> {
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font = Font::from_bytes(font_bytes, FontSettings::default())?;
        Ok(font)
    }

    // get new texture framebuffer of fixed size
    fn new_tex_fb(&self) -> Result<TextureFramebuffer, BitmapError> {
        let framebuffer = TextureFramebuffer::new(
            MAP_SIZE[0] as i32,
            MAP_SIZE[1] as i32,
            self.window_size[0],
            self.window_size[1]
        )?;
        Ok(framebuffer)
    }

    // get quad vertex data from +/- x/y position and texture coords
    fn get_quad(px: f32, nx: f32, py: f32, ny: f32, tpx: f32, tnx: f32, tpy: f32, tny: f32)
    -> Vec<BitmapVert> {
        bmp_vert![
            [px, py, tpx, tpy],
            [nx, py, tnx, tpy],
            [nx, ny, tnx, tny],
            [nx, ny, tnx, tny],
            [px, ny, tpx, tny],
            [px, py, tpx, tpy]
        ]
    }

    // get rgba data from byte array for compatibility with gl color formats
    fn rgba_from_bytes(bytes: Vec<u8>) -> Vec<u8> {
        let mut rgba: Vec<u8> = vec![0; bytes.len() * 4];
        for i in 0..bytes.len() {
            for j in 0..4 {
                rgba[i*4 + j] = bytes[i];
            }
        }
        rgba
    }
}

impl Drop for Bitmap {
    fn drop(&self) {
        self.program.drop();
        self.vao.drop();
        self.buffer.drop();
    }
}

static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
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
pub const VERT_PER_CHAR: usize = 6; // num vertices per char in output vertex data

extern crate thiserror;
use thiserror::Error;
use crate::gl_wrap::{ProgramError, FramebufferError};

#[derive(Error, Debug)]
pub enum BitmapError {
    #[error("{0}")]
    ProgramError(#[from] ProgramError),
    #[error("{0}")]
    FramebufferError(#[from] FramebufferError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    StringError(#[from] std::ffi::NulError),
    #[error("{0}")]
    FontError(String)
}

impl From<&str> for BitmapError {
    fn from(s: &str) -> Self {
        Self::FontError(s.to_string())
    }
}
