extern crate glutin;
extern crate gl;
extern crate thiserror;
use glutin::{ContextWrapper, ContextBuilder, GlRequest, Api, CreationError, PossiblyCurrent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent};
use glutin::window::WindowBuilder;
use gl::types::{GLuint, GLint, GLenum, GLsizeiptr};
use thiserror::Error;
use std::{ptr, fs};
use std::ffi::{CString, NulError};
use std::string::FromUtf8Error;

pub struct Window {
    ctx: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>
}

impl Window {
    pub unsafe fn new(title: &str) -> Result<Self, CreationError> {
        let window = WindowBuilder::new().with_title(title);
        let event_loop = EventLoop::new();
        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .build_windowed(window, &event_loop)?;
        let ctx = ctx.make_current().unwrap();
        gl::load_with(|ptr| ctx.get_proc_address(ptr) as *const _);
        Ok(Self { ctx, event_loop })
    }

    pub unsafe fn run<F: Fn() -> () + 'static>(self, draw: F) -> () {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => ()
                },
                Event::RedrawRequested(_) => {
                    draw();
                    self.ctx.swap_buffers().unwrap();
                },
                _ => ()
            }
        })
    }
}

pub struct Shader {
    pub id: GLuint
}

impl Shader {
    pub unsafe fn new(source_file: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let shader = Self { id: gl::CreateShader(shader_type) };
        let source_code = CString::new(fs::read_to_string(source_file)?)?;
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        if success == 1 {
            Ok(shader)
        } else {
            let mut log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(log_size as usize);
            gl::GetShaderInfoLog(shader.id, log_size, &mut log_size, error_log.as_mut_ptr() as *mut _);
            error_log.set_len(log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }
}

pub struct Program {
    pub id: GLuint
}

impl Program {
    pub unsafe fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, ShaderError> {
        let program = Self { id: gl::CreateProgram() };
        gl::AttachShader(program.id, vertex_shader.id);
        gl::AttachShader(program.id, fragment_shader.id);
        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        if success == 1 {
            Ok(program)
        } else {
            let mut log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(log_size as usize);
            gl::GetProgramInfoLog(program.id, log_size, &mut log_size, error_log.as_mut_ptr() as *mut _);
            error_log.set_len(log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }
}

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

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Compilation Failed: {0}")]
    CompilationError(String),
    #[error("Linking Failed: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
    #[error{"{0}"}]
    IoError(#[from] std::io::Error)
}
