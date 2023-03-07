//! # Shader Program Creation
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Shader_Program_Creation>
//!
//! # Description
//! The core OpenGL API reference for functions that create
//! [GLSL](https://www.khronos.org/opengl/wiki/OpenGL_Shading_Language)
//! [shader and program objects](https://www.khronos.org/opengl/wiki/GLSL_Object), as well as
//! [Program Pipeline Objects](https://www.khronos.org/opengl/wiki/Shader_Compilation#Program_pipelines).
//! These functions either directly create shader objects or affect how shaders and programs are
//! compiled or linked together.

use crate::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Program(u32);

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Shader(u32);

/// # Attaches a shader object to a program object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glAttachShader.xhtml>
///
/// # Arguments
/// * `program` - Specifies the program object to which a shader object will be attached.
/// * `shader` - Specifies the shader object that is to be attached.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// attach_shader(Program(42), Shader(7));
/// ```
///
/// # Description
/// In order to create a complete shader program, there must be a way to specify the list of things
/// that will be linked together. Program objects provide this mechanism. Shaders that are to be
/// linked together in a program object must first be attached to that program object.
/// [attach_shader] attaches the shader object specified by `shader` to the program object specified
/// by `program`. This indicates that `shader` will be included in link operations that will be
/// performed on `program`.
///
/// All operations that can be performed on a shader object are valid whether or not the shader
/// object is attached to a program object. It is permissible to attach a shader object to a program
/// object before source code has been loaded into the shader object or before the shader object has
/// been compiled. It is permissible to attach multiple shader objects of the same type because
/// each may contain a portion of the complete shader. It is also permissible to attach a shader
/// object to more than one program object. If a shader object is deleted while it is attached to a
/// program object, it will be flagged for deletion, and deletion will not occur until
/// [detach_shader] is called to detach it from all program objects to which it is attached.
///
/// # Errors
/// * [Error::InvalidValue] - if either `program` or `shader` is not a value generated by OpenGL.
/// * [Error::InvalidOperation] - if `program` is not a program object
/// * [Error::InvalidOperation] - if `shader` is not a shader object
/// * [Error::InvalidOperation] - if `shader` is already attached to `program`
///
/// # Associated Gets
/// * [get_attached_shaders]
/// * [get_shader_info_log]
/// * [get_shader_source]
/// * [is_program]
/// * [is_shader]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [attach_shader] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [compile_shader]
/// * [create_shader]
/// * [delete_shader]
/// * [detach_shader]
/// * [link_program]
/// * [shader_source]
pub fn attach_shader(program: Program, shader: Shader) {
    let program = program.0;
    let shader = shader.0;

    // SAFE: synchronous integer copy
    unsafe { gl::AttachShader(program, shader) }
}

/// # Compiles a shader object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glCompileShader.xhtml>
///
/// # Arguments
/// * `shader` - Specifies the shader object to be compiled.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// compile_shader(Shader(42))
/// ```
///
/// # Description
/// [compile_shader] compiles the source code strings that have been stored in the shader object
/// specified by `shader`.
///
/// The compilation status will be stored as part of the shader object's state. This value will be
/// set to true if the shader was compiled without errors and is ready for use, and false otherwise.
/// It can be queried by calling [get_shader_compile_status].
///
/// Compilation of a shader can fail for a number of reasons as specified by the OpenGL Shading
/// Language Specification. Whether or not the compilation was successful, information about the
/// compilation can be obtained from the shader object's information log by calling
/// [get_shader_info_log].
///
/// # Errors
/// * [Error::InvalidValue] - if `shader` is not a value generated by OpenGL.
/// * [Error::InvalidOperation] - if `shader` is not a shader object.
///
/// # Associated Gets
/// * [get_shader_info_log]
/// * [get_shader_compile_status]
/// * [is_shader]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [compile_shader] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [create_shader]
/// * [link_program]
/// * [shader_source]
pub fn compile_shader(shader: Shader) {
    let shader = shader.0;
    unsafe { gl::CompileShader(shader) }
}

/// # Creates a program object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml>
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let program: Program = create_program();
/// assert_ne!(program, Program(0));
/// ```
///
/// # Description
/// [create_program] creates an empty program object and returns a non-zero value by which it can be
/// referenced. A program object is an object to which shader objects can be attached. This provides
/// a mechanism to specify the shader objects that will be linked to create a program. It also
/// provides a means for checking the compatibility of the shaders that will be used to create a
/// program (for instance, checking the compatibility between a vertex shader and a fragment
/// shader). When no longer needed as part of a program object, shader objects can be detached.
///
/// One or more executables are created in a program object by successfully attaching shader objects
/// to it with [attach_shader], successfully compiling the shader objects with [compile_shader], and
/// successfully linking the program object with [link_program]. These executables are made part of
/// current state when [use_program] is called. Program objects can be deleted by calling
/// [delete_program]. The memory associated with the program object will be deleted when it is no
/// longer part of current rendering state for any context.
///
/// Like buffer and texture objects, the name space for program objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space. If the
/// name space is shared across contexts, any attached objects and the data associated with those
/// attached objects are shared as well.
///
/// Applications are responsible for providing the synchronization across API calls when objects are
/// accessed from different execution threads.
///
/// # Errors
/// * This function returns Program(0) if an error occurs creating the program object.
///
/// # Associated Gets
/// * [get_current_program]
/// * [get_active_attrib] with a valid program object and the index of an active attribute variable
/// * [get_active_uniform] with a valid program object and the index of an active uniform variable
/// * [get_attached_shaders] with a valid program object
/// * [get_attrib_location] with a valid program object and the name of an attribute variable
/// *  all `get_program_*` variants
/// * [get_program_info_log] with a valid program object
/// * [get_uniform] with a valid program object and the location of a uniform variable
/// * [get_uniform_location] with a valid program object and the name of a uniform variable
/// * [is_program]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [create_program] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [attach_shader]
/// * [bind_attrib_location]
/// * [create_shader]
/// * [delete_program]
/// * [detach_shader]
/// * [link_program]
/// * all `uniform_*` variants
/// * [use_program]
/// * [validate_program]
pub fn create_program() -> Program {
    let val = unsafe { gl::CreateProgram() };
    Program(val)
}
