use crate::*;

/// Detaches a shader object from a program object to which it is attached
///
/// [detach_shader] detaches the shader object specified by `shader` from the program object specified
/// by `program`. This command can be used to undo the effect of the command [attach_shader].
///
/// If `shader` has already been flagged for deletion by a call to [delete_shader] and it is not
/// attached to any other program object, it will be deleted after it has been detached.
pub fn detach_shader(program: Program, shader: Shader) -> Result<(), Error> {
    unsafe { gl::DetachShader(program.0, shader.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => {
            if !is_shader(shader) {
                Err(Error::NonOpenGLShader(shader))
            } else if !is_program(program) {
                Err(Error::NonOpenGLProgram(program))
            } else {
                Err(Error::Unreachable(ErrorOpenGL::InvalidValue))
            }
        }
        ErrorOpenGL::InvalidOperation => {
            if !is_shader(shader) {
                Err(Error::NotAShader(shader))
            } else if !is_program(program) {
                Err(Error::NotAProgram(program))
            } else {
                Err(Error::ShaderAlreadyAttachedToProgram(program, shader))
            }
        }
        error => Err(Error::Unreachable(error)),
    }
}
