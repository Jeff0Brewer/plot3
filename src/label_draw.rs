extern crate gl;
use crate::gl_wrap::{Program, VertexArray, Buffer, Texture};
use crate::font_map::{FontMap, VERT_PER_CHAR};
use crate::vertices::BitmapVert;
use crate::scene::{Scene, DrawPass};
use std::collections::HashMap;

pub struct LabelDrawer {
    fontmap: FontMap,
    font_verts: Vec<BitmapVert>,
    font_inds: HashMap<char, usize>,
    font_texture: Texture
}

impl LabelDrawer {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, LabelError> {
        let fontmap = FontMap::new(window_width, window_height)?;
        let font_verts = Vec::<BitmapVert>::new();
        let font_inds = HashMap::new();
        let font_texture = Texture::new_blank(1, 1);
        Ok(Self { fontmap, font_texture, font_verts, font_inds })
    }

    pub fn set_font(&mut self, font_file: &str) -> Result<(), LabelError> {
        let (texture, vertices, indices) = self.fontmap.gen_font_map(font_file)?;
        self.font_texture = texture;
        self.font_verts = vertices;
        self.font_inds = indices;
        Ok(())
    }

    pub fn get_label_scene(&self, label: &str) -> Result<Scene, LabelError> {
        if self.font_verts.len() == 0 {
            // error if drawing requested before font fontmap generation
            return Err(LabelError::InvalidFontDataError);
        }
        // create buffer data from label chars and fontmap data
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
            match self.font_inds.get(&c) {
                Some(&index) => { vert_ind = index; },
                None => { return Err(LabelError::CharacterError(c)) }
            }
            // character width taken from first vertex x coordinate
            let char_spacing = kearning + self.font_verts[vert_ind].0[0];
            offset += char_spacing;
            for i in 0..VERT_PER_CHAR {
                let mut vert = self.font_verts[i + vert_ind].clone();
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
use crate::font_map::FontMapError;
use crate::gl_wrap::ProgramError;

#[derive(Error, Debug)]
pub enum LabelError {
    #[error("{0}")]
    FontMapError(#[from] FontMapError),
    #[error("{0}")]
    ProgramError(#[from] ProgramError),
    #[error("No font data available, must set font before drawing labels")]
    InvalidFontDataError,
    #[error("Invalid character '{0}'")]
    CharacterError(char)
}
