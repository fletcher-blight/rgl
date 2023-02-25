use crate::*;

/// Creates a program object
///
/// # Description
/// [create_program] creates an empty program object and returns a non-zero value by which it can
/// be referenced. A program object is an object to which shader objects can be attached.
/// This provides a mechanism to specify the shader objects that will be linked to create a program.
/// It also provides a means for checking the compatibility of the shaders that will be used to
/// create a program (for instance, checking the compatibility between a vertex shader
/// and a fragment shader). When no longer needed as part of a program object,
/// shader objects can be detached.
///
/// One or more executables are created in a program object by successfully attaching shader objects
/// to it with [attach_shader], successfully compiling the shader objects with [compile_shader],
/// and successfully linking the program object with [link_program]. These executables are made part
/// of current state when [use_program] is called. Program objects can be deleted by calling [delete_program].
/// The memory associated with the program object will be deleted when it is no longer part of
/// current rendering state for any context.
///
/// # Notes
/// Like buffer and texture objects, the name space for program objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space. If the
/// name space is shared across contexts, any attached objects and the data associated with those
/// attached objects are shared as well.
///
/// Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
///
/// # Errors
/// This function returns 0 if an error occurs creating the program object.
pub fn create_program() -> Program {
    let id = unsafe { gl::CreateProgram() };
    Program(id)
}
