extern crate gl;
extern crate thiserror;
use gl::types::{GLuint, GLint, GLenum};
use thiserror::Error;
use std::{ptr, fs};
use std::ffi::{CString, NulError};
use std::string::FromUtf8Error;

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

pub struct ShaderProgram {
    pub id: GLuint
}

impl ShaderProgram {
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
