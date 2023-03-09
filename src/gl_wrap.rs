extern crate gl;
extern crate glutin;
use crate::scene::Scene;
use gl::types::{GLenum, GLint, GLsizeiptr, GLuint};
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, ContextWrapper, CreationError, GlRequest, PossiblyCurrent};
use std::ffi::{CString, NulError};
use std::string::FromUtf8Error;
use std::{fs, ptr};

pub struct Window {
    ctx: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    event_loop: EventLoop<()>,
}

impl Window {
    // initialize window with OpenGl 3.3 context
    pub fn new(title: &str, width: f64, height: f64) -> Result<Self, CreationError> {
        let window = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(title);
        let event_loop = EventLoop::new();
        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_multisampling(4)
            .build_windowed(window, &event_loop)?;
        unsafe {
            let ctx = ctx.make_current().unwrap();
            gl::load_with(|ptr| ctx.get_proc_address(ptr) as *const _);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            Ok(Self { ctx, event_loop })
        }
    }

    // begin draw loop with generic user defined scenes
    pub fn run(self, scenes: Vec<Scene>) {
        self.ctx.swap_buffers().unwrap();
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::LoopDestroyed => {
                    // free gl resources on loop end
                    for scene in &scenes {
                        scene.drop();
                    }
                }
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    for scene in &scenes {
                        scene.draw().unwrap();
                    }
                    self.ctx.swap_buffers().unwrap();
                }
                _ => (),
            }
        })
    }
}

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(source_file: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        // load and compile shader from text file
        let source_code = CString::new(fs::read_to_string(source_file)?)?;
        let shader: Self;
        unsafe {
            shader = Self {
                id: gl::CreateShader(shader_type),
            };
            gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
            gl::CompileShader(shader.id);
        }

        // check if shader compiled successfully
        let mut success: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 1 {
            Ok(shader)
        } else {
            // get shader info log and throw error on compilation failure
            let mut log_size: GLint = 0;
            unsafe {
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut log_size);
            }
            let mut error_log: Vec<u8> = Vec::with_capacity(log_size as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    shader.id,
                    log_size,
                    &mut log_size,
                    error_log.as_mut_ptr() as *mut _,
                );
                error_log.set_len(log_size as usize);
            }
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::Compilation(log))
        }
    }
}

impl Drop for Shader {
    fn drop(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Program {
    pub id: GLuint,
}

impl Program {
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, ProgramError> {
        // link shaders into program
        let program: Self;
        unsafe {
            program = Self {
                id: gl::CreateProgram(),
            };
            gl::AttachShader(program.id, vertex_shader.id);
            gl::AttachShader(program.id, fragment_shader.id);
            gl::LinkProgram(program.id);
        }

        // check if program linked successfully
        let mut success: GLint = 0;
        unsafe {
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        }
        if success == 1 {
            Ok(program)
        } else {
            // get program info log and throw error on linking failure
            let mut log_size: GLint = 0;
            unsafe {
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut log_size);
            }
            let mut error_log: Vec<u8> = Vec::with_capacity(log_size as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program.id,
                    log_size,
                    &mut log_size,
                    error_log.as_mut_ptr() as *mut _,
                );
                error_log.set_len(log_size as usize);
            }
            let log = String::from_utf8(error_log)?;
            Err(ProgramError::Linking(log))
        }
    }

    // constructor from files to minimize initialization steps
    pub fn new_from_files(vertex_file: &str, fragment_file: &str) -> Result<Self, ProgramError> {
        let vertex_shader = Shader::new(vertex_file, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(fragment_file, gl::FRAGMENT_SHADER)?;
        let result = Self::new(&vertex_shader, &fragment_shader);

        // free unneccesary shader resources after linking
        vertex_shader.drop();
        fragment_shader.drop();

        // return result of default constructor
        result
    }

    pub fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, ProgramError> {
        let attrib = CString::new(attrib)?;
        unsafe { Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint) }
    }
}

impl Drop for Program {
    fn drop(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Bind for Program {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

pub struct Buffer {
    pub id: GLuint,
}

impl Buffer {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id }
    }

    pub fn set_data<D>(&self, data: &[D], draw_type: GLuint) {
        self.bind();
        unsafe {
            let (_, bytes, _) = data.align_to::<u8>();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                bytes.len() as GLsizeiptr,
                bytes.as_ptr() as *const _,
                draw_type,
            );
        }
    }

