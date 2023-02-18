extern crate gl;
extern crate glam;
extern crate alloc;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::scene::{Scene, DrawPass};
use crate::plot::{Bounds};

pub struct Axis {
    bounds: Bounds,
    border_style: BorderStyle,
    border_color: [f32; 4]
}

impl Axis {
    pub fn new() -> Self {
        let bounds = Bounds::new(1.0, 1.0, 1.0);
        let border_style = BorderStyle::Box;
        let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        Self { bounds, border_style, border_color }
    }

    pub fn get_scene(&self, mvp: [f32; 16]) -> Result<Scene, AxisError> {
        // get axis geometry from current fields
        let lines: Vec<PosVert>;
        let tris: Vec<PosVert>;
        match self.border_style {
            BorderStyle::Arrow => { (lines, tris) = get_arrow_axis(&self.bounds); },
            BorderStyle::Box => { (lines, tris) = get_box_axis(&self.bounds); }
        }

        // init gl resources
        let solid_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let mvp_uniform = UniformMatrix::new("mvp", mvp, vec![solid_program.id])?;
        let color_uniform = UniformVector::new("color", self.border_color, vec![solid_program.id])?;

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
        let matrices = vec![mvp_uniform];
        let vectors = vec![color_uniform];
        let draw_passes = vec![
            DrawPass::new(gl::LINES, 0, 0, vec![0], vec![0], 0, lines.len() as i32),
            DrawPass::new(gl::TRIANGLES, 0, 1, vec![0], vec![0], 0, tris.len() as i32)
        ];
        let scene = Scene::new(draw_passes, programs, vaos, buffers, matrices, vectors);
        Ok(scene)
    }

    pub fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    pub fn set_bounds(&mut self, x: f32, y: f32, z: f32) {
        self.bounds = Bounds::new(x, y, z);
    }

    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
    }

}

pub enum BorderStyle {
    Arrow,
    Box
}

// sized position vertex with C repr for gl buffering
type Pos = [f32; 3];
#[repr(C, packed)]
struct PosVert(Pos);
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert($pos),)*]
    }
}

fn get_arrow_axis(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    const S: f32 = 0.02; // arrow size
    let lines = pos_vert![
        [0.0, 0.0, 0.0],
        [b.x, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, b.y, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, b.z]
    ];
    let tris = pos_vert![
        [b.x, 0.0, 0.0],
        [b.x-S, S, 0.0],
        [b.x-S, -S, 0.0],
        [0.0, b.y, 0.0],
        [S, b.y-S, 0.0],
        [-S, b.y-S, 0.0],
        [0.0, 0.0, b.z],
        [S, 0.0, b.z-S],
        [-S, 0.0, b.z-S]
    ];
    (lines, tris)
}

fn get_box_axis(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    let lines = pos_vert![
        [b.x, b.y, 0.0],
        [b.x, 0.0, 0.0],
        [b.x, 0.0, 0.0],
        [b.x, 0.0, b.z],
        [b.x, 0.0, b.z],
        [0.0, 0.0, b.z],
        [0.0, 0.0, b.z],
        [0.0, b.y, b.z],
        [0.0, b.y, b.z],
        [0.0, b.y, 0.0],
        [0.0, b.y, 0.0],
        [b.x, b.y, 0.0]
    ];
    let tris = pos_vert![];
    (lines, tris)
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
