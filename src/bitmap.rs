extern crate fontdue;
use crate::vertices::{BitmapVert, bmp_vert};
use crate::gl_wrap::{TextureFramebuffer, Program, Buffer, VertexArray, Texture, Bind, Drop};
use fontdue::{Font, FontSettings};
use std::fs;

pub struct Bitmap {
    framebuffer: TextureFramebuffer,
    program: Program,
    vao: VertexArray,
    buffer: Buffer,
    chars: Vec<char>
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

        let chars: Vec<char> = CHAR_SET.chars().collect();
        Ok(
            Self { framebuffer, program, vao, buffer, chars }
        )
    }

    pub fn gen_font_map(&self, font_file: &str) -> Result<(), BitmapError> {
        let font_bytes = &fs::read(font_file)? as &[u8];
        let font_result = Font::from_bytes(font_bytes, FontSettings::default());
        // verbose error check since FontResult returns &str for error type
        if let Err(err) = font_result {
            return Err(BitmapError::FontError(err.to_string()));
        }
        let font = font_result.unwrap();
        self.program.bind();
        for &c in &self.chars {
            let (metrics, bitmap) = font.rasterize(c, FONT_SIZE);
            let texture = Texture::new(&bitmap, metrics.width as i32, metrics.height as i32);
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

static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static FONT_SIZE: f32 = 20.0;
static VERTICES: [BitmapVert; 4] = bmp_vert![
    [0.5, 0.5, 1.0, 1.0],
    [0.5, -0.5, 1.0, 0.0],
    [-0.5, -0.5, 0.0, 0.0],
    [-0.5, 0.5, 0.0, 1.0]
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
    FontError(String)
}
