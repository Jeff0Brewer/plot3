extern crate gl;
extern crate glam;
use glam::{Mat4, Vec3};
use crate::gl_wrap::Window;
use crate::axis::Axis;
use crate::scene::Scene;

// values for default camera initialization
const DEFAULT_EYE: Vec3 = Vec3::new(1.25, 1.25, 1.25);
const DEFAULT_FOV: f32 = 70.0 * 3.14 / 180.0;
const CAMERA_NEAR: f32 = 0.0;
const CAMERA_FAR: f32 = 10.0;

pub struct Plot {
    window: Window,
    scene: Scene,
    mvp: [f32; 16],
    pub axis: Axis
}

impl Plot {
    pub fn new(title: &str, width: f64, height: f64) -> Result<Self, PlotError> {
        let window = Window::new(title, width, height)?;
        let scene = Scene::new_empty();

        let proj_matrix = Mat4::perspective_rh_gl(
            DEFAULT_FOV,
            (width / height) as f32,
            CAMERA_NEAR,
            CAMERA_FAR
        );
        let view_matrix = Mat4::look_at_rh(
            DEFAULT_EYE,
            Vec3::ZERO,
            Vec3::Y
        );
        let mvp = proj_matrix.mul_mat4(&view_matrix).to_cols_array();
        let axis = Axis::new();

        unsafe { gl::ClearColor(0.1, 0.1, 0.1, 1.0); }

        Ok(Self { window, scene, mvp, axis })
    }

    pub fn display(self) -> Result<(), PlotError> {
        let axis_scene = self.axis.get_scene(self.mvp.clone())?;
        self.window.run(vec![axis_scene, self.scene]);
        Ok(())
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
