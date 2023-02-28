extern crate gl;
extern crate glam;
use glam::{Mat4, Vec3};
use crate::gl_wrap::Window;
use crate::axis::Axis;
use crate::label_draw::LabelDrawer;
use crate::scene::Scene;

pub struct Plot {
    window: Window,
    scene: Scene,
    mvp: [f32; 16],
    bg_color: [f32; 3],
    bounds: Bounds,
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
        let bg_color = DEFAULT_BG;
        let bounds = Bounds::new(1.0, 1.0, 1.0);
        let axis = Axis::new();
        let labels = LabelDrawer::new(width as i32, height as i32)?;

        Ok(Self { window, scene, mvp, bg_color, bounds, axis, labels })
    }

    pub fn display(self) -> Result<(), PlotError> {
        let axis_scene = self.axis.get_scene(self.mvp.clone(), &self.bounds)?;
        let label_scene = self.labels.get_scene(self.mvp.clone(), &self.bounds)?;
        unsafe {
            gl::ClearColor(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
        }
        self.window.run(vec![axis_scene, label_scene, self.scene]);
        Ok(())
    }

    pub fn set_background_color(&mut self, color: [f32; 3]) {
        self.bg_color = color;
    }

    pub fn set_bounds(&mut self, x: f32, y: f32, z: f32) {
        self.bounds.x = x;
        self.bounds.y = y;
        self.bounds.z = z;
    }
}

pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Bounds {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn max(&self) -> f32 {
        self.x.max(self.y).max(self.x)
    }
}

pub static DEFAULT_EYE: Vec3 = Vec3::new(1.5, 1.5, 1.5);
static DEFAULT_FOV: f32 = 70.0 * 3.14 / 180.0;
static CAMERA_NEAR: f32 = 0.0;
static CAMERA_FAR: f32 = 10.0;
static DEFAULT_BG: [f32; 3] = [0.1, 0.1, 0.1];

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
