extern crate gl;
use crate::gl_wrap::{Program, Buffer, VertexArray};
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

pub struct Axis {
    scene: Scene
}

impl Axis {
    pub fn new() -> Result<Self, AxisError> {
        let line_program = Program::new_from_files(
            "./shaders/vert.glsl",
            "./shaders/frag.glsl"
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
        Ok(Self { scene })
    }

    pub fn get_scene(self) -> Scene {
        self.scene
    }
}

extern crate thiserror;
use thiserror::Error;
extern crate glutin;
use crate::gl_wrap::ShaderError;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    ShaderError(#[from] ShaderError)
}
