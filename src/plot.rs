extern crate gl;
use crate::gl_wrap::Window;
use crate::axis::Axis;
use crate::scene::Scene;

pub struct Plot {
    window: Window,
    scene: Scene,
    pub axis: Axis
}

impl Plot {
    pub fn new(title: &str) -> Result<Self, PlotError> {
        let window = Window::new(title)?;
        let scene = Scene::new_empty();
        let axis = Axis::new()?;
        unsafe { gl::ClearColor(0.1, 0.1, 0.1, 1.0); }
        Ok(Self { window, scene, axis })
    }

    pub fn display(self) {
        let axis_scene = self.axis.get_scene();
        self.window.run(vec![axis_scene, self.scene]);
    }
}

extern crate thiserror;
use thiserror::Error;
extern crate glutin;
use glutin::{CreationError};
use crate::gl_wrap::ShaderError;
use crate::axis::AxisError;
#[derive(Error, Debug)]
pub enum PlotError {
    #[error("{0}")]
    CreationError(#[from] CreationError),
    #[error("{0}")]
    ShaderError(#[from] ShaderError),
    #[error("{0}")]
    AxisError(#[from] AxisError)
}
