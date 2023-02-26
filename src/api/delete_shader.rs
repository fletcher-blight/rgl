use crate::*;

/// Deletes a shader object
///
/// # Description
/// [delete_shader] frees the memory and invalidates the name associated with the shader object
/// specified by shader. This command effectively undoes the effects of a call to [create_shader].
///
/// If a shader object to be deleted is attached to a program object, it will be flagged for deletion,
/// but it will not be deleted until it is no longer attached to any program object, for any rendering
/// context (i.e., it must be detached from wherever it was attached before it will be deleted).
/// A value of 0 for shader will be silently ignored.
///
/// To determine whether an object has been flagged for deletion, call [get_shader_delete_status].
///
/// # Examples
/// ```no_run
/// # fn cleanup(shader: rgl::Shader) -> Result<(), rgl::Error> {
/// rgl::delete_shader(shader)?;
/// # Ok(())
/// # }
/// ```
pub fn delete_shader(shader: Shader) -> Result<(), Error> {
    unsafe { gl::DeleteShader(shader.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLShader(shader)),
        error => Err(Error::Unreachable(error)),
    }
}
