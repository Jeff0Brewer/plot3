extern crate gl;
use gl::types::GLenum;
use crate::gl_wrap::{Program, Buffer, VertexArray, UniformMatrix, UniformVector};
use crate::gl_wrap::{UniformError};

// struct containing all info for single gl draw operation
pub struct DrawPass {
    draw_type: GLenum,
    program_ind: usize,
    buffer_ind: usize,
    attrib_ind: usize,
    matrix_inds: Vec<usize>,
    vector_inds: Vec<usize>,
    draw_start: i32,
    draw_end: i32
}

impl DrawPass {
    pub fn new(
        draw_type: GLenum,
        program_ind: usize,
        buffer_ind: usize,
        attrib_ind: usize,
        matrix_inds: Vec<usize>,
        vector_inds: Vec<usize>,
        draw_start: i32,
        draw_end: i32
    ) -> Self {
        Self {
            draw_type,
            program_ind,
            buffer_ind,
            attrib_ind,
            matrix_inds,
            vector_inds,
            draw_start,
            draw_end
        }
    }

    pub fn draw(
        &self,
        programs: &Vec<Program>,
        buffers: &Vec<Buffer>,
        attribs: &Vec<VertexArray>,
        matrices: &Vec<UniformMatrix>,
        vectors: &Vec<UniformVector>
    ) -> Result<(), UniformError> {
        let program = &programs[self.program_ind];
        program.apply();
        //buffers[self.buffer_ind].bind();
        attribs[self.attrib_ind].bind();
        for &i in &self.matrix_inds { matrices[i].apply(program.id)?; }
        for &i in &self.vector_inds { vectors[i].apply(program.id)?; }
        unsafe { gl::DrawArrays(self.draw_type, self.draw_start, self.draw_end) }
        Ok(())
    }
}

// struct containing all gl resources and draw operations for complex scene
pub struct Scene {
    draws: Vec<DrawPass>,
    programs: Vec<Program>,
    buffers: Vec<Buffer>,
    attribs: Vec<VertexArray>,
    matrices: Vec<UniformMatrix>,
    vectors: Vec<UniformVector>
}

impl Scene {
    pub fn new(
        draws: Vec<DrawPass>,
        programs: Vec<Program>,
        buffers: Vec<Buffer>,
        attribs: Vec<VertexArray>,
        matrices: Vec<UniformMatrix>,
        vectors: Vec<UniformVector>
    ) -> Self {
        Self { draws, programs, buffers, attribs, matrices, vectors }
    }

    pub fn new_empty() -> Self {
        let draws = Vec::<DrawPass>::new();
        let programs = Vec::<Program>::new();
        let buffers = Vec::<Buffer>::new();
        let attribs = Vec::<VertexArray>::new();
        let matrices = Vec::<UniformMatrix>::new();
        let vectors = Vec::<UniformVector>::new();
        Self { draws, programs, buffers, attribs, matrices, vectors }
    }

    pub fn draw(&self) -> Result<(), UniformError> {
        for pass in &self.draws {
            pass.draw(&self.programs, &self.buffers, &self.attribs, &self.matrices, &self.vectors)?;
        }
        Ok(())
    }

    pub fn drop(&self) {
        for program in &self.programs { program.drop(); }
        for buffer in &self.buffers { buffer.drop(); }
        for attrib in &self.attribs { attrib.drop(); }
    }
}
