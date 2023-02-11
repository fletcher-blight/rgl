use crate::*;

/// Compiles a shader object
///
/// [compile_shader] compiles the source code strings that have been stored in the shader object
/// specified by `shader`.
///
/// The compilation status will be stored as part of the shader object's state. This value will be
/// set to true if the shader was compiled without errors and is ready for use, and false otherwise.
/// It can be queried by calling [get_shader_compile_status].
///
/// Compilation of a shader can fail for a number of reasons as specified by the OpenGL Shading
/// Language Specification. Whether or not the compilation was successful, information about the
/// compilation can be obtained from the shader object's information log by calling [get_shader_info_log].
pub fn compile_shader(shader: Shader) -> Result<(), Error> {
    unsafe { gl::CompileShader(shader.id) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLShader(shader)),
        ErrorOpenGL::InvalidOperation => Err(Error::NotAShader(shader)),
        error => Err(Error::Unreachable(error)),
    }
}
