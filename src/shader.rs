extern crate gl;
use gl::types::{GLuint, GLint, GLenum};
use std::{fmt, ptr, fs};
use std::ffi::{CString, NulError};
use std::string::FromUtf8Error;
use std::error::Error;

#[derive(Debug)]
pub enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    NulError(NulError),
    Utf8Error(FromUtf8Error),
    IoError(std::io::Error)
}

impl From<NulError> for ShaderError {
    fn from(other: NulError) -> ShaderError {
        ShaderError::NulError(other)
    }
}

impl From<FromUtf8Error> for ShaderError {
    fn from(other: FromUtf8Error) -> ShaderError {
        ShaderError::Utf8Error(other)
    }
}

impl From<std::io::Error> for ShaderError {
    fn from(other: std::io::Error) -> ShaderError {
        ShaderError::IoError(other)
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderError::CompilationError(log) => write!(f, "Compilation Failed: {}", log),
            ShaderError::LinkingError(log) => write!(f, "{}", log),
            ShaderError::NulError(err) => write!(f, "{}", err),
            ShaderError::Utf8Error(err) => write!(f, "{}", err),
            ShaderError::IoError(err) => write!(f, "{}", err)
        }
    }
}

impl Error for ShaderError {}

pub struct Shader {
    pub id: GLuint
}

impl Shader {
    pub unsafe fn new(source_file: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(fs::read_to_string(source_file)?)?;
        let shader = Self {
            id: gl::CreateShader(shader_type)
        };
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

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    pub id: GLuint
}

impl ShaderProgram {
    pub unsafe fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram()
        };
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

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
