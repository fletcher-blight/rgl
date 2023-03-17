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
pub mod get;
pub mod masks;
pub mod prelude;
pub mod shader_creation;
pub mod shader_query;
pub mod shader_state;
pub mod texture;
pub mod vertex_array;
pub mod vertex_render;

use gl::types::*;

pub use gl::load_with;
