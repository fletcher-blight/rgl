use crate::*;
use gl::types::*;

/// generate a vertex array object names
///
/// # Description
/// [gen_vertex_array] generates a vertex array object names. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that the returned name was not in
/// use immediately before the call to [gen_vertex_array].
///
/// The vertex array object name returned by a call to [gen_vertex_array] is not returned by subsequent
/// calls, unless they are first deleted with [delete_vertex_arrays].
///
/// The name returned is marked as used, for the purposes of [gen_vertex_array] only,
/// but will acquire state and type only when first bound.
///
/// # Examples
/// ```no_run
/// let vertex_array: rgl::VertexArray = rgl::gen_vertex_array();
/// ```
///
/// For safety, vertex_array must be generated from [gen_vertex_array]
/// ```compile_fail
/// let vertex_array = rgl::VertexArray(42);
/// ```
pub fn gen_vertex_array() -> VertexArray {
    let mut vertex_array = VertexArray(0);
    let vertex_arrays = &mut vertex_array.0 as *mut GLuint;
    unsafe { gl::GenVertexArrays(1, vertex_arrays) };
    vertex_array
}