    pub fn new_from<D>(data: &[D], draw_type: GLuint) -> Self {
        let buffer = Buffer::new();
        buffer.set_data(data, draw_type);
        buffer
    }
}

impl Drop for Buffer {
    fn drop(&self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Bind for Buffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}

pub struct VertexArray {
    pub id: GLuint,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    pub fn set_attribute<V: Sized>(&self, location: GLuint, size: GLint, offset_ind: i32) {
        self.bind();
        let stride = std::mem::size_of::<V>() as GLint;
        let offset_ptr = (offset_ind * (core::mem::size_of::<f32>() as i32)) as *const _;
        unsafe {
            gl::VertexAttribPointer(location, size, gl::FLOAT, gl::FALSE, stride, offset_ptr);
            gl::EnableVertexAttribArray(location);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl Bind for VertexArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Texture {
    id: GLuint,
}

impl Texture {
    pub fn new(data: &[u8], width: i32, height: i32) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &data[0] as *const _ as *const std::ffi::c_void,
            );
        }
        Self { id }
    }

    pub fn new_blank(width: i32, height: i32) -> Self {
        let data: Vec<u8> = vec![0; (width * height * 4) as usize];
        Self::new(&data, width, height)
    }
}

impl Drop for Texture {
    fn drop(&self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl Bind for Texture {
    fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

pub struct TextureFramebuffer {
    id: GLuint,
    pub texture: Texture,
    pub width: i32,
    pub height: i32,
    window_width: i32,
    window_height: i32,
}

impl TextureFramebuffer {
    pub fn new(
        width: i32,
        height: i32,
        window_width: i32,
        window_height: i32,
    ) -> Result<Self, FramebufferError> {
        let mut id: GLuint = 0;
        let texture = Texture::new_blank(width, height);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0); // unbind fb texture
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.id,
                0,
            );
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(FramebufferError::CreationError);
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // bind default fb
        }
        Ok(Self {
            id,
            texture,
            width,
            height,
            window_width,
            window_height,
        })
    }

    pub fn bind_default(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Viewport(0, 0, self.window_width, self.window_height);
        }
    }
}

impl Drop for TextureFramebuffer {
    fn drop(&self) {
        unsafe {
            gl::DeleteFramebuffers(1, [self.id].as_ptr());
        }
    }
}

impl Bind for TextureFramebuffer {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::Viewport(0, 0, self.width, self.height);
        }
    }
}

pub struct UniformVec {
    location: i32,
    values: Vec<[f32; 4]>,
}

impl UniformVec {
    pub fn new(program: &Program, name: &str, values: Vec<[f32; 4]>) -> Result<Self, UniformError> {
        let cname = CString::new(name)?;
        let location: i32;
        unsafe {
            location = gl::GetUniformLocation(program.id, cname.as_ptr());
        }
        Ok(Self { location, values })
    }

    pub fn set(&self, i: usize) {
        unsafe {
            gl::Uniform4fv(self.location, 1, &self.values[i][0]);
        }
    }
}

pub struct UniformMat {
    location: i32,
    values: Vec<[f32; 16]>,
}

impl UniformMat {
    pub fn new(
        program: &Program,
        name: &str,
        values: Vec<[f32; 16]>,
    ) -> Result<Self, UniformError> {
        let cname = CString::new(name)?;
        let location: i32;
        unsafe {
            location = gl::GetUniformLocation(program.id, cname.as_ptr());
        }
        Ok(Self { location, values })
    }

    pub fn set(&self, i: usize) {
        unsafe {
            gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &self.values[i][0]);
        }
    }
}

// traits to update gl context state
pub trait Drop {
    fn drop(&self);
}
pub trait Bind {
    fn bind(&self);
}

extern crate thiserror;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Compilation failed: {0}")]
    Compilation(String),
    #[error{"{0}"}]
    Utf8(#[from] FromUtf8Error),
    #[error{"{0}"}]
    Io(#[from] std::io::Error),
    #[error{"{0}"}]
    Nul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("Linking failed: {0}")]
    Linking(String),
    #[error{"{0}"}]
    Utf8(#[from] FromUtf8Error),
    #[error{"{0}"}]
    Shader(#[from] ShaderError),
    #[error{"{0}"}]
    Nul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum FramebufferError {
    #[error("Framebuffer creation failed")]
    CreationError,
}

#[derive(Error, Debug)]
pub enum UniformError {
    #[error("{0}")]
    NulError(#[from] NulError),
}
