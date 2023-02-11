use crate::*;

/// Installs a program object as part of current rendering state
///
/// [use_program] installs the program object specified by program as part of current rendering state.
/// One or more executables are created in a program object by successfully attaching shader objects
/// to it with [attach_shader], successfully compiling the shader objects with [compile_shader],
/// and successfully linking the program object with [link_program].
///
/// A program object will contain an executable that will run on the vertex processor if it contains
/// one or more shader objects of type [Vertex](ShaderType::Vertex) that have been successfully
/// compiled and linked. A program object will contain an executable that will run on the geometry
/// processor if it contains one or more shader objects of type [Geometry](ShaderType::Geometry) that
/// have been successfully compiled and linked. Similarly, a program object will contain an executable
/// that will run on the fragment processor if it contains one or more shader objects of type
/// [Fragment](ShaderType::Fragment) that have been successfully compiled and linked.
///
/// While a program object is in use, applications are free to modify attached shader objects,
/// compile attached shader objects, attach additional shader objects, and detach or delete
/// shader objects. None of these operations will affect the executables that are part of the
/// current state. However, relinking the program object that is currently in use will install the
/// program object as part of the current rendering state if the link operation was successful
/// (see [link_program]). If the program object currently in use is relinked unsuccessfully,
/// [get_program_link_status]\) will be false, but the executables and associated
/// state will remain part of the current state until a subsequent call to [use_program] removes
/// it from use. After it is removed from use, it cannot be made part of current state until it
/// has been successfully relinked.
///
/// If program is zero, then the current rendering state refers to an invalid program object and
/// the results of shader execution are undefined. However, this is not an error.
///
/// If program does not contain shader objects of type [Fragment](ShaderType::Fragment), an executable
/// will be installed on the vertex, and possibly geometry processors, but the results of
/// fragment shader execution will be undefined.
///
/// # Notes
/// - Like buffer and texture objects, the name space for program objects may be shared across a
/// set of contexts, as long as the server sides of the contexts share the same address space.
/// If the name space is shared across contexts, any attached objects and the data associated
/// with those attached objects are shared as well.
/// - Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
pub fn use_program(program: Program) -> Result<(), Error> {
    unsafe { gl::UseProgram(program.id) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(Error::NotAProgram(program))
            } else if let Ok(_) = get_program_transform_feedback_buffer_mode(program) {
                Err(Error::TransportFeedbackModeActive(program))
            } else {
                Err(Error::ProgramCannotBeUsed(program))
            }
        }
        error => Err(Error::Unreachable(error)),
    }
}
