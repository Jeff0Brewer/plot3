extern crate gl;
use crate::gl_wrap::UniformError;
use crate::gl_wrap::{Bind, Buffer, Drop, Program, Texture, Uniform, VertexArray};
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
    pub uniform: Vec<usize>,
}

impl DrawPass {
    pub fn draw(
        &self,
        programs: &[Program],
        vaos: &[VertexArray],
        textures: &[Texture],
        uniforms: &[Uniform],
    ) -> Result<(), UniformError> {
        let program = &programs[self.inds.program];
        program.bind();
        vaos[self.inds.vao].bind();
        if let Some(ind) = self.inds.texture {
            textures[ind].bind();
        }
        for &i in &self.inds.uniform {
            uniforms[i].set()?;
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
    pub uniforms: Vec<Uniform>,
}

impl Scene {
    pub fn draw(&self) -> Result<(), UniformError> {
        for pass in &self.passes {
            // do not pass in buffers since buffer state is stored in vaos
            pass.draw(&self.programs, &self.vaos, &self.textures, &self.uniforms)?;
        }
        Ok(())
    }
}

impl Drop for Scene {
    fn drop(&self) {
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
