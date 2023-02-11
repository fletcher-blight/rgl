use crate::*;

/// Determines if a name corresponds to a program object
///
/// [is_program] returns true if `program` is the name of a program object previously created with
/// [create_program] and not yet deleted with [delete_program]. If `program` is zero or a non-zero
/// value that is not the name of a program object, or if an error occurs, [is_program] returns false.
///
/// # Arguments
/// * `program` - Specifies a potential program object
///
/// # Notes
/// - No error is generated if program is not a valid program object name.
/// - A program object marked for deletion with [delete_program] but still in use as part of current
/// rendering state is still considered a program object and [is_program] will return true.
pub fn is_program(program: Program) -> bool {
    (unsafe { gl::IsProgram(program.id) }) == gl::TRUE
}
