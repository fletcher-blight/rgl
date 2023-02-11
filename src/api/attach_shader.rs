use crate::*;

/// Attaches a shader object to a program object
///
/// In order to create a complete shader program, there must be a way to specify the list of things
/// that will be linked together. Program objects provide this mechanism. Shaders that are to be
/// linked together in a program object must first be attached to that program object. [attach_shader]
/// attaches the shader object specified by shader to the program object specified by program.
/// This indicates that `shader` will be included in link operations that will be performed on `program`.
///
/// All operations that can be performed on a shader object are valid whether or not the shader
/// object is attached to a program object. It is permissible to attach a shader object to a program
/// object before source code has been loaded into the shader object or before the shader object has
/// been compiled. It is permissible to attach multiple shader objects of the same type because each
/// may contain a portion of the complete shader. It is also permissible to attach a shader object
/// to more than one program object. If a shader object is deleted while it is attached to a program
/// object, it will be flagged for deletion, and deletion will not occur until [detach_shader] is
/// called to detach it from all program objects to which it is attached.
///
/// # Arguments
/// * `program` - Specifies the program object to which a shader object will be attached
/// * `shader` - Specifies the shader object that is to be attached
pub fn attach_shader(program: Program, shader: Shader) -> Result<(), Error> {
    unsafe { gl::AttachShader(program.id, shader.id) };
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
