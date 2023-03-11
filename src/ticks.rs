extern crate alloc;
extern crate gl;
extern crate glam;
use crate::gl_wrap::{Buffer, Program, UniformMat, UniformVec, VertexArray};
use crate::plot::Bounds;
use crate::scene::{DrawInds, DrawPass, Scene};
use crate::text::{FontMap, TextParams};
use crate::vertices::{pos_vert, PosVert, TextVert};

pub struct Ticks {
    pub style: TickStyle,
    pub color: [f32; 4],
    pub count: i32,
    pub text: TextParams,
    pub labels: TickLabels,
}

impl Ticks {
    pub fn new() -> Self {
        Self {
            style: TickStyle::Blank,
            color: [0.5, 0.5, 0.5, 1.0],
            count: 10,
            text: TextParams::new(),
            labels: TickLabels::new(),
        }
    }

    pub fn get_scene(
        &self,
        mvp: [f32; 16],
        bounds: &Bounds,
        font: &FontMap,
    ) -> Result<Scene, TicksError> {
        let line_verts = self.get_lines(bounds);
        let text_verts = self.get_text(bounds, font)?;

        const LINE_VERT: &str = "./shaders/solid_vert.glsl";
        const LINE_FRAG: &str = "./shaders/solid_frag.glsl";
        let line_program = Program::new_from_files(LINE_VERT, LINE_FRAG)?;
        let line_vao = VertexArray::new();
        let line_buffer = Buffer::new_from(&line_verts, gl::STATIC_DRAW);
        let line_pos_loc = line_program.get_attrib_location("position")?;
        line_vao.set_attribute::<PosVert>(line_pos_loc, 3, 0);
        let u_mvp_line = UniformMat::new(&line_program, "mvp", vec![mvp])?;
        let u_color = UniformVec::new(&line_program, "color", vec![self.color])?;

        const TEXT_VERT: &str = "./shaders/text_vert.glsl";
        const TEXT_FRAG: &str = "./shaders/text_frag.glsl";
        let text_program = Program::new_from_files(TEXT_VERT, TEXT_FRAG)?;
        let text_vao = VertexArray::new();
        let text_buffer = Buffer::new_from(&text_verts, gl::STATIC_DRAW);
        let text_pos_loc = text_program.get_attrib_location("position")?;
        let text_off_loc = text_program.get_attrib_location("offset")?;
        let text_tco_loc = text_program.get_attrib_location("a_texCoord")?;
        text_vao.set_attribute::<TextVert>(text_pos_loc, 3, 0);
        text_vao.set_attribute::<TextVert>(text_off_loc, 2, 3);
        text_vao.set_attribute::<TextVert>(text_tco_loc, 2, 5);
        let u_mvp_text = UniformMat::new(&text_program, "mvp", vec![mvp])?;

        let scene = Scene {
            programs: vec![line_program, text_program],
            vaos: vec![line_vao, text_vao],
            buffers: vec![line_buffer, text_buffer],
            textures: vec![font.texture],
            matrices: vec![u_mvp_line, u_mvp_text],
            vectors: vec![u_color],
            passes: vec![
                DrawPass {
                    draw_type: gl::TRIANGLES,
                    start: 0,
                    count: text_verts.len() as i32,
                    inds: DrawInds {
                        program: 1,
                        vao: 1,
                        texture: Some(0),
                        matrix: vec![[1, 0]],
                        vector: vec![],
                    },
                },
                DrawPass {
                    draw_type: gl::LINES,
                    start: 0,
                    count: line_verts.len() as i32,
                    inds: DrawInds {
                        program: 0,
                        vao: 0,
                        texture: None,
                        matrix: vec![[0, 0]],
                        vector: vec![[0, 0]],
                    },
                },
            ],
        };
        Ok(scene)
    }

