//! Specify the value of a uniform variable for the current program object
//!
//! # Description
//! [uniform] api modify the value of a uniform variable or a uniform variable array. The
//! location of the uniform variable to be modified is specified by location, which should be a
//! value returned by [get_uniform_location]. [uniform] api operate on the program object
//! that was made part of current state by calling [use_program].
//!
//! The commands uniform_{1|2|3|4}{f32|i32|u32} are used to change the value of the uniform variable
//! specified by `location` using the values passed as arguments. The number specified in the command
//! should match the number of components in the data type of the specified uniform variable
//! (e.g., 1 for float, int, unsigned int, bool; 2 for vec2, ivec2, uvec2, bvec2, etc.). The suffix
//! f32 indicates that floating-point values are being passed; the suffix i32 indicates that integer
//! values are being passed; the suffix u32 indicates that unsigned integer values are being passed,
//! and this type should also match the data type of the specified uniform variable. The i32 variants
//! of this function should be used to provide values for uniform variables defined as int, ivec2, ivec3,
//! ivec4, or arrays of these. The u32 variants of this function should be used to provide values for
//! uniform variables defined as unsigned int, uvec2, uvec3, uvec4, or arrays of these. The f32 variants
//! should be used to provide values for uniform variables of type float, vec2, vec3, vec4, or arrays
//! of these. Either the i32, u32 or f32 variants may be used to provide values for uniform variables
//! of type bool, bvec2, bvec3, bvec4, or arrays of these. The uniform variable will be set to
//! false if the input value is 0 or 0.0f32, and it will be set to true otherwise.
//!
//! All active uniform variables defined in a program object are initialized to 0 when the program
//! object is linked successfully. They retain the values assigned to them by a call to a [uniform]
//! function until the next successful link operation occurs on the program object, when they are
//! once again initialized to 0.
//!
//! The commands uniform_{1|2|3|4}{f32|i32|u32}v can be used to modify a single uniform variable or
//! a uniform variable array. These commands pass a slice of values to be loaded into a uniform
//! variable or a uniform variable array. A slice of 1 should be used if modifying the value of a
//! single uniform variable, and a slice of 1 or greater can be used to modify an entire array or
//! part of an array. When loading n elements starting at an arbitrary position m in a uniform variable
//! array, elements m + n - 1 in the array will be replaced with the new values. If m + n - 1 is
//! larger than the size of the uniform variable array, values for all array elements beyond the end
//! of the array will be ignored. The number specified in the name of the command indicates the number
//! of components for each element in value, and it should match the number of components in the
//! data type of the specified uniform variable (e.g., 1 for float, int, bool; 2 for vec2, ivec2,
//! bvec2, etc.). The data type specified in the name of the command must match the data type for
//! the specified uniform variable as described previously for uniform_{1|2|3|4}{f32|i32|u32}.
//!
//! For uniform variable arrays, each element of the array is considered to be of the type indicated
//! in the name of the command (e.g., [uniform_3f32] or [uniform_3f32v] can be used to load a uniform
//! variable array of type vec3). The number of elements of the uniform variable array to be modified
//! is specified by the number of elements in the `value` slice.
//!
//! The commands uniformMatrix_{2|3|4|2x3|3x2|2x4|4x2|3x4|4x3}f32v are used to modify a matrix or an
//! array of matrices. The numbers in the command name are interpreted as the dimensionality of the
//! matrix. The number 2 indicates a 2 × 2 matrix (i.e., 4 values), the number 3 indicates a 3 × 3
//! matrix (i.e., 9 values), and the number 4 indicates a 4 × 4 matrix (i.e., 16 values). Non-square
//! matrix dimensionality is explicit, with the first number representing the number of columns and
//! the second number representing the number of rows. For example, 2x4 indicates a 2 × 4 matrix with
//! 2 columns and 4 rows (i.e., 8 values). If transpose is false, each matrix is assumed to be supplied
//! in column major order. If transpose is true, each matrix is assumed to be supplied in row major order.
//! A slice of 1 should be used if modifying the value of a single matrix, and a slice greater than
//! 1 can be used to modify an array of matrices.
//!
//! # Notes
//! - [uniform_1i32] and [uniform_1i32v] are the only two api that may be used to load uniform
//! variables defined as sampler types. Loading samplers with any other function will result in a
//! [InvalidOperation](ErrorOpenGL::InvalidOperation) error.
//! - If slice len is greater than 1 and the indicated uniform variable is not an array, a
//! [InvalidOperation](ErrorOpenGL::InvalidOperation) error is generated and the specified uniform
//! variable will remain unchanged.
//! - Other than the preceding exceptions, if the type and size of the uniform variable as defined
//! in the shader do not match the type and size specified in the name of the command used to load
//! its value, a [InvalidOperation](ErrorOpenGL::InvalidOperation) error will be generated and the
//! specified uniform variable will remain unchanged.
//! - If `location` is a value other than -1 and it does not represent a valid uniform variable location
//! in the current program object, an error will be generated, and no changes will be made to the
//! uniform variable storage of the current program object. If `location` is equal to -1, the data
///! passed in will be silently ignored and the specified uniform variable will not be changed.
use crate::*;
use gl::types::*;

/// Specify a single f32 value of a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1f32(location: UniformLocation, v0: f32) -> Result<(), Error> {
    unsafe { gl::Uniform1f(location.0, v0) }
    internal_handle_uniform_set_error()
}

