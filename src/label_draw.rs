extern crate gl;
extern crate glam;
use crate::font_map::{FontMap, VERT_PER_CHAR};
use crate::gl_wrap::{Buffer, Drop, Program, Texture, UniformMat, UniformVec, VertexArray};
use crate::plot::Bounds;
use crate::scene::{DrawPass, Scene};
use crate::vertices::{bmp_to_text_vert, BitmapVert, TextVert};
use std::collections::HashMap;

pub struct LabelDrawer {
    fontmap: FontMap,
    font_verts: Vec<BitmapVert>,
    font_inds: HashMap<char, usize>,
    font_texture: Texture,
    labels: AxisLabels,
    params: LabelParams,
}

struct AxisLabels {
    pub x: String,
    pub y: String,
    pub z: String,
}

struct LabelParams {
    pub kearning: f32,
    pub font_size: f32,
}

impl LabelDrawer {
    pub fn new(window_width: i32, window_height: i32) -> Result<Self, LabelError> {
        let fontmap = FontMap::new(window_width, window_height)?;
        let (texture, vertices, indices) = fontmap.gen_font_map(DEFAULT_FONT)?;
        let font_texture = texture;
        let font_verts = vertices;
        let font_inds = indices;
        let labels = AxisLabels::new();
        let params = LabelParams::new();
        Ok(Self {
            fontmap,
            font_texture,
            font_verts,
            font_inds,
            labels,
            params,
        })
    }

    pub fn set_labels(&mut self, x: &str, y: &str, z: &str) {
        self.labels.x = x.to_string();
        self.labels.y = y.to_string();
        self.labels.z = z.to_string();
    }

    pub fn set_font_face(&mut self, font_file: &str) -> Result<(), LabelError> {
        self.font_texture.drop(); // free old font texture
        let (texture, vertices, indices) = self.fontmap.gen_font_map(font_file)?;
        self.font_texture = texture;
        self.font_verts = vertices;
        self.font_inds = indices;
        Ok(())
    }

    fn get_label_verts(
        &self,
        label: &str,
        position: [f32; 3],
    ) -> Result<Vec<TextVert>, LabelError> {
        let mut vertices = Vec::<TextVert>::new();
        let mut offset: f32 = 0.0;
        for c in label.chars() {
            if let ' ' = c {
                // add fixed width for space character
                offset += self.params.font_size;
                continue;
            }
            // get start index of vertex data if char exists in font texture
            let vert_ind = match self.font_inds.get(&c) {
                Some(&index) => index,
                None => return Err(LabelError::Character(c)),
            };
            // character width / 2 from first vertex x coordinate
            let char_spacing = self.font_verts[vert_ind].position[0] + self.params.kearning;
            offset += char_spacing;
            for i in 0..VERT_PER_CHAR {
                let mut vert = bmp_to_text_vert!(self.font_verts[i + vert_ind], position);
                vert.offset[0] += offset; // layout text on x axis
                vertices.push(vert);
            }
            offset += char_spacing;
        }
        // center text about origin
        let mid_width = offset * 0.5;
        for vert in &mut vertices {
            vert.offset[0] -= mid_width;
        }

        Ok(vertices)
    }

    pub fn get_scene(&self, mvp: [f32; 16], bounds: &Bounds) -> Result<Scene, LabelError> {
        let mut vertices = Vec::<TextVert>::new();
        vertices.append(&mut self.get_label_verts(
            &self.labels.x,
            [bounds.x * 0.5, 0.0, bounds.z + LABEL_MARGIN],
        )?);
        let x_len = vertices.len() as i32;
        vertices.append(&mut self.get_label_verts(
            &self.labels.y,
            [bounds.x + LABEL_MARGIN, bounds.y * 0.5, 0.0],
        )?);
        let y_len = vertices.len() as i32;
        vertices.append(&mut self.get_label_verts(
            &self.labels.z,
            [bounds.x + LABEL_MARGIN, 0.0, bounds.z * 0.5],
        )?);
        let z_len = vertices.len() as i32;

        // create gl resources
        let program =
            Program::new_from_files("./shaders/text_align_vert.glsl", "./shaders/text_frag.glsl")?;
        let vao = VertexArray::new();
        let buffer = Buffer::new_from(&vertices, gl::STATIC_DRAW);
        let pos_loc = program.get_attrib_location("position")?;
        let offset_loc = program.get_attrib_location("offset")?;
        let tcoord_loc = program.get_attrib_location("a_texCoord")?;
        vao.set_attribute::<TextVert>(pos_loc, 3, 0);
        vao.set_attribute::<TextVert>(offset_loc, 2, 3);
        vao.set_attribute::<TextVert>(tcoord_loc, 2, 5);
        let u_mvp = UniformMat::new(&program, "mvp", vec![mvp])?;
        let u_alignment = UniformVec::new(
            &program,
            "alignment",
            vec![
                [1.0, 0.0, 0.0, 1.0],
                [0.0, -1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
            ],
        )?;

        // create scene
        let programs = vec![program];
        let vaos = vec![vao];
        let buffers = vec![buffer];
        let textures = vec![self.font_texture];
        let matrices = vec![u_mvp];
        let vectors = vec![u_alignment];
        let draw_passes = vec![
            DrawPass::new(
                gl::TRIANGLES,
                0,
                0,
                Some(0),
                vec![[0, 0]],
                vec![[0, 0]],
                0,
                x_len,
            ),
            DrawPass::new(
                gl::TRIANGLES,
                0,
                0,
                Some(0),
                vec![[0, 0]],
                vec![[0, 1]],
                x_len,
                y_len - x_len,
            ),
            DrawPass::new(
                gl::TRIANGLES,
                0,
                0,
                Some(0),
                vec![[0, 0]],
                vec![[0, 2]],
                y_len,
                z_len - y_len,
            ),
        ];
        Ok(Scene::new(
            draw_passes,
            programs,
            vaos,
            buffers,
            textures,
            matrices,
            vectors,
        ))
    }
}

impl AxisLabels {
    pub fn new() -> Self {
        Self {
            x: "".to_string(),
            y: "".to_string(),
            z: "".to_string(),
        }
    }
}

impl LabelParams {
    pub fn new() -> Self {
        Self {
            kearning: 0.0,
            font_size: 20.0,
        }
    }
}

static DEFAULT_FONT: &str = "./resources/Ubuntu-Regular.ttf";
static LABEL_MARGIN: f32 = 0.1;

extern crate thiserror;
use crate::font_map::FontMapError;
use crate::gl_wrap::{ProgramError, UniformError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LabelError {
    #[error("{0}")]
    FontMap(#[from] FontMapError),
    #[error("{0}")]
    Program(#[from] ProgramError),
    #[error("{0}")]
    Uniform(#[from] UniformError),
    #[error("Invalid character '{0}'")]
    Character(char),
}
