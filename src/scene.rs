extern crate gl;
use gl::types::GLenum;
use crate::gl_wrap::{Program, Buffer, VertexArray};

pub struct DrawPass {
    draw_type: GLenum,
    program_ind: usize,
    buffer_ind: usize,
    attrib_ind: usize,
    draw_start: i32,
    draw_end: i32
}

impl DrawPass {
    pub fn new(
        draw_type: GLenum,
        program_ind: usize,
        buffer_ind: usize,
        attrib_ind: usize,
        draw_start: i32,
        draw_end: i32
    ) -> Self {
        Self { draw_type, program_ind, buffer_ind, attrib_ind, draw_start, draw_end }
    }

    pub fn draw(
        &self,
        programs: &Vec<Program>,
        buffers: &Vec<Buffer>,
        attribs: &Vec<VertexArray>
    ) {
        programs[self.program_ind].apply();
        buffers[self.buffer_ind].bind();
        attribs[self.attrib_ind].bind();
        unsafe { gl::DrawArrays(self.draw_type, self.draw_start, self.draw_end) }
    }
}

pub struct Scene {
    pub draws: Vec<DrawPass>,
    pub programs: Vec<Program>,
    pub buffers: Vec<Buffer>,
    pub attribs: Vec<VertexArray>
}

impl Scene {
    pub fn new(
        draws: Vec<DrawPass>,
        programs: Vec<Program>,
        buffers: Vec<Buffer>,
        attribs: Vec<VertexArray>
    ) -> Self {
        Self { draws, programs, buffers, attribs }
    }

    pub fn new_empty() -> Self {
        let draws = Vec::<DrawPass>::new();
        let programs = Vec::<Program>::new();
        let buffers = Vec::<Buffer>::new();
        let attribs = Vec::<VertexArray>::new();
        Self { draws, programs, buffers, attribs }
    }

    pub fn draw(&self) {
        for pass in &self.draws {
            pass.draw(&self.programs, &self.buffers, &self.attribs);
        }
    }
}
