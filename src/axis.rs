extern crate alloc;
extern crate gl;
extern crate glam;
use crate::axis_vert::*;
use crate::gl_wrap::{Buffer, Program, UniformMat, UniformVec, VertexArray};
use crate::plot::Bounds;
use crate::scene::{DrawPass, Scene};
use crate::vertices::PosVert;

pub struct Axis {
    border_color: [f32; 4],
    tick_style: TickStyle,
    tick_color: [f32; 4],
    tick_count: i32,
}

impl Axis {
    pub fn new() -> Self {
        let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let tick_style = TickStyle::Blank;
        let tick_color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let tick_count: i32 = 10;
        Self {
            border_color,
            tick_style,
            tick_color,
            tick_count,
        }
    }

    pub fn get_scene(&self, mvp: [f32; 16], bounds: &Bounds) -> Result<Scene, AxisError> {
        // get axis geometry from current fields
        let (mut axis, mut ticks) = get_axis_verts(bounds, &self.tick_style, self.tick_count);
        let border_len = axis.len() as i32;
        let ticks_len = ticks.len() as i32;
        axis.append(&mut ticks);

        // init gl resources
        let solid_program =
            Program::new_from_files("./shaders/solid_vert.glsl", "./shaders/solid_frag.glsl")?;
        let u_mvp = UniformMat::new(&solid_program, "mvp", vec![mvp])?;
        let u_color = UniformVec::new(
            &solid_program,
            "color",
            vec![self.border_color, self.tick_color],
        )?;

        // setup vaos with data and attribs
        let pos_loc = solid_program.get_attrib_location("position")?;
        let vao = VertexArray::new();
        let buffer = Buffer::new_from(&axis, gl::STATIC_DRAW);
        vao.set_attribute::<PosVert>(pos_loc, 3, 0);

        // create scene
        let programs = vec![solid_program];
        let vaos = vec![vao];
        let buffers = vec![buffer];
        let textures = vec![];
        let matrices = vec![u_mvp];
        let vectors = vec![u_color];
        let draw_passes = vec![
            DrawPass::new(
                gl::LINES,
                0,
                0,
                None,
                vec![[0, 0]],
                vec![[0, 1]],
                border_len,
                ticks_len,
            ),
            DrawPass::new(
                gl::LINES,
                0,
                0,
                None,
                vec![[0, 0]],
                vec![[0, 0]],
                0,
                border_len,
            ),
        ];
        let scene = Scene::new(
            draw_passes,
            programs,
            vaos,
            buffers,
            textures,
            matrices,
            vectors,
        );
        Ok(scene)
    }

    pub fn set_tick_style(&mut self, style: TickStyle) {
        self.tick_style = style;
    }

    pub fn set_border_color(&mut self, color: [f32; 3]) {
        self.border_color = [color[0], color[1], color[2], 1.0];
    }

    pub fn set_tick_color(&mut self, color: [f32; 3]) {
        self.tick_color = [color[0], color[1], color[2], 1.0];
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
use std::ffi::NulError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    Shader(#[from] ShaderError),
    #[error("{0}")]
    Program(#[from] ProgramError),
    #[error("{0}")]
    Uniform(#[from] UniformError),
    #[error("{0}")]
    Nul(#[from] NulError),
}
