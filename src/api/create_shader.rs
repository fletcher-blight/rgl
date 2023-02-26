use crate::*;
use gl::types::*;

/// Creates a shader object
///
/// # Description
/// [create_shader] creates an empty shader object and returns a non-zero value by which it can
/// be referenced. A shader object is used to maintain the source code strings that define a shader.
/// `shader_type` indicates the type of shader to be created.
///
/// This function returns `None` if an error occurs creating the shader object.
///
/// # Notes
/// - Like buffer and texture objects, the name space for shader objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space.
/// If the name space is shared across contexts, any attached objects and the data associated with
/// those attached objects are shared as well.
///
/// - Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
///
/// # Examples
/// ```no_run
/// let shader: rgl::Shader = rgl::create_shader(rgl::ShaderType::Vertex)
///     .expect("Vertex Shader Create Failed");
/// ```
///
/// For safety, shader must be generated from [create_shader]
/// ```compile_fail
/// let shader = rgl::Shader(42);
/// ```
pub fn create_shader(shader_type: ShaderType) -> Option<Shader> {
    let type_: GLuint = shader_type.into();
    let id = unsafe { gl::CreateShader(type_) };
    if id == 0 {
        None
    } else {
        Some(Shader(id))
    }
}
