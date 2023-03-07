//! RGL - Minimal Safe Rust OpenGL Bindings
//!
//! # Overview
//! RGL uses the API reference documentation from <https://www.khronos.org/opengl/wiki/Category:Core_API_Reference>
//!

/// # Template Function Documentation
/// <http://kronos/docs>
///
/// # Arguments
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// ```
///
/// # Description
///
/// # Compatability
///
/// # Errors
///
/// # Associated Gets
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [my_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
#[allow(unused)]
struct Stub;

pub mod buffer;
pub mod masks;
pub mod prelude;
pub mod shader_creation;
pub mod shader_state;
pub mod texture;
pub mod vertex_array;
pub mod vertex_render;

use gl::types::*;

pub use gl::load_with;

#[derive(Debug, Copy, Clone, PartialEq)]
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
