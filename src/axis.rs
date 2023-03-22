extern crate alloc;
extern crate gl;
extern crate glam;
use crate::gl_wrap::{Buffer, Program, Uniform, VertexArray};
use crate::plot::Bounds;
use crate::scene::{DrawInds, DrawPass, Scene};
use crate::text::{FontMap, TextParams, DEFAULT_FONT};
use crate::vertices::{pos_vert, PosVert, TextVert};

pub struct AxisLabels {
    pub x: String,
    pub y: String,
    pub z: String,
    pub param: TextParams,
}

impl AxisLabels {
    pub fn new() -> Self {
        Self {
            x: "x axis".to_string(),
            y: "y axis".to_string(),
            z: "z axis".to_string(),
            param: TextParams {
                font: DEFAULT_FONT.to_string(),
                size: 14.0,
                kearning: 0.0,
            },
        }
    }
}

pub struct Axis {
    pub color: [f32; 4],
    pub labels: AxisLabels,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            labels: AxisLabels::new(),
        }
    }

    pub fn get_scene(
        &self,
        mvp: [f32; 16],
        bounds: &Bounds,
        font: &FontMap,
    ) -> Result<Scene, AxisError> {
        // get vertex data for scene
        let line_verts = Axis::get_verts(bounds);
        let orient = Axis::get_label_orient(bounds);
        let mut text_verts = Vec::<TextVert>::new();
        text_verts.append(&mut font.get_verts(&self.labels.x, &self.labels.param, orient.x.pos)?);
        let x_vlen = text_verts.len() as i32;
        text_verts.append(&mut font.get_verts(&self.labels.y, &self.labels.param, orient.y.pos)?);
        let y_vlen = text_verts.len() as i32;
        text_verts.append(&mut font.get_verts(&self.labels.z, &self.labels.param, orient.z.pos)?);
        let z_vlen = text_verts.len() as i32;

        // init gl resources for line drawing
        const LINE_VERT: &str = "./shaders/solid_vert.glsl";
        const LINE_FRAG: &str = "./shaders/solid_frag.glsl";
        let line_program = Program::new_from_files(LINE_VERT, LINE_FRAG)?;
        let line_vao = VertexArray::new();
        let line_buffer = Buffer::new_from(&line_verts, gl::STATIC_DRAW);
        let line_pos_loc = line_program.get_attrib_location("position")?;
        line_vao.set_attribute::<PosVert>(line_pos_loc, 3, 0);
        let u_mvp_line = Uniform::new(&line_program, "mvp", &mvp)?;
        let u_color = Uniform::new(&line_program, "color", &self.color)?;

        // init gl resources for text drawing
        const TEXT_VERT: &str = "./shaders/text_align_vert.glsl";
        const TEXT_FRAG: &str = "./shaders/text_frag.glsl";
        let text_program = Program::new_from_files(TEXT_VERT, TEXT_FRAG)?;
        let text_vao = VertexArray::new();
        let text_buffer = Buffer::new_from(&text_verts, gl::STATIC_DRAW);
        let text_pos_loc = text_program.get_attrib_location("position")?;
        let text_off_loc = text_program.get_attrib_location("offset")?;
        let text_tco_loc = text_program.get_attrib_location("a_texCoord")?;
        text_vao.set_attribute::<TextVert>(text_pos_loc, 3, 0);
        text_vao.set_attribute::<TextVert>(text_off_loc, 2, 3);
        text_vao.set_attribute::<TextVert>(text_tco_loc, 2, 5);
        let scale = font.scale * self.labels.param.size;
        let u_scale = Uniform::new(&text_program, "scale", &[scale])?;
        let u_align_x = Uniform::new(&text_program, "alignment", &orient.x.align)?;
        let u_align_y = Uniform::new(&text_program, "alignment", &orient.y.align)?;
        let u_align_z = Uniform::new(&text_program, "alignment", &orient.z.align)?;
        let u_mvp_text = Uniform::new(&text_program, "mvp", &mvp)?;

        let scene = Scene {
            programs: vec![line_program, text_program],
            vaos: vec![line_vao, text_vao],
            buffers: vec![line_buffer, text_buffer],
            textures: vec![font.texture],
            uniforms: vec![
                u_mvp_line, u_color, u_mvp_text, u_scale, u_align_x, u_align_y, u_align_z,
            ],
            passes: vec![
                // axis lines
                DrawPass {
                    draw_type: gl::LINES,
                    start: 0,
                    count: line_verts.len() as i32,
                    inds: DrawInds {
                        program: 0,
                        vao: 0,
                        texture: None,
                        uniform: vec![0, 1],
                    },
                },
                // x label
                DrawPass {
                    draw_type: gl::TRIANGLES,
                    start: 0,
                    count: x_vlen,
                    inds: DrawInds {
                        program: 1,
                        vao: 1,
                        texture: Some(0),
                        uniform: vec![2, 3, 4],
                    },
                },
                // y label
                DrawPass {
                    draw_type: gl::TRIANGLES,
                    start: x_vlen,
                    count: y_vlen - x_vlen,
                    inds: DrawInds {
                        program: 1,
                        vao: 1,
                        texture: Some(0),
                        uniform: vec![2, 3, 5],
                    },
                },
                // z label
                DrawPass {
                    draw_type: gl::TRIANGLES,
                    start: y_vlen,
                    count: z_vlen - y_vlen,
                    inds: DrawInds {
                        program: 1,
                        vao: 1,
                        texture: Some(0),
                        uniform: vec![2, 3, 6],
                    },
                },
            ],
        };
        Ok(scene)
    }

    fn get_verts(b: &Bounds) -> Vec<PosVert> {
        pos_vert![
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
        ]
    }

    fn get_label_orient(b: &Bounds) -> LabelOrientations {
        const M: f32 = 0.15; // label margin
        LabelOrientations {
            x: LabelOrientation {
                pos: [b.x * 0.5, 0.0, b.z + M],
                align: [1.0, 0.0, 0.0],
            },
            y: LabelOrientation {
                pos: [b.x + M, b.y * 0.5, 0.0],
                align: [0.0, -1.0, 0.0],
            },
            z: LabelOrientation {
                pos: [b.x + M, 0.0, b.z * 0.5],
                align: [0.0, 0.0, 1.0],
            },
        }
    }
}

struct LabelOrientations {
    pub x: LabelOrientation,
    pub y: LabelOrientation,
    pub z: LabelOrientation,
}

struct LabelOrientation {
    pub pos: [f32; 3],
    pub align: [f32; 3],
}

extern crate thiserror;
use crate::gl_wrap::{ProgramError, ShaderError, UniformError};
use crate::text::FontMapError;
use std::ffi::NulError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum AxisError {
    #[error("{0}")]
    Shader(#[from] ShaderError),
    #[error("{0}")]
    Program(#[from] ProgramError),
    #[error("{0}")]
    Uniform(#[from] UniformError),
    #[error("{0}")]
    Nul(#[from] NulError),
    #[error("{0}")]
    Font(#[from] FontMapError),
}
