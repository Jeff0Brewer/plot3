extern crate gl;
extern crate glam;
extern crate alloc;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::scene::{Scene, DrawPass};

pub enum BorderStyle {
    Arrow,
    Box
}

type Pos = [f32; 3];
#[repr(C, packed)]
struct PosVert(Pos);

const ARROW_SIZE: f32 = 0.02;
fn get_arrow_axis(bounds: [f32; 3]) -> ([PosVert; 6], [PosVert; 9]) {
    let lines = [
        PosVert([0.0, 0.0, 0.0]),
        PosVert([bounds[0], 0.0, 0.0]),
        PosVert([0.0, 0.0, 0.0]),
        PosVert([0.0, bounds[1], 0.0]),
        PosVert([0.0, 0.0, 0.0]),
        PosVert([0.0, 0.0, bounds[2]])
    ];
    let tris = [
        PosVert([bounds[0], 0.0, 0.0]),
        PosVert([bounds[0] - ARROW_SIZE, ARROW_SIZE, 0.0]),
        PosVert([bounds[0] - ARROW_SIZE, -ARROW_SIZE, 0.0]),
        PosVert([0.0, bounds[1], 0.0]),
        PosVert([ARROW_SIZE, bounds[1] - ARROW_SIZE, 0.0]),
        PosVert([-ARROW_SIZE, bounds[1] - ARROW_SIZE, 0.0]),
        PosVert([0.0, 0.0, bounds[2]]),
        PosVert([ARROW_SIZE, 0.0, bounds[2] - ARROW_SIZE]),
        PosVert([-ARROW_SIZE, 0.0, bounds[2] - ARROW_SIZE])
    ];
    (lines, tris)
}

fn get_box_axis(bounds: [f32; 3]) -> [PosVert; 12] {
    let lines = [
        PosVert([bounds[0], bounds[1], 0.0]),
        PosVert([bounds[0], 0.0, 0.0]),
        PosVert([bounds[0], 0.0, 0.0]),
        PosVert([bounds[0], 0.0, bounds[2]]),
        PosVert([bounds[0], 0.0, bounds[2]]),
        PosVert([0.0, 0.0, bounds[2]]),
        PosVert([0.0, 0.0, bounds[2]]),
        PosVert([0.0, bounds[1], bounds[2]]),
        PosVert([0.0, bounds[1], bounds[2]]),
        PosVert([0.0, bounds[1], 0.0]),
        PosVert([0.0, bounds[1], 0.0]),
        PosVert([bounds[0], bounds[1], 0.0]),
    ];
    lines
}

pub struct Axis {
    bounds: [f32; 3],
    border_style: BorderStyle,
    border_color: [f32; 4]
}

impl Axis {
    pub fn new() -> Self {
        let bounds = [1.0, 1.0, 1.0];
        let border_style = BorderStyle::Box;
        let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        Self { bounds, border_style, border_color }
    }

    pub fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    pub fn set_bounds(&mut self, bounds: [f32; 3]) {
        self.bounds = bounds;
    }

    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
    }

    pub fn get_scene(&self, mvp: [f32; 16]) -> Result<Scene, AxisError> {
        let solid_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let line_buffer = Buffer::new();
        let tri_buffer = Buffer::new();
        let line_len: i32;
        let tri_len: i32;
        match self.border_style {
            BorderStyle::Arrow => {
                let (lines, tris) = get_arrow_axis(self.bounds);
                line_buffer.set_data(&lines, gl::STATIC_DRAW);
                line_len = lines.len() as i32;
                tri_buffer.set_data(&tris, gl::STATIC_DRAW);
                tri_len = tris.len() as i32;
            },
            BorderStyle::Box => {
                let lines = get_box_axis(self.bounds);
                line_buffer.set_data(&lines, gl::STATIC_DRAW);
                line_len = lines.len() as i32;
                tri_len = 0;
            }
        }
        let pos_loc = solid_program.get_attrib_location("position")?;
        let line_attrib = VertexArray::new();
        line_buffer.bind();
        line_attrib.set_attribute::<PosVert>(pos_loc, 3, 0);
        let tri_attrib = VertexArray::new();
        tri_buffer.bind();
        tri_attrib.set_attribute::<PosVert>(pos_loc, 3, 0);

        let mvp_uniform = UniformMatrix::new("mvp", mvp, vec![solid_program.id])?;
        let color_uniform = UniformVector::new("color", self.border_color, vec![solid_program.id])?;

        let programs = vec![solid_program];
        let buffers = vec![line_buffer, tri_buffer];
        let attribs = vec![line_attrib, tri_attrib];
        let matrices = vec![mvp_uniform];
        let vectors = vec![color_uniform];
        let draw_passes = vec![
            DrawPass::new(gl::LINES, 0, 0, 0, vec![0], vec![0], 0, line_len),
            DrawPass::new(gl::TRIANGLES, 0, 1, 1, vec![0], vec![0], 0, tri_len)
        ];
        let scene = Scene::new(draw_passes, programs, buffers, attribs, matrices, vectors);
        Ok(scene)
    }
}

extern crate thiserror;
use thiserror::Error;
use crate::gl_wrap::{ShaderError, UniformError};
use std::ffi::NulError;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    ShaderError(#[from] ShaderError),
    #[error("{0}")]
    UniformError(#[from] UniformError),
    #[error("{0}")]
    NulError(#[from] NulError)
}
