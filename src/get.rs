//! Return the value or values of a selected parameter
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

pub fn get_active_texture() -> u32 {
    get_i32(gl::ACTIVE_TEXTURE) as u32
}