    fn get_text(&self, bounds: &Bounds, font: &FontMap) -> Result<Vec<TextVert>, TicksError> {
        let mut verts = Vec::<TextVert>::new();
        let spacing = bounds.max() / (self.count as f32);
        if self.labels.x {
            for i in 0..((bounds.x / spacing) as i32) {
                let x = spacing * (i as f32);
                verts.append(&mut font.get_verts(
                    &format!("{:.2}", x),
                    &self.text,
                    [x, 0.0, bounds.z],
                )?);
            }
        }
        if self.labels.y {
            for i in 0..((bounds.y / spacing) as i32) {
                let y = spacing * (i as f32);
                verts.append(&mut font.get_verts(
                    &format!("{:.2}", y),
                    &self.text,
                    [bounds.x, y, 0.0],
                )?);
            }
        }
        if self.labels.z {
            for i in 0..((bounds.z / spacing) as i32) {
                let z = spacing * (i as f32);
                verts.append(&mut font.get_verts(
                    &format!("{:.2}", z),
                    &self.text,
                    [bounds.x, 0.0, z],
                )?);
            }
        }
        Ok(verts)
    }

    fn get_lines(&self, bounds: &Bounds) -> Vec<PosVert> {
        // init blank bg
        let mut verts = pos_vert![
            [0.0, 0.0, 0.0],
            [bounds.x, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, bounds.y, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, bounds.z]
        ];
        let spacing = bounds.max() / (self.count as f32);
        verts.append(&mut match self.style {
            TickStyle::Grid => Ticks::get_grid_verts(bounds, spacing),
            TickStyle::Tick => Ticks::get_tick_verts(bounds, spacing),
            TickStyle::Blank => vec![],
        });
        verts
    }

    fn get_tick_verts(b: &Bounds, spacing: f32) -> Vec<PosVert> {
        const S: f32 = 0.02; // tick size
        let mut verts = Vec::<PosVert>::new();
        for i in 0..((b.x / spacing) as i32) {
            let x = spacing * (i as f32);
            verts.append(&mut pos_vert![[x, 0.0, b.z], [x, 0.0, b.z + S]]);
        }
        for i in 0..((b.y / spacing) as i32) {
            let y = spacing * (i as f32);
            verts.append(&mut pos_vert![[b.x, y, 0.0], [b.x + S, y, 0.0]]);
        }
        for i in 0..((b.z / spacing) as i32) {
            let z = spacing * (i as f32);
            verts.append(&mut pos_vert![[b.x, 0.0, z], [b.x + S, 0.0, z]]);
        }
        verts
    }

    fn get_grid_verts(b: &Bounds, spacing: f32) -> Vec<PosVert> {
        let mut verts = Vec::<PosVert>::new();
        for i in 0..((b.x / spacing) as i32) {
            let x = spacing * (i as f32);
            verts.append(&mut pos_vert![
                [x, 0.0, 0.0],
                [x, b.y, 0.0],
                [x, 0.0, 0.0],
                [x, 0.0, b.z]
            ]);
        }
        for i in 0..((b.y / spacing) as i32) {
            let y = spacing * (i as f32);
            verts.append(&mut pos_vert![
                [0.0, y, 0.0],
                [b.x, y, 0.0],
                [0.0, y, 0.0],
                [0.0, y, b.z]
            ]);
        }
        for i in 0..((b.z / spacing) as i32) {
            let z = spacing * (i as f32);
            verts.append(&mut pos_vert![
                [0.0, 0.0, z],
                [b.x, 0.0, z],
                [0.0, 0.0, z],
                [0.0, b.y, z]
            ]);
        }
        verts
    }
}

pub struct TickLabels {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl TickLabels {
    pub fn new() -> Self {
        Self {
            x: true,
            y: true,
            z: true,
        }
    }
}

#[allow(dead_code)]
pub enum TickStyle {
    Tick,
    Grid,
    Blank,
}

extern crate thiserror;
use crate::gl_wrap::{ProgramError, ShaderError, UniformError};
use crate::text::FontMapError;
use std::ffi::NulError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum TicksError {
    #[error("{0}")]
    Shader(#[from] ShaderError),
    #[error("{0}")]
    Program(#[from] ProgramError),
    #[error("{0}")]
    Uniform(#[from] UniformError),
    #[error("{0}")]
    Nul(#[from] NulError),
    #[error("{0}")]
    FontMap(#[from] FontMapError),
}
