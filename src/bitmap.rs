extern crate fontdue;
extern crate gl;
use crate::vertices::{BitmapVert, bmp_vert};
use crate::gl_wrap::{TextureFramebuffer, Program, Buffer, VertexArray, Texture, Bind, Drop};
use fontdue::{Font, FontSettings};
use gl::types::GLuint;
use std::ffi::CString;
use std::fs;

pub struct Bitmap {
    framebuffer: TextureFramebuffer,
    program: Program,
    vao: VertexArray,
    buffer: Buffer,
    chars: Vec<char>,
    uniforms: BitmapUniforms
}

struct BitmapUniforms {
    pub map_size: i32,
    pub char_size: i32,
    pub offset: i32,
}

impl Bitmap {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, BitmapError> {
        let framebuffer = TextureFramebuffer::new(1024, 512, window_width, window_height)?;
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
        Ok(Self { framebuffer, program, vao, buffer, chars, uniforms })
    }

    // create texture with rasterized chars for single font face
    pub fn gen_font_map(&self, font_file: &str) -> Result<GLuint, BitmapError> {
        let map_size: [f32; 2] = [
            self.framebuffer.width as f32,
            self.framebuffer.height as f32
        ];
        // character layout params
        let char_per_row = (map_size[0] / FONT_SIZE).floor() as usize;
        let padding: f32 = FONT_SIZE * 0.5;
        let line_height: f32 = FONT_SIZE * 1.25;

        // get fontdue font for rasterization
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font = Font::from_bytes(font_bytes, FontSettings::default())?;

        // bind constant gl resources
        self.framebuffer.bind();
        self.program.bind();
        self.vao.bind();
        unsafe {
            gl::Uniform2fv(self.uniforms.map_size, 1, &map_size[0]);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // rasterize and draw characters to texture
        for i in 0..self.chars.len() {
            let (metrics, bitmap) = font.rasterize(self.chars[i], FONT_SIZE * FONT_SUPERSAMPLE);
            let char_size: [f32; 2] = [
                metrics.width as f32 / FONT_SUPERSAMPLE,
                metrics.height as f32 / FONT_SUPERSAMPLE
            ];
            let char_alignment = metrics.bounds.ymin / FONT_SUPERSAMPLE;
            let off_x = (i % char_per_row) as f32 * FONT_SIZE;
            let off_y = (i / char_per_row) as f32 * line_height;
            let offset: [f32; 2] = [
                off_x + padding,
                off_y + char_alignment + padding
            ];
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
        }
        // unbind texture framebuffer
        self.framebuffer.bind_default();

        // return finished font texture id
        Ok(self.framebuffer.tex_id)
    }
}

impl Drop for Bitmap {
    fn drop(&self) {
        self.framebuffer.drop();
        self.program.drop();
        self.vao.drop();
        self.buffer.drop();
    }
}

static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static FONT_SIZE: f32 = 31.0;
static FONT_SUPERSAMPLE: f32 = 2.0;
static NUM_VERTEX: i32 = 4;
static VERTICES: [BitmapVert; 4] = bmp_vert![
    [0.5, 1.0, 1.0, 0.0],
    [0.5, 0.0, 1.0, 1.0],
    [-0.5, 1.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 1.0]
];

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
