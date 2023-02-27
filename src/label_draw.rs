extern crate gl;
use crate::gl_wrap::{Program, VertexArray, Buffer, Texture, Drop};
use crate::font_map::{FontMap, VERT_PER_CHAR};
use crate::vertices::BitmapVert;
use crate::scene::{Scene, DrawPass};
use std::collections::HashMap;

pub struct LabelDrawer {
    fontmap: FontMap,
    font_verts: Vec<BitmapVert>,
    font_inds: HashMap<char, usize>,
    font_texture: Texture,
    params: LabelParams
}

struct LabelParams {
    pub kearning: f32,
    pub font_size: f32
}

impl LabelDrawer {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, LabelError> {
        let fontmap = FontMap::new(window_width, window_height)?;
        let (texture, vertices, indices) = fontmap.gen_font_map(DEFAULT_FONT)?;
        let font_texture = texture;
        let font_verts = vertices;
        let font_inds = indices;
        let params = LabelParams::new_default();
        Ok(Self { fontmap, font_texture, font_verts, font_inds, params })
    }

    pub fn set_font_face(&mut self, font_file: &str) -> Result<(), LabelError> {
        self.font_texture.drop(); // free old font texture
        let (texture, vertices, indices) = self.fontmap.gen_font_map(font_file)?;
        self.font_texture = texture;
        self.font_verts = vertices;
        self.font_inds = indices;
        Ok(())
    }

    pub fn get_label_scene(&self, label: &str) -> Result<Scene, LabelError> {
        // create gl resources
        let program = Program::new_from_files(
            "./shaders/label_vert.glsl",
            "./shaders/label_frag.glsl"
        )?;
        let vao = VertexArray::new();
        let vertices = self.get_vertex_data(label)?;
        let buffer = Buffer::new_from(&vertices, gl::STATIC_DRAW);
        let pos_loc = program.get_attrib_location("position")?;
        let tcoord_loc = program.get_attrib_location("a_texCoord")?;
        vao.set_attribute::<BitmapVert>(pos_loc, 2, 0);
        vao.set_attribute::<BitmapVert>(tcoord_loc, 2, 2);

        // create scene
        let programs = vec![program];
        let vaos = vec![vao];
        let buffers = vec![buffer];
        let textures = vec![self.font_texture];
        let draw_passes = vec![
            DrawPass::new( gl::TRIANGLES, 0, 0, Some(0), vec![], vec![], 0, vertices.len() as i32)
        ];
        Ok(Scene::new(draw_passes, programs, vaos, buffers, textures, vec![], vec![]))
    }

    // create buffer data for string of characters from copies of font map vertex data
    fn get_vertex_data(&self, label: &str) -> Result<Vec<BitmapVert>, LabelError> {
        let mut vertices = Vec::<BitmapVert>::new();
        let mut offset: f32 = 0.0;
        for c in label.chars() {
            if let ' ' = c {
                // add fixed width for space character
                offset += self.params.font_size;
                continue;
            }
            // get start index of vertex data if char exists in font texture
            let vert_ind: usize;
            match self.font_inds.get(&c) {
                Some(&index) => { vert_ind = index; },
                None => { return Err(LabelError::CharacterError(c)); }
            }
            // character width / 2 from first vertex x coordinate
            let char_spacing = self.font_verts[vert_ind].position[0] + self.params.kearning;
            offset += char_spacing;
            for i in 0..VERT_PER_CHAR {
                let mut vert = self.font_verts[i + vert_ind].clone();
                vert.position[0] += offset; // offset vertex in x
                vertices.push(vert);
            }
            offset += char_spacing;
        }
        Ok(vertices)
    }
}

impl LabelParams {
    pub fn new_default() -> Self {
        LabelParams {
            kearning: 0.0,
            font_size: 20.0
        }
    }
}

static DEFAULT_FONT: &str = "./resources/Ubuntu-Regular.ttf";

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
    #[error("Invalid character '{0}'")]
    CharacterError(char)
}
