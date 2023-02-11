use crate::*;
use gl::types::*;

/// Returns the information log for a shader object
///
/// [get_shader_info_log] returns the information log for the specified shader object.
/// The information log for a shader object is modified when the shader is compiled.
/// The string that is returned will be null terminated.
///
/// [get_shader_info_log] returns in `buffer` as much of the information log as it can. The number
/// of characters actually returned, excluding the null termination character, is specified by the
/// Ok return value. The size of the buffer required to store the returned information log can be
/// obtained by calling [get_shader_info_log_length].
///
/// The information log for a shader object is a string that may contain diagnostic messages,
/// warning messages, and other information about the last compile operation. When a shader object
/// is created, its information log will be a string of length 0.
///
/// # Arguments
/// * `shader` - Specifies the shader object whose information log is to be queried
/// * `buffer` - Specifies an array of characters that is used to return the information log
///
/// # Notes
/// The information log for a shader object is the OpenGL implementer's primary mechanism for conveying
/// information about the compilation process. Therefore, the information log can be helpful to
/// application developers during the development process, even when compilation is successful.
/// Application developers should not expect different OpenGL implementations to
/// produce identical information logs.
pub fn get_shader_info_log(shader: Shader, buffer: &mut [u8]) -> Result<u32, Error> {
    let buf_size = buffer.len() as GLsizei;
    let mut written_length: GLsizei = 0;
    let info_log = buffer.as_mut_ptr() as *mut GLchar;
    unsafe { gl::GetShaderInfoLog(shader.0, buf_size, &mut written_length, info_log) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(written_length as u32),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLShader(shader)),
        ErrorOpenGL::InvalidOperation => Err(Error::NotAShader(shader)),
        error => Err(Error::Unreachable(error)),
    }
}
