extern crate gl;
use crate::gl_wrap::{Program, VertexArray, Buffer, Texture};
use crate::bitmap::{Bitmap, VERT_PER_CHAR};
use crate::vertices::BitmapVert;
use crate::scene::{Scene, DrawPass};
use std::collections::HashMap;

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
        let font_texture = Texture::new_blank(1, 1);
        Ok(Self { bitmap, font_texture, bitmap_verts, bitmap_inds })
    }

    pub fn set_font(&mut self, font_file: &str) -> Result<(), LabelError> {
        let (texture, vertices, indices) = self.bitmap.gen_font_map(font_file)?;
        self.font_texture = texture;
        self.bitmap_verts = vertices;
        self.bitmap_inds = indices;
        Ok(())
    }

    pub fn get_label_scene(&self, label: &str) -> Result<Scene, LabelError> {
        if self.bitmap_verts.len() == 0 {
            // error if drawing requested before font bitmap generation
            return Err(LabelError::FontMapError);
        }
        // create buffer data from label chars and bitmap data
        let mut vertices = Vec::<BitmapVert>::new();
        let mut offset: f32 = 0.0;
        let kearning = 2.0;
        for c in label.chars() {
            if let ' ' = c {
                // fixed width for space character
                offset += 20.0;
                continue;
            }
            let vert_ind: usize;
            match self.bitmap_inds.get(&c) {
                Some(&index) => { vert_ind = index; },
                None => { return Err(LabelError::CharacterError(c)) }
            }
            // character width taken from first vertex x coordinate
            let char_spacing = kearning + self.bitmap_verts[vert_ind].0[0];
            offset += char_spacing;
            for i in 0..VERT_PER_CHAR {
                let mut vert = self.bitmap_verts[i + vert_ind].clone();
                vert.0[0] += offset;
                vertices.push(vert);
            }
            offset += char_spacing;
        }

        // create gl resources
        let program = Program::new_from_files(
            "./shaders/label_vert.glsl",
            "./shaders/label_frag.glsl"
        )?;
        let vao = VertexArray::new();
        let buffer = Buffer::new_from(&vertices, gl::STATIC_DRAW);
        let pos_loc = program.get_attrib_location("position")?;
        let tcoord_loc = program.get_attrib_location("a_texCoord")?;
        vao.set_attribute::<BitmapVert>(pos_loc, 2, 0);
        vao.set_attribute::<BitmapVert>(tcoord_loc, 2, 2);

        let programs = vec![program];
        let vaos = vec![vao];
        let buffers = vec![buffer];
        let textures = vec![self.font_texture];
        let draw_passes = vec![
            DrawPass::new( gl::TRIANGLES, 0, 0, Some(0), vec![], vec![], 0, vertices.len() as i32)
        ];
        Ok(Scene::new(draw_passes, programs, vaos, buffers, textures, vec![], vec![]))
    }
}

extern crate thiserror;
use thiserror::Error;
use crate::bitmap::BitmapError;
use crate::gl_wrap::ProgramError;

#[derive(Error, Debug)]
pub enum LabelError {
    #[error("{0}")]
    BitmapError(#[from] BitmapError),
    #[error("{0}")]
    ProgramError(#[from] ProgramError),
    #[error("No font bitmap available")]
    FontMapError,
    #[error("Invalid character '{0}'")]
    CharacterError(char)
}
