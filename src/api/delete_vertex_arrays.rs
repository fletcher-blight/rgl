use crate::*;
use gl::types::*;

/// delete vertex array objects
///
/// # Description
/// [delete_vertex_arrays] deletes all vertex array objects whose names are stored in the array
/// addressed by `arrays`. Once a vertex array object is deleted it has no contents and its name is
/// again unused. If a vertex array object that is currently bound is deleted, the binding for that
/// object reverts to zero and the default vertex array becomes current. Unused names in `arrays`
/// are silently ignored, as is the value zero.
///
/// # Arguments
/// * `arrays` - Specifies an array of vertex array objects to be deleted
pub fn delete_vertex_arrays(arrays: &[VertexArray]) -> () {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_ptr() as *const GLuint;
    unsafe { gl::DeleteVertexArrays(n, arrays) }
}
