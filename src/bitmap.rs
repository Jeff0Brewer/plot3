extern crate fontdue;
use crate::vertices::{BitmapVert, bmp_vert};
use crate::gl_wrap::{Program};
use fontdue::{Font, FontSettings};
use std::fs;

pub struct Bitmap {
    program: Program,
    vertices: [BitmapVert; 4],
    chars: Vec<char>
}

impl Bitmap {
    pub fn new() -> Result<Self, BitmapError> {
        let program = Program::new_from_files(
            "./shaders/bitmap_vert.glsl",
            "./shaders/bitmap_frag.glsl"
        )?;
        let vertices = bmp_vert![
            [0.5, 0.5, 1.0, 1.0],
            [0.5, -0.5, 1.0, 0.0],
            [-0.5, -0.5, 0.0, 0.0],
            [-0.5, 0.5, 0.0, 1.0]
        ];
        let chars: Vec<char> = CHAR_SET.chars().collect();
        Ok(
            Self { program, vertices, chars }
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
        for &c in &self.chars {
            let (metrics, bitmap) = font.rasterize(c, FONT_SIZE);
        }
        Ok(())
    }
}

static CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
static FONT_SIZE: f32 = 20.0;

extern crate thiserror;
use thiserror::Error;
use crate::gl_wrap::{ProgramError};

#[derive(Error, Debug)]
pub enum BitmapError {
    #[error("{0}")]
    ProgramError(#[from] ProgramError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    FontError(String)
}
