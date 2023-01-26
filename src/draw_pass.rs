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
