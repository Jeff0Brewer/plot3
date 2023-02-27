extern crate gl;
extern crate glam;
use glam::{Mat4, Vec3};
use crate::gl_wrap::Window;
use crate::axis::Axis;
use crate::label_draw::LabelDrawer;
use crate::scene::Scene;

// values for default camera initialization
const DEFAULT_EYE: Vec3 = Vec3::new(1.5, 1.5, 1.5);
const DEFAULT_FOV: f32 = 70.0 * 3.14 / 180.0;
const CAMERA_NEAR: f32 = 0.0;
const CAMERA_FAR: f32 = 10.0;

pub struct Plot {
    window: Window,
    scene: Scene,
    mvp: [f32; 16],
    pub axis: Axis,
    pub labels: LabelDrawer
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
        let mut labels = LabelDrawer::new(width as i32, height as i32)?;
        labels.set_font("./resources/Ubuntu-Regular.ttf")?;

        unsafe { gl::ClearColor(0.1, 0.1, 0.1, 1.0); }

        Ok(Self { window, scene, mvp, axis, labels })
    }

    pub fn display(self) -> Result<(), PlotError> {
        let axis_scene = self.axis.get_scene(self.mvp.clone())?;
        let label_scene = self.labels.get_label_scene("Glady 0123")?;
        self.window.run(vec![axis_scene, label_scene, self.scene]);
        Ok(())
    }
}

extern crate thiserror;
use thiserror::Error;
extern crate glutin;
use glutin::{CreationError};
use crate::gl_wrap::ShaderError;
use crate::axis::AxisError;
use crate::label_draw::LabelError;
#[derive(Error, Debug)]
pub enum PlotError {
    #[error("{0}")]
    CreationError(#[from] CreationError),
    #[error("{0}")]
    ShaderError(#[from] ShaderError),
    #[error("{0}")]
    AxisError(#[from] AxisError),
    #[error("{0}")]
    LabelError(#[from] LabelError)
}
