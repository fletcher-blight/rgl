use crate::*;
use gl::types::*;

pub fn get_shader_shader_type(shader: Shader) -> Result<ShaderType, Error> {
    get_shader_iv(shader, gl::SHADER_TYPE).map(|res| (res as GLenum).try_into())?
}

/// returns true if shader is currently flagged for deletion, and false otherwise.
pub fn get_shader_delete_status(shader: Shader) -> Result<bool, Error> {
    get_shader_iv(shader, gl::DELETE_STATUS).map(as_bool)?
}

/// returns true if the last compile operation on shader was successful, and false otherwise.
pub fn get_shader_compile_status(shader: Shader) -> Result<bool, Error> {
    get_shader_iv(shader, gl::COMPILE_STATUS).map(as_bool)?
}

/// returns the number of characters in the information log for shader including the null
/// termination character (i.e., the size of the character buffer required to store the
/// information log). If shader has no information log, a value of 0 is returned.
pub fn get_shader_info_log_length(shader: Shader) -> Result<u32, Error> {
    get_shader_iv(shader, gl::INFO_LOG_LENGTH).map(as_u32)
}

/// returns the length of the concatenation of the source strings that make up the shader source
/// for the shader, including the null termination character. (i.e., the size of the character
/// buffer required to store the shader source). If no source code exists, 0 is returned.
pub fn get_shader_source_length(shader: Shader) -> Result<u32, Error> {
    get_shader_iv(shader, gl::SHADER_SOURCE_LENGTH).map(as_u32)
}

fn get_shader_iv(shader: Shader, pname: GLenum) -> Result<GLint, Error> {
    let mut params: GLint = 0;
    unsafe { gl::GetShaderiv(shader.id, pname, &mut params) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(params.into()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLShader(shader)),
        ErrorOpenGL::InvalidOperation => Err(Error::NotAShader(shader)),
        error => Err(Error::Unreachable(error)),
    }
}

fn as_bool(i: GLint) -> Result<bool, Error> {
    match i as GLboolean {
        gl::TRUE => Ok(true),
        gl::FALSE => Ok(false),
        val => Err(Error::ConversionFailure(format!(
            "Invalid GLboolean value: {val}"
        ))),
    }
}

fn as_u32(i: GLint) -> u32 {
    i as u32
}
