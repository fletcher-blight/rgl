use crate::*;
use gl::types::*;

/// Returns the information log for a program object
///
/// [get_program_info_log] returns the information log for the specified program object.
/// The information log for a program object is modified when the program object is linked or validated.
/// The string that is returned will be null terminated.
///
/// [get_program_info_log] returns in `buffer` as much of the information log as it can. The number
/// of characters actually returned, excluding the null termination character, is specified by the
/// Ok return value. The size of the buffer required to store the returned information log can be
/// obtained by calling [get_program_info_log_length]\).
///
/// The information log for a program object is either an empty string, or a string containing
/// information about the last link operation, or a string containing information about the last
/// validation operation. It may contain diagnostic messages, warning messages, and other information.
/// When a program object is created, its information log will be a string of length 0.
///
/// # Arguments
/// * `program` - Specifies the program object whose information log is to be queried
/// * `buffer` - Specifies an array of characters that is used to return the information log
///
/// # Notes
/// The information log for a program object is the OpenGL implementer's primary mechanism for
/// conveying information about linking and validating. Therefore, the information log can be helpful
/// to application developers during the development process, even when these operations are successful.
/// Application developers should not expect different OpenGL implementations to
/// produce identical information logs.
pub fn get_program_info_log(program: Program, buffer: &mut [u8]) -> Result<u32, Error> {
    let buf_size = buffer.len() as GLsizei;
    let mut written_length: GLsizei = 0;
    let info_log = buffer.as_mut_ptr() as *mut GLchar;
    unsafe { gl::GetProgramInfoLog(program.0, buf_size, &mut written_length, info_log) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(written_length as u32),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        ErrorOpenGL::InvalidOperation => Err(Error::NotAProgram(program)),
        error => Err(Error::Unreachable(error)),
    }
}
