extern crate alloc;
extern crate gl;
extern crate glam;
use crate::gl_wrap::{Buffer, Program, UniformMat, UniformVec, VertexArray};
use crate::plot::Bounds;
use crate::scene::{DrawInds, DrawPass, Scene};
use crate::text::{FontMap, TextParams};
use crate::vertices::{pos_vert, PosVert, TextVert};

pub struct Ticks {
    style: TickStyle,
    color: [f32; 4],
    count: i32,
}

impl Ticks {
    pub fn new() -> Self {
        Self {
            style: TickStyle::Blank,
            color: [0.5, 0.5, 0.5, 1.0],
            count: 10,
        }
    }

    pub fn set_style(&mut self, style: TickStyle) {
        self.style = style;
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = [color[0], color[1], color[2], 1.0];
    }

    pub fn set_count(&mut self, count: i32) {
        self.count = count;
    }

    pub fn get_scene(&self, mvp: [f32; 16], bounds: &Bounds) -> Result<Scene, TicksError> {
        const VERT: &str = "./shaders/solid_vert.glsl";
        const FRAG: &str = "./shaders/solid_frag.glsl";
        let program = Program::new_from_files(VERT, FRAG)?;
        let vao = VertexArray::new();
        let verts = self.get_verts(bounds);
        let buffer = Buffer::new_from(&verts, gl::STATIC_DRAW);
        let pos_loc = program.get_attrib_location("position")?;
        vao.set_attribute::<PosVert>(pos_loc, 3, 0);
        let u_mvp = UniformMat::new(&program, "mvp", vec![mvp])?;
        let u_color = UniformVec::new(&program, "color", vec![self.color])?;

        let scene = Scene {
            programs: vec![program],
            vaos: vec![vao],
            buffers: vec![buffer],
            textures: vec![],
            matrices: vec![u_mvp],
            vectors: vec![u_color],
            passes: vec![DrawPass {
                draw_type: gl::LINES,
                start: 0,
                count: verts.len() as i32,
                inds: DrawInds {
                    program: 0,
                    vao: 0,
                    texture: None,
                    matrix: vec![[0, 0]],
                    vector: vec![[0, 0]],
                },
            }],
        };
        Ok(scene)
    }

    fn get_verts(&self, bounds: &Bounds) -> Vec<PosVert> {
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

pub enum TickStyle {
    Tick,
    Grid,
    Blank,
}

extern crate thiserror;
use crate::gl_wrap::{ProgramError, ShaderError, UniformError};
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
}
