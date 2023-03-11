extern crate gl;
use crate::gl_wrap::UniformError;
use crate::gl_wrap::{Bind, Buffer, Drop, Program, Texture, UniformMat, UniformVec, VertexArray};
use gl::types::GLenum;

// struct containing all info for single gl draw operation
pub struct DrawPass {
    pub draw_type: GLenum,
    pub start: i32,
    pub count: i32,
    pub inds: DrawInds,
}

pub struct DrawInds {
    pub program: usize,
    pub vao: usize,
    pub texture: Option<usize>,
    pub matrix: Vec<[usize; 2]>,
    pub vector: Vec<[usize; 2]>,
}

impl DrawPass {
    pub fn draw(
        &self,
        programs: &[Program],
        vaos: &[VertexArray],
        textures: &[Texture],
        matrices: &[UniformMat],
        vectors: &[UniformVec],
    ) -> Result<(), UniformError> {
        let program = &programs[self.inds.program];
        program.bind();
        vaos[self.inds.vao].bind();
        if let Some(ind) = self.inds.texture {
            textures[ind].bind();
        }
        for &m in &self.inds.matrix {
            matrices[m[0]].set(m[1]);
        }
        for &v in &self.inds.vector {
            vectors[v[0]].set(v[1]);
        }
        unsafe {
            gl::DrawArrays(self.draw_type, self.start, self.count);
        }
        Ok(())
    }
}

// struct containing all gl resources and draw operations for complex scene
pub struct Scene {
    pub passes: Vec<DrawPass>,
    pub programs: Vec<Program>,
    pub vaos: Vec<VertexArray>,
    pub buffers: Vec<Buffer>,
    pub textures: Vec<Texture>,
    pub matrices: Vec<UniformMat>,
    pub vectors: Vec<UniformVec>,
}

impl Scene {
    pub fn new_empty() -> Self {
        let passes = Vec::<DrawPass>::new();
        let programs = Vec::<Program>::new();
        let vaos = Vec::<VertexArray>::new();
        let buffers = Vec::<Buffer>::new();
        let textures = Vec::<Texture>::new();
        let matrices = Vec::<UniformMat>::new();
        let vectors = Vec::<UniformVec>::new();
        Self {
            passes,
            programs,
            vaos,
            buffers,
            textures,
            matrices,
            vectors,
        }
    }

    pub fn draw(&self) -> Result<(), UniformError> {
        for pass in &self.passes {
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
