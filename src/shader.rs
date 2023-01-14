extern crate gl;
use gl::types::{GLuint, GLenum};
use std::error::Error;
use std::fmt;
use std::ffi::{CString, NulError};

#[derive(Debug)]
enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    NulError(NulError)
}

impl From<NulError> for ShaderError {
    fn from(other: NulError) -> ShaderError {
        ShaderError::NulError(other)
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderError::CompilationError(log) => write!(f, "{}", log),
            ShaderError::LinkingError(log) => write!(f, "{}", log),
            ShaderError::NulError(err) => write!(f, "{}", err)
        }
    }
}

impl Error for ShaderError {}

pub struct Shader {
    pub id: GLuint
}

impl Shader {
    fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;
        let log = "comp err";
        Err(ShaderError::CompilationError(log.to_string()))
    }
}
