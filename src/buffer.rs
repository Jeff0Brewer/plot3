extern crate gl;
use gl::types::{GLuint, GLint, GLsizeiptr};

pub struct Buffer {
    pub id: GLuint,
    target: GLuint
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn set_data<D>(&self, data: &[D], draw_type: GLuint) {
        self.bind();
        let (_, bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            bytes.len() as GLsizeiptr,
            bytes.as_ptr() as *const _,
            draw_type
        );
    }
}

pub struct VertexArray {
    pub id: GLuint
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    pub unsafe fn set_attribute<V: Sized>(
        &self,
        index: GLuint,
        size: GLint,
        offset: GLint
    ) {
        self.bind();
        let off_ptr = offset * (core::mem::size_of::<f32>() as i32);
        gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            off_ptr as *const _
        );
        gl::EnableVertexAttribArray(index);
    }
}
