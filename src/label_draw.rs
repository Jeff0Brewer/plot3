extern crate gl;
use crate::gl_wrap::Texture;
use crate::bitmap::{Bitmap, VERT_PER_CHAR};
use crate::vertices::BitmapVert;
use std::collections::HashMap;
use gl::types::GLuint;

pub struct LabelDrawer {
    bitmap: Bitmap,
    font_texture: Texture,
    bitmap_verts: Vec<BitmapVert>,
    bitmap_inds: HashMap<char, usize>
}

impl LabelDrawer {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, LabelError> {
        let bitmap = Bitmap::new(window_width, window_height)?;
        let bitmap_verts = Vec::<BitmapVert>::new();
        let bitmap_inds = HashMap::new();
        let font_texture = Texture::new(&[], 0, 0);
        Ok(Self { bitmap, font_texture, bitmap_verts, bitmap_inds })
    }

    pub fn set_font(&mut self, font_file: &str) -> Result<(), LabelError> {
        let (texture, vertices, indices) = self.bitmap.gen_font_map(font_file)?;
        self.font_texture = texture;
        self.bitmap_verts = vertices;
        self.bitmap_inds = indices;
        Ok(())
    }

    pub fn draw_label(&self, label: &str) -> Result<(), LabelError> {
        if self.bitmap_verts.len() == 0 {
            // error if drawing requested before font bitmap generation
            return Err(LabelError::FontMapError);
        }
        let mut vertices = Vec::<BitmapVert>::new();
        for c in label.chars() {
            let ind: usize;
            match self.bitmap_inds.get(&c) {
                Some(&index) => { ind = index; },
                None => { return Err(LabelError::CharacterError(c)) }
            }

            let mut char_verts = self.bitmap_verts[ind..(ind + VERT_PER_CHAR)].to_vec();
            vertices.append(&mut char_verts);
        }
        Ok(())
    }
}

extern crate thiserror;
use thiserror::Error;
use crate::bitmap::BitmapError;

#[derive(Error, Debug)]
pub enum LabelError {
    #[error("{0}")]
    BitmapError(#[from] BitmapError),
    #[error("No font bitmap available")]
    FontMapError,
    #[error("Invalid character '{0}'")]
    CharacterError(char)
}
