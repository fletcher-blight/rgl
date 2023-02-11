use crate::*;

/// Deletes a program object
///
/// [delete_program] frees the memory and invalidates the name associated with the program object
/// specified by program. This command effectively undoes the effects of a call to [create_program].
///
/// If a program object is in use as part of current rendering state, it will be flagged for deletion,
/// but it will not be deleted until it is no longer part of current state for any rendering context.
/// If a program object to be deleted has shader objects attached to it, those shader objects will
/// be automatically detached but not deleted unless they have already been flagged for deletion by
/// a previous call to [delete_shader]. A value of 0 for program will be silently ignored.
///
/// To determine whether a program object has been flagged for deletion, call
/// [get_program_delete_status]\).
pub fn delete_program(program: Program) -> Result<(), Error> {
    unsafe { gl::DeleteProgram(program.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        error => Err(Error::Unreachable(error)),
    }
}
