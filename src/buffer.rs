extern crate gl;
use gl::types::{GLuint, GLint, GLsizeiptr};

pub struct Buffer {
    pub id: GLuint
}

impl Buffer {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id }
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    pub unsafe fn set_data<D>(&self, data: &[D], draw_type: GLuint) {
        self.bind();
        let (_, bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            gl::ARRAY_BUFFER,
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

    pub unsafe fn set_attribute<V: Sized>(&self, index: GLuint, size: GLint, offset_ind: i32) {
        self.bind();
        let stride = std::mem::size_of::<V>() as GLint;
        let offset_ptr = (offset_ind * (core::mem::size_of::<f32>() as i32)) as *const _;
        gl::VertexAttribPointer(index, size, gl::FLOAT, gl::FALSE, stride, offset_ptr);
        gl::EnableVertexAttribArray(index);
    }
}