/// Specify a dual f32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2f32(location: UniformLocation, v0: f32, v1: f32) -> Result<(), Error> {
    unsafe { gl::Uniform2f(location.0, v0, v1) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet f32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3f32(location: UniformLocation, v0: f32, v1: f32, v2: f32) -> Result<(), Error> {
    unsafe { gl::Uniform3f(location.0, v0, v1, v2) }
    internal_handle_uniform_set_error()
}

/// Specify a quad f32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4f32(
    location: UniformLocation,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
) -> Result<(), Error> {
    unsafe { gl::Uniform4f(location.0, v0, v1, v2, v3) }
    internal_handle_uniform_set_error()
}

/// Specify a single i32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1i32(location: UniformLocation, v0: i32) -> Result<(), Error> {
    unsafe { gl::Uniform1i(location.0, v0) }
    internal_handle_uniform_set_error()
}

/// Specify a dual i32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2i32(location: UniformLocation, v0: i32, v1: i32) -> Result<(), Error> {
    unsafe { gl::Uniform2i(location.0, v0, v1) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet i32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3i32(location: UniformLocation, v0: i32, v1: i32, v2: i32) -> Result<(), Error> {
    unsafe { gl::Uniform3i(location.0, v0, v1, v2) }
    internal_handle_uniform_set_error()
}

/// Specify a quad i32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4i32(
    location: UniformLocation,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
) -> Result<(), Error> {
    unsafe { gl::Uniform4i(location.0, v0, v1, v2, v3) }
    internal_handle_uniform_set_error()
}

/// Specify a single u32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1u32(location: UniformLocation, v0: u32) -> Result<(), Error> {
    unsafe { gl::Uniform1ui(location.0, v0) }
    internal_handle_uniform_set_error()
}

/// Specify a dual u32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2u32(location: UniformLocation, v0: u32, v1: u32) -> Result<(), Error> {
    unsafe { gl::Uniform2ui(location.0, v0, v1) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet u32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3u32(location: UniformLocation, v0: u32, v1: u32, v2: u32) -> Result<(), Error> {
    unsafe { gl::Uniform3ui(location.0, v0, v1, v2) }
    internal_handle_uniform_set_error()
}

/// Specify a quad u32 value for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4u32(
    location: UniformLocation,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
) -> Result<(), Error> {
    unsafe { gl::Uniform4ui(location.0, v0, v1, v2, v3) }
    internal_handle_uniform_set_error()
}

/// Specify a single f32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1f32v(location: UniformLocation, value: &[f32]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::Uniform1fv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a dual f32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2f32v(location: UniformLocation, value: &[(f32, f32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::Uniform2fv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet f32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3f32v(location: UniformLocation, value: &[(f32, f32, f32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::Uniform3fv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a quad f32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4f32v(
    location: UniformLocation,
    value: &[(f32, f32, f32, f32)],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::Uniform4fv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a single i32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1i32v(location: UniformLocation, value: &[i32]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLint;
    unsafe { gl::Uniform1iv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a dual i32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2i32v(location: UniformLocation, value: &[(i32, i32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLint;
    unsafe { gl::Uniform2iv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet i32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3i32v(location: UniformLocation, value: &[(i32, i32, i32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLint;
    unsafe { gl::Uniform3iv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a quad i32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4i32v(
    location: UniformLocation,
    value: &[(i32, i32, i32, i32)],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLint;
    unsafe { gl::Uniform4iv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a single u32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_1u32v(location: UniformLocation, value: &[u32]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLuint;
    unsafe { gl::Uniform1uiv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a dual u32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_2u32v(location: UniformLocation, value: &[(u32, u32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLuint;
    unsafe { gl::Uniform2uiv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a triplet u32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_3u32v(location: UniformLocation, value: &[(u32, u32, u32)]) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLuint;
    unsafe { gl::Uniform3uiv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a quad u32 value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_4u32v(
    location: UniformLocation,
    value: &[(u32, u32, u32, u32)],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let value = value.as_ptr() as *const GLuint;
    unsafe { gl::Uniform4uiv(location.0, count, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 2x2 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_2f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 4]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix2fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 3x3 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_3f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 9]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix3fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 4x4 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_4f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 16]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix4fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 2x3 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_2x3f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 6]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix2x3fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 3x2 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_3x2f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 6]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix3x2fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 2x4 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_2x4f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 8]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix2x4fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 4x2 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_4x2f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 8]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix4x2fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 3x4 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_3x4f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 12]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix3x4fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

/// Specify a 4x3 matrix value array for a uniform variable
///
/// See [uniform] for more details
pub fn uniform_matrix_4x3f32v(
    location: UniformLocation,
    transpose: bool,
    value: &[[f32; 12]],
) -> Result<(), Error> {
    let count = value.len() as GLsizei;
    let transpose = as_gl_bool(transpose);
    let value = value.as_ptr() as *const GLfloat;
    unsafe { gl::UniformMatrix4x3fv(location.0, count, transpose, value) }
    internal_handle_uniform_set_error()
}

fn internal_handle_uniform_set_error() -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        error => Err(Error::Unreachable(error)),
    }
}

fn as_gl_bool(value: bool) -> GLboolean {
    match value {
        true => gl::TRUE,
        false => gl::FALSE,
    }
}
