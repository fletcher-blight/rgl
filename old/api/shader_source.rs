use crate::*;
use gl::types::*;

/// Replaces the source code in a shader object
///
/// # Description
/// [shader_source] sets the source code in `shader` to the source code in `source`. Any source code
/// previously stored in the shader object is completely replaced. The source code strings are not
/// scanned or parsed at this time; they are simply copied into the specified shader object.
///
/// # Notes
/// OpenGL copies the shader source code strings when [shader_source] is called, so an application
/// may free its copy of the source code strings immediately after the function returns.
///
/// # Examples
/// ```no_run
/// # fn set_source(shader: rgl::Shader) -> Result<(), rgl::Error> {
/// let source = r#"
/// # version 330
/// out vec4 colour;
/// void main() {
///     colour = vec4(1.0, 0.0, 0.0, 1.0);
/// }
/// "#;
/// rgl::shader_source(shader, source.as_bytes())?;
/// # Ok(())
/// # }
/// ```
pub fn shader_source(shader: Shader, source: &[u8]) -> Result<(), Error> {
    let source_length = source.len() as GLint;
    let source_ptr = source.as_ptr();
    let sources = std::slice::from_ref(&source_ptr);

    let count: GLsizei = 1;
    let string = sources.as_ptr() as *const *const GLchar;
    let length = &source_length as *const GLint;
    unsafe { gl::ShaderSource(shader.0, count, string, length) };

    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLShader(shader)),
        ErrorOpenGL::InvalidOperation => Err(Error::NotAShader(shader)),
        error => Err(Error::Unreachable(error)),
    }
}
