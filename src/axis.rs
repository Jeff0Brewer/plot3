extern crate gl;
extern crate glam;
extern crate alloc;
use glam::{Mat4, Vec4};
use crate::gl_wrap::{Program, Buffer, VertexArray};
use crate::scene::{Scene, DrawPass};
use std::ffi::CString;

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
const DEFAULT_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 1.0);

pub struct Axis {
    pub scene: Scene
}

impl Axis {
    pub fn new(mvp: &Mat4) -> Result<Self, AxisError> {
        let line_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let line_buffer = Buffer::new();
        line_buffer.set_data(&DEFAULT_AXIS, gl::STATIC_DRAW);
        let line_attrib = VertexArray::new();
        let pos_index = line_program.get_attrib_location("position")?;
        line_attrib.set_attribute::<PosVert>(pos_index, 3, 0);

        let programs = vec![line_program];
        let buffers = vec![line_buffer];
        let attribs = vec![line_attrib];
        let draws = vec![DrawPass::new(gl::LINES, 0, 0, 0, 0, 6)];
        let scene = Scene::new(draws, programs, buffers, attribs);

        let mvp_cname = CString::new("mvp")?;
        let color_cname = CString::new("color")?;
        for program in &scene.programs {
            program.apply();
            unsafe {
                let mvp_loc = gl::GetUniformLocation(program.id, mvp_cname.as_ptr());
                gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, &mvp.to_cols_array()[0]);
                let color_loc = gl::GetUniformLocation(program.id, color_cname.as_ptr());
                gl::Uniform4fv(color_loc, 1, &DEFAULT_COLOR[0]);
            }
        }

        Ok(Self { scene })
    }
}

extern crate thiserror;
use thiserror::Error;
use crate::gl_wrap::ShaderError;
use std::ffi::NulError;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    ShaderError(#[from] ShaderError),
    #[error("{0}")]
    NulError(#[from] NulError)
}
