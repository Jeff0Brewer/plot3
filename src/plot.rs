extern crate gl;
extern crate glam;
use crate::axis::Axis;
use crate::gl_wrap::Window;
use crate::text::FontMapper;
use crate::ticks::Ticks;
use glam::{Mat4, Vec3};

pub struct Plot {
    window: Window,
    mvp: [f32; 16],
    bg_color: [f32; 3],
    bounds: Bounds,
    font_mapper: FontMapper,
    pub axis: Axis,
    pub ticks: Ticks,
}

impl Plot {
    pub fn new(title: &str, width: f64, height: f64) -> Result<Self, PlotError> {
        let proj_matrix = Mat4::perspective_rh_gl(
            DEFAULT_FOV,
            (width / height) as f32,
            CAMERA_NEAR,
            CAMERA_FAR,
        );
        let view_matrix = Mat4::look_at_rh(DEFAULT_EYE, Vec3::ZERO, Vec3::Y);
        let mvp = proj_matrix.mul_mat4(&view_matrix).to_cols_array();
        Ok(Self {
            window: Window::new(title, width, height)?,
            mvp,
            bg_color: DEFAULT_BG,
            bounds: Bounds::new(1.0, 1.0, 1.0),
            font_mapper: FontMapper::new(width as i32, height as i32)?,
            axis: Axis::new(),
            ticks: Ticks::new(),
        })
    }

    pub fn display(self) -> Result<(), PlotError> {
        let axis_font = self.font_mapper.gen_font_map(&self.axis.text.font)?;
        let ticks_font = self.font_mapper.gen_font_map(&self.ticks.text.font)?;
        let scenes = vec![
            self.axis.get_scene(self.mvp, &self.bounds, &axis_font)?,
            self.ticks.get_scene(self.mvp, &self.bounds, &ticks_font)?,
        ];
        unsafe {
            gl::ClearColor(self.bg_color[0], self.bg_color[1], self.bg_color[2], 1.0);
        }
        self.window.run(scenes);
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
    pub z: f32,
}

impl Bounds {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn max(&self) -> f32 {
        self.x.max(self.y).max(self.x)
    }
}

pub static DEFAULT_EYE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
static DEFAULT_FOV: f32 = 50.0 * std::f32::consts::PI / 180.0;
static CAMERA_NEAR: f32 = 0.0;
static CAMERA_FAR: f32 = 10.0;
static DEFAULT_BG: [f32; 3] = [0.1, 0.1, 0.1];

extern crate thiserror;
use thiserror::Error;
extern crate glutin;
use crate::axis::AxisError;
use crate::gl_wrap::ShaderError;
use crate::text::FontMapperError;
use crate::ticks::TicksError;
use glutin::CreationError;
#[derive(Error, Debug)]
pub enum PlotError {
    #[error("{0}")]
    Creation(#[from] CreationError),
    #[error("{0}")]
    Shader(#[from] ShaderError),
    #[error("{0}")]
    Axis(#[from] AxisError),
    #[error("{0}")]
    Ticks(#[from] TicksError),
    #[error("{0}")]
    Font(#[from] FontMapperError),
}
