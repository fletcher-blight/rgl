//! RGL - Minimal Safe Rust OpenGL Bindings
//!
//! # Overview
//! RGL uses the API reference documentation from <https://www.khronos.org/opengl/wiki/Category:Core_API_Reference>
//!

/// # Template Function Documentation
/// http://kronos/docs
///
/// # Description
///
/// # Arguments
///
/// # Compatability
///
/// # Errors
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// ```
#[allow(unused)]
struct Stub;

pub mod buffer;
pub mod prelude;
pub mod texture;

use gl::types::*;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    ImplementationSpecific(u32),
}

impl From<GLenum> for Error {
    fn from(value: GLenum) -> Self {
        match value {
            gl::NO_ERROR => Error::NoError,
            gl::INVALID_ENUM => Error::InvalidEnum,
            other => Error::ImplementationSpecific(other),
        }
    }
}

/// # Return error information
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetError.xhtml>
pub fn get_error() -> Error {
    // SAFE: just an integer copy
    let error = unsafe { gl::GetError() };
    Error::from(error)
}
