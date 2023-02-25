use crate::*;
use gl::types::*;

/// generate vertex array object names
///
/// # Description
/// [gen_vertex_arrays] fills all vertex array object names in `arrays`. There is no guarantee that
/// the names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_vertex_arrays].
///
/// Vertex array object names returned by a call to [gen_vertex_arrays] are not returned by subsequent
/// calls, unless they are first deleted with [delete_vertex_arrays].
///
/// The names returned in `arrays` are marked as used, for the purposes of [gen_vertex_arrays] only,
/// but they acquire state and type only when they are first bound.
///
/// # Arguments
/// * `arrays` - Specifies an array in which the generated vertex array object names are stored
pub fn gen_vertex_arrays(arrays: &mut [VertexArray]) -> () {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenVertexArrays(n, arrays) }
}
