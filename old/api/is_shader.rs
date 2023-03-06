use crate::*;

/// Determines if a name corresponds to a shader object
///
/// # Description
/// [is_shader] returns true if `shader` is the name of a shader object previously created with
/// [create_shader] and not yet deleted with [delete_shader]. If `shader` is zero or a non-zero
/// value that is not the name of a shader object, or if an error occurs, [is_shader] returns false.
///
/// # Arguments
/// * `shader` - Specifies a potential shader object
///
/// # Notes
/// - No error is generated if shader is not a valid shader object name.
/// - A shader object marked for deletion with [delete_shader] but still attached to a program object
/// is still considered a shader object and [is_shader] will return true.
pub fn is_shader(shader: Shader) -> bool {
    (unsafe { gl::IsShader(shader.0) }) == gl::TRUE
}
