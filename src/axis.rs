extern crate gl;
extern crate glam;
extern crate alloc;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::scene::{Scene, DrawPass};
use crate::plot::Bounds;
use crate::vertices::PosVert;
use crate::axis_vert::*;

pub struct Axis {
    border_style: BorderStyle,
    border_color: [f32; 4],
    tick_style: TickStyle,
    tick_color: [f32; 4],
    tick_count: i32
}

#[allow(dead_code)]
impl Axis {
    pub fn new() -> Self {
        let border_style = BorderStyle::Box;
        let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let tick_style = TickStyle::Grid;
        let tick_color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let tick_count: i32 = 10;
        Self { border_style, border_color, tick_style, tick_color, tick_count }
    }

    pub fn get_scene(&self, mvp: [f32; 16], bounds: &Bounds) -> Result<Scene, AxisError> {
        // get axis geometry from current fields
        let (mut lines, tris) = get_axis_border(&self.border_style, bounds);
        let border_line_len = lines.len();
        let mut tick_lines = get_axis_ticks(&self.tick_style, &self.border_style, &bounds, &self.tick_count);
        lines.append(&mut tick_lines);

        // init gl resources
        let solid_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let mvp_matrix = UniformMatrix::new("mvp", mvp, vec![solid_program.id])?;
        let border_color = UniformVector::new("color", self.border_color, vec![solid_program.id])?;
        let tick_color = UniformVector::new("color", self.tick_color, vec![solid_program.id])?;

        // setup vaos with data and attribs
        let pos_loc = solid_program.get_attrib_location("position")?;
        let line_vao = VertexArray::new();
        let line_buffer = Buffer::new_from(&lines, gl::STATIC_DRAW);
        line_vao.set_attribute::<PosVert>(pos_loc, 3, 0);
        let tri_vao = VertexArray::new();
        let tri_buffer = Buffer::new_from(&tris, gl::STATIC_DRAW);
        tri_vao.set_attribute::<PosVert>(pos_loc, 3, 0);

        // create scene
        let programs = vec![solid_program];
        let vaos = vec![line_vao, tri_vao];
        let buffers = vec![line_buffer, tri_buffer];
        let textures = vec![];
        let matrices = vec![mvp_matrix];
        let vectors = vec![border_color, tick_color];
        let draw_passes = vec![
            DrawPass::new(gl::LINES, 0, 0, None, vec![0], vec![1], border_line_len as i32, (lines.len() - border_line_len) as i32),
            DrawPass::new(gl::LINES, 0, 0, None, vec![0], vec![0], 0, border_line_len as i32),
            DrawPass::new(gl::TRIANGLES, 0, 1, None, vec![0], vec![0], 0, tris.len() as i32)
        ];
        let scene = Scene::new(draw_passes, programs, vaos, buffers, textures, matrices, vectors);
        Ok(scene)
    }

    pub fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    pub fn set_tick_style(&mut self, style: TickStyle) {
        self.tick_style = style;
    }

    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
    }

    pub fn set_tick_color(&mut self, color: [f32; 4]) {
        self.tick_color = color;
    }
}

#[allow(dead_code)]
pub enum BorderStyle {
    Arrow,
    Box
}

#[allow(dead_code)]
pub enum TickStyle {
    Tick,
    Grid,
    Blank
}

extern crate thiserror;
use thiserror::Error;
use crate::gl_wrap::{ShaderError, ProgramError, UniformError};
use std::ffi::NulError;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    ShaderError(#[from] ShaderError),
    #[error("{0}")]
    ProgramError(#[from] ProgramError),
    #[error("{0}")]
    UniformError(#[from] UniformError),
    #[error("{0}")]
    NulError(#[from] NulError)
}
