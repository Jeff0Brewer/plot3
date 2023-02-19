extern crate gl;
extern crate glam;
extern crate alloc;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::scene::{Scene, DrawPass};

pub struct Axis {
    bounds: Bounds,
    border_style: BorderStyle,
    border_color: [f32; 4],
    tick_style: TickStyle,
    tick_color: [f32; 4],
    tick_count: i32
}

#[allow(dead_code)]
impl Axis {
    pub fn new() -> Self {
        let bounds = Bounds::new(1.0, 1.0, 1.0);
        let border_style = BorderStyle::Box;
        let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let tick_style = TickStyle::Grid;
        let tick_color: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        let tick_count: i32 = 10;
        Self { bounds, border_style, border_color, tick_style, tick_color, tick_count }
    }

    pub fn get_scene(&self, mvp: [f32; 16]) -> Result<Scene, AxisError> {
        // get axis geometry from current fields
        let (mut lines, tris) = get_axis_border(&self.border_style, &self.bounds);
        let border_line_len = lines.len();
        let mut tick_lines = get_axis_ticks(&self.tick_style, &self.border_style, &self.bounds, &self.tick_count);
        lines.append(&mut tick_lines);

        // init gl resources
        let solid_program = Program::new_from_files(
            "./shaders/solid_vert.glsl",
            "./shaders/solid_frag.glsl"
        )?;
        let mvp_matrix = UniformMatrix::new("mvp", mvp, vec![solid_program.id])?;
        let border_color = UniformVector::new("color", self.border_color, vec![solid_program.id])?;
        let tick_color = UniformVector::new("color", self.tick_color, vec![solid_program.id])?;

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
        let matrices = vec![mvp_matrix];
        let vectors = vec![border_color, tick_color];
        let draw_passes = vec![
            DrawPass::new(gl::LINES, 0, 0, vec![0], vec![1], border_line_len as i32, (lines.len() - border_line_len) as i32),
            DrawPass::new(gl::LINES, 0, 0, vec![0], vec![0], 0, border_line_len as i32),
            DrawPass::new(gl::TRIANGLES, 0, 1, vec![0], vec![0], 0, tris.len() as i32)
        ];
        let scene = Scene::new(draw_passes, programs, vaos, buffers, matrices, vectors);
        Ok(scene)
    }

    pub fn set_border_style(&mut self, style: BorderStyle) {
        self.border_style = style;
    }

    pub fn set_tick_style(&mut self, style: TickStyle) {
        self.tick_style = style;
    }

    pub fn set_border_color(&mut self, color: [f32; 4]) {
        self.border_color = color;
    }

    pub fn set_tick_color(&mut self, color: [f32; 4]) {
        self.tick_color = color;
    }

    pub fn set_bounds(&mut self, x: f32, y: f32, z: f32) {
        self.bounds = Bounds::new(x, y, z);
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

#[allow(dead_code)]
pub enum BorderStyle {
    Arrow,
    Box
}

#[allow(dead_code)]
pub enum TickStyle {
    Tick,
    Grid,
    Blank
}

// sized position vertex with C repr for gl buffering
type Pos = [f32; 3];
#[repr(C, packed)]
struct PosVert(Pos);
// macro to remove init boilerplate
macro_rules! pos_vert {
    ($($pos:expr),*) => {
        vec![$(PosVert($pos),)*]
    }
}

fn get_axis_border(style: &BorderStyle, bounds: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    match style {
        BorderStyle::Arrow => get_arrow_border(bounds),
        BorderStyle::Box => get_box_border(bounds)
    }
}

fn get_arrow_border(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
    const S: f32 = 0.02;
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

fn get_box_border(b: &Bounds) -> (Vec<PosVert>, Vec<PosVert>) {
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

fn get_axis_ticks(style: &TickStyle, border: &BorderStyle, bounds: &Bounds, count: &i32) -> Vec<PosVert> {
    let spacing = bounds.max() / (*count as f32);
    match style {
        TickStyle::Blank => pos_vert![],
        TickStyle::Grid => get_grid(bounds, spacing),
        TickStyle::Tick => {
            // separate funcs for diff border styles since ticks placed on border
            match border {
                BorderStyle::Arrow => get_arrow_ticks(bounds, spacing),
                BorderStyle::Box => get_box_ticks(bounds, spacing)
            }
        }
    }
}

fn get_grid(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, 0.0],
            [x, b.y, 0.0],
            [x, 0.0, 0.0],
            [x, 0.0, b.z]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, y, 0.0],
            [b.x, y, 0.0],
            [0.0, y, 0.0],
            [0.0, y, b.z]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, 0.0, z],
            [b.x, 0.0, z],
            [0.0, 0.0, z],
            [0.0, b.y, z]
        ]);
    }
    lines

}

const TICK_SIZE: f32 = 0.02;

fn get_arrow_ticks(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    const S: f32 = -TICK_SIZE;
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, 0.0],
            [x, S, 0.0],
            [x, 0.0, 0.0],
            [x, 0.0, S]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, y, 0.0],
            [S, y, 0.0],
            [0.0, y, 0.0],
            [0.0, y, S]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [0.0, 0.0, z],
            [S, 0.0, z],
            [0.0, 0.0, z],
            [0.0, S, z]
        ]);
    }

    lines
}

fn get_box_ticks(b: &Bounds, spacing: f32) -> Vec<PosVert> {
    let mut lines = Vec::<PosVert>::new();
    const S: f32 = TICK_SIZE;
    for i in 0..((b.x / spacing) as i32) {
        let x = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [x, 0.0, b.z],
            [x, 0.0, b.z + S]
        ]);
    }
    for i in 0..((b.y / spacing) as i32) {
        let y = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [b.x, y, 0.0],
            [b.x + S, y, 0.0]
        ]);
    }
    for i in 0..((b.z / spacing) as i32) {
        let z = spacing * (i as f32);
        lines.append(&mut pos_vert![
            [b.x, 0.0, z],
            [b.x + S, 0.0, z]
        ]);
    }

    lines
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
