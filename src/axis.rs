extern crate gl;
extern crate glam;
extern crate alloc;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::scene::{Scene, DrawPass};

type Pos = [f32; 3];
#[repr(C, packed)]
struct PosVert(Pos);
const DEFAULT_AXIS: [PosVert; 6] = [
    PosVert([0.0, 0.0, 0.0]),
    PosVert([1.0, 0.0, 0.0]),
    PosVert([0.0, 0.0, 0.0]),
    PosVert([0.0, 1.0, 0.0]),
    PosVert([0.0, 0.0, 0.0]),
    PosVert([0.0, 0.0, 1.0])
];
const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Axis<'a> {
    border: &'a [PosVert]
}

impl<'a> Axis<'a> {
    pub fn new() -> Self {
        let border = &DEFAULT_AXIS;
        Self { border }
    }

    pub fn get_scene(&self, mvp: [f32; 16]) -> Result<Scene, AxisError> {
        let line_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let line_buffer = Buffer::new();
        line_buffer.set_data(&self.border, gl::STATIC_DRAW);
        let line_attrib = VertexArray::new();
        let pos_loc = line_program.get_attrib_location("position")?;
        line_attrib.set_attribute::<PosVert>(pos_loc, 3, 0);

        let mut mvp_uniform = UniformMatrix::new("mvp", mvp)?;
        let mut color_uniform = UniformVector::new("color", DEFAULT_COLOR.clone())?;
        line_program.apply();
        mvp_uniform.apply(&line_program)?;
        color_uniform.apply(&line_program)?;

        let programs = vec![line_program];
        let buffers = vec![line_buffer];
        let attribs = vec![line_attrib];
        let draw_passes = vec![DrawPass::new(gl::LINES, 0, 0, 0, 0, 6)];
        let scene = Scene::new(draw_passes, programs, buffers, attribs);
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
