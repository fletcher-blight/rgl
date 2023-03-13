//! # Return the value or values of a selected parameter
//! <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml>

use crate::prelude::*;
use gl::types::*;

fn get_bool(pname: GLenum) -> bool {
    let mut val = GLboolean::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetBooleanv(pname, &mut val) };
    val == gl::TRUE
}

fn get_f64(pname: GLenum) -> f64 {
    let mut val = GLdouble::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetDoublev(pname, &mut val) };
    val
}

fn get_f32(pname: GLenum) -> f32 {
    let mut val = GLfloat::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetFloatv(pname, &mut val) };
    val
}

fn get_i32(pname: GLenum) -> i32 {
    let mut val = GLint::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetIntegerv(pname, &mut val) };
    val
}

/// # Return the value or values of a selected parameter
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml>
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// active_texture(42);
/// assert_eq!(get_active_texture(), 42);
/// ```
///
/// # Description
/// Returns a single value indicating the active multitexture unit. The initial value is `0``. See
/// [active_texture].
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
/// | [get_active_texture] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
pub fn get_active_texture() -> u32 {
    get_i32(gl::ACTIVE_TEXTURE) as u32
}
