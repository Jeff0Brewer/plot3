extern crate fontdue;
use crate::vertices::{BitmapVert, bmp_vert};
use crate::gl_wrap::{TextureFramebuffer, Program, Buffer, VertexArray, Texture, Bind, Drop};
use fontdue::{Font, FontSettings};
use std::ffi::CString;
use std::fs;

pub struct Bitmap {
    framebuffer: TextureFramebuffer,
    program: Program,
    vao: VertexArray,
    buffer: Buffer,
    chars: Vec<char>,
    locations: BitmapUniforms
}

struct BitmapUniforms {
    pub offset: i32,
    pub dimensions: i32,
    pub scale: i32
}

impl Bitmap {
    pub fn new() -> Result<Self, BitmapError> {
        let framebuffer = TextureFramebuffer::new(1024, 512)?;
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

        let offset_cname = CString::new("offset")?;
        let dimensions_cname = CString::new("dimensions")?;
        let scale_cname = CString::new("scale")?;
        let locations: BitmapUniforms;
        unsafe {
            locations = BitmapUniforms {
                offset: gl::GetUniformLocation(program.id, offset_cname.as_ptr()),
                dimensions: gl::GetUniformLocation(program.id, dimensions_cname.as_ptr()),
                scale: gl::GetUniformLocation(program.id, scale_cname.as_ptr())
            };
        }

        let chars: Vec<char> = CHAR_SET.chars().collect();
        Ok(
            Self { framebuffer, program, vao, buffer, chars, locations }
        )
    }

    pub fn gen_font_map(&self, font_file: &str) -> Result<(), BitmapError> {
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font = Font::from_bytes(font_bytes, FontSettings::default())?;
        self.program.bind();
        self.vao.bind();
        const TEXTURE_SIZE: f32 = 800.0;
        let scale = FONT_SIZE / TEXTURE_SIZE;
        let mut offset: [f32; 2] = [-1.0 + scale, -1.0 + scale];
        unsafe { gl::Uniform1f(self.locations.scale, scale); }
        for i in 0..self.chars.len() {
            let (metrics, bitmap) = font.rasterize(self.chars[i], FONT_SIZE * FONT_SUPERSAMPLE);
            let dimensions: [f32; 2] = [
                metrics.width as f32 / FONT_SIZE,
                metrics.height as f32 / FONT_SIZE
            ];
            unsafe {
                gl::Uniform2fv(self.locations.offset, 1, &offset[0]);
                gl::Uniform2fv(self.locations.dimensions, 1, &dimensions[0]);
            }
            offset[0] += scale;
            if offset[0] > 1.0 {
                offset[0] = -1.0;
                offset[1] += scale;
            }
            let data = rgba_from_bitmap(bitmap);
            let texture = Texture::new(&data, metrics.width as i32, metrics.height as i32);
            unsafe { gl::DrawArrays(gl::TRIANGLE_STRIP, 0, NUM_VERTEX); }
            texture.drop();
        }
        Ok(())
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

fn rgba_from_bitmap(bitmap: Vec<u8>) -> Vec<u8> {
    let mut rgba: Vec<u8> = vec![0; bitmap.len() * 4];
    for i in 0..bitmap.len() {
        for j in 0..3 {
            rgba[i*4 + j] = bitmap[i];
        }
        rgba[i*4 + 3] = 255;
    }
    rgba
}

static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static FONT_SIZE: f32 = 30.0;
static FONT_SUPERSAMPLE: f32 = 2.0;
static NUM_VERTEX: i32 = 4;
static VERTICES: [BitmapVert; 4] = bmp_vert![
    [0.5, 0.5, 1.0, 0.0],
    [0.5, -0.5, 1.0, 1.0],
    [-0.5, 0.5, 0.0, 0.0],
    [-0.5, -0.5, 0.0, 1.0]
];

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
