extern crate gl;
use crate::gl_wrap::UniformError;
use crate::gl_wrap::{Bind, Buffer, Drop, Program, Texture, UniformMat, UniformVec, VertexArray};
use gl::types::GLenum;

// struct containing all info for single gl draw operation
pub struct DrawPass {
    draw_type: GLenum,
    program_ind: usize,
    vao_ind: usize,
    texture_ind: Option<usize>,
    matrix_inds: Vec<[usize; 2]>,
    vector_inds: Vec<[usize; 2]>,
    draw_start: i32,
    draw_end: i32,
}

impl DrawPass {
    pub fn new(
        draw_type: GLenum,
        program_ind: usize,
        vao_ind: usize,
        texture_ind: Option<usize>,
        matrix_inds: Vec<[usize; 2]>,
        vector_inds: Vec<[usize; 2]>,
        draw_start: i32,
        draw_end: i32,
    ) -> Self {
        Self {
            draw_type,
            program_ind,
            vao_ind,
            texture_ind,
            matrix_inds,
            vector_inds,
            draw_start,
            draw_end,
        }
    }

    pub fn draw(
        &self,
        programs: &[Program],
        vaos: &[VertexArray],
        textures: &[Texture],
        matrices: &[UniformMat],
        vectors: &[UniformVec],
    ) -> Result<(), UniformError> {
        let program = &programs[self.program_ind];
        program.bind();
        vaos[self.vao_ind].bind();
        if let Some(ind) = self.texture_ind {
            textures[ind].bind();
        }
        for &m in &self.matrix_inds {
            matrices[m[0]].set(m[1]);
        }
        for &v in &self.vector_inds {
            vectors[v[0]].set(v[1]);
        }
        unsafe { gl::DrawArrays(self.draw_type, self.draw_start, self.draw_end) }
        Ok(())
    }
}

// struct containing all gl resources and draw operations for complex scene
pub struct Scene {
    draws: Vec<DrawPass>,
    programs: Vec<Program>,
    vaos: Vec<VertexArray>,
    buffers: Vec<Buffer>,
    textures: Vec<Texture>,
    matrices: Vec<UniformMat>,
    vectors: Vec<UniformVec>,
}

impl Scene {
    pub fn new(
        draws: Vec<DrawPass>,
        programs: Vec<Program>,
        vaos: Vec<VertexArray>,
        buffers: Vec<Buffer>,
        textures: Vec<Texture>,
        matrices: Vec<UniformMat>,
        vectors: Vec<UniformVec>,
    ) -> Self {
        Self {
            draws,
            programs,
            vaos,
            buffers,
            textures,
            matrices,
            vectors,
        }
    }

    pub fn new_empty() -> Self {
        let draws = Vec::<DrawPass>::new();
        let programs = Vec::<Program>::new();
        let vaos = Vec::<VertexArray>::new();
        let buffers = Vec::<Buffer>::new();
        let textures = Vec::<Texture>::new();
        let matrices = Vec::<UniformMat>::new();
        let vectors = Vec::<UniformVec>::new();
        Self {
            draws,
            programs,
            vaos,
            buffers,
            textures,
            matrices,
            vectors,
        }
    }

    pub fn draw(&self) -> Result<(), UniformError> {
        for pass in &self.draws {
            // do not pass in buffers since buffer state is stored in vaos
            pass.draw(
                &self.programs,
                &self.vaos,
                &self.textures,
                &self.matrices,
                &self.vectors,
            )?;
        }
        Ok(())
    }

    pub fn drop(&self) {
        for program in &self.programs {
            program.drop();
        }
        for vao in &self.vaos {
            vao.drop();
        }
        for texture in &self.textures {
            texture.drop();
        }
        // references stored for buffers only to drop on scene deletion
        for buffer in &self.buffers {
            buffer.drop();
        }
    }
}
