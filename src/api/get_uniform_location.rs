use crate::*;
use gl::types::*;

/// Returns the location of a uniform variable
///
/// [get_uniform_location] returns [UniformLocation] that represents the location of a specific
/// uniform variable within a program object. `uniform_name` must be a null terminated string that
/// contains no white space. `uniform_name` must be an active uniform variable name in program that
/// is not a structure, an array of structures, or a subcomponent of a vector or a matrix.
/// This function errors with [UnknownUniformName](Error::UnknownUniformName)
/// if `name` does not correspond to an active uniform variable in program, if `name` starts with
/// the reserved prefix "gl_", or if `name` is associated with an atomic counter or a named uniform block.
///
/// Uniform variables that are structures or arrays of structures may be queried by calling
/// [get_uniform_location] for each field within the structure. The array element operator "\[\]"
/// and the structure field operator "." may be used in name in order to select elements within an
/// array or fields within a structure. The result of using these operators is not allowed to be
/// another structure, an array of structures, or a subcomponent of a vector or a matrix.
/// Except if the last part of name indicates a uniform variable array, the location of the first
/// element of an array can be retrieved by using the name of the array, or by using the
/// name appended by "\[0\]".
///
/// The actual locations assigned to uniform variables are not known until the program object is
/// linked successfully. After linking has occurred, the command [get_uniform_location] can be used
/// to obtain the location of a uniform variable. This location value can then be passed to
/// [uniform], to set the value of the uniform variable or to [get_uniform] in order to
/// query the current value of the uniform variable. After a program object has been linked successfully,
/// the index values for uniform variables remain fixed until the next link command occurs.
/// Uniform variable locations and values can only be queried after a link if the link was successful.
pub fn get_uniform_location(
    program: Program,
    uniform_name: &std::ffi::CStr,
) -> Result<UniformLocation, Error> {
    let name = uniform_name.as_ptr() as *const GLchar;
    let location = unsafe { gl::GetUniformLocation(program.0, name) };
    match internal_get_error() {
        ErrorOpenGL::NoError => {
            if location < 0 {
                Err(Error::UnknownUniformName(uniform_name.into()))
            } else {
                Ok(UniformLocation(location))
            }
        }
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(Error::NotAProgram(program))
            } else {
                Err(Error::UnlinkedProgram(program))
            }
        }
        error => Err(Error::Unreachable(error)),
    }
}
