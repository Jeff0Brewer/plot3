extern crate fontdue;
extern crate gl;
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
    window_dims: [i32; 2],
    map_dims: [f32; 2]
}

struct BitmapUniforms {
    pub map_size: i32,
    pub char_size: i32,
    pub offset: i32,
}

impl Bitmap {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, BitmapError> {
        let window_dims = [window_width, window_height];
        let map_dims = [1024.0, 512.0];
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

        let map_size_cname = CString::new("map_size")?;
        let char_size_cname = CString::new("char_size")?;
        let offset_cname = CString::new("offset")?;
        let uniforms: BitmapUniforms;
        unsafe {
            uniforms = BitmapUniforms {
                map_size: gl::GetUniformLocation(program.id, map_size_cname.as_ptr()),
                char_size: gl::GetUniformLocation(program.id, char_size_cname.as_ptr()),
                offset: gl::GetUniformLocation(program.id, offset_cname.as_ptr())
            };
        }

        let chars: Vec<char> = CHAR_SET.chars().collect();
        Ok(Self { program, vao, buffer, chars, uniforms, window_dims, map_dims })
    }

    // create texture with rasterized chars for single font face
    pub fn gen_font_map(&self, font_file: &str)
        -> Result<(Texture, Vec<BitmapVert>, HashMap<char, usize>), BitmapError> {
        // character layout params
        let char_per_row = (self.map_dims[0] / FONT_SIZE).floor() as usize;
        let padding: f32 = FONT_SIZE * 0.5;
        let line_height: f32 = FONT_SIZE * 1.25;

        // get fontdue font for rasterization
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font = Font::from_bytes(font_bytes, FontSettings::default())?;

        // bind constant gl resources
        let framebuffer = TextureFramebuffer::new(
            self.map_dims[0] as i32,
            self.map_dims[1] as i32,
            self.window_dims[0],
            self.window_dims[1]
        )?;
        framebuffer.bind();
        self.program.bind();
        self.vao.bind();
        unsafe {
            gl::Uniform2fv(self.uniforms.map_size, 1, &self.map_dims[0]);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut vertices = Vec::<BitmapVert>::new();
        let mut indices = HashMap::<char, usize>::new();

        // rasterize and draw characters to texture
        for i in 0..self.chars.len() {
            let (metrics, bitmap) = font.rasterize(self.chars[i], FONT_SIZE * FONT_SUPERSAMPLE);

            // set uniforms for character position
            let char_size: [f32; 2] = [
                metrics.width as f32 / FONT_SUPERSAMPLE,
                metrics.height as f32 / FONT_SUPERSAMPLE
            ];
            let char_alignment = metrics.bounds.ymin / FONT_SUPERSAMPLE;
            let grid_x = (i % char_per_row) as f32 * FONT_SIZE + padding;
            let grid_y = (i / char_per_row) as f32 * line_height + padding;
            let offset: [f32; 2] = [grid_x, grid_y + char_alignment];

            // get rgba data from byte array for compatibility with gl color formats
            let rgba = rgba_from_bytes(bitmap);
            let texture = Texture::new(&rgba, metrics.width as i32, metrics.height as i32);
            unsafe {
                gl::Uniform2fv(self.uniforms.char_size, 1, &char_size[0]);
                gl::Uniform2fv(self.uniforms.offset, 1, &offset[0]);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, NUM_VERTEX);
            }
            // free drawn character texture
            texture.drop();

            let len = vertices.len();
            let avg_alignment = padding * 0.5;

            // quad size / 2
            let w2 = FONT_SIZE * 0.5;
            let h2 = line_height * 0.5;
            // bitmap texture coords for +/- x/y
            let tpx = (grid_x + w2) / self.map_dims[0];
            let tnx = (grid_x - w2) / self.map_dims[0];
            let tpy = (grid_y + h2 + avg_alignment) / self.map_dims[1];
            let tny = (grid_y - h2 + avg_alignment) / self.map_dims[1];
            vertices.append(&mut bmp_vert![
                [ w2,  h2, tpx, tpy],
                [-w2,  h2, tnx, tpy],
                [-w2, -h2, tnx, tny],
                [-w2, -h2, tnx, tny],
                [ w2, -h2, tpx, tny],
                [ w2,  h2, tpx, tpy]
            ]);
            indices.insert(self.chars[i], len);
        }
        // free texture framebuffer and bind default
        framebuffer.bind_default();
        framebuffer.drop();

        // return finished font texture with vertex data
        // caller is responsible for freeing texture once used
        Ok((framebuffer.texture, vertices, indices))
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
static FONT_SIZE: f32 = 31.0;
static FONT_SUPERSAMPLE: f32 = 2.0;
static NUM_VERTEX: i32 = 4;
static VERTICES: [BitmapVert; 4] = bmp_arr![
    [0.5, 1.0, 1.0, 0.0],
    [0.5, 0.0, 1.0, 1.0],
    [-0.5, 1.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 1.0]
];
pub const VERT_PER_CHAR: usize = 6; // num vertices per char in output vertex data

fn rgba_from_bytes(bytes: Vec<u8>) -> Vec<u8> {
    let mut rgba: Vec<u8> = vec![0; bytes.len() * 4];
    for i in 0..bytes.len() {
        for j in 0..3 {
            rgba[i*4 + j] = bytes[i];
        }
        rgba[i*4 + 3] = 255;
    }
    rgba
}

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
