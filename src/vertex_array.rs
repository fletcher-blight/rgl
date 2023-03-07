//! # Vertex Arrays
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Vertex_Arrays>
//!
//! # Description
//! The core OpenGL API reference for functions that create and destroy
//! [vertex array objects](https://www.khronos.org/opengl/wiki/Vertex_Array_Object), as well as
//! modify the state within them. They use
//! [buffer objects as the source for vertex and index data](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object).
//! These are a key component in
//! [sending vertex data to the GPU](https://www.khronos.org/opengl/wiki/Vertex_Specification).

use crate::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct VertexArray(pub u32);

/// # Bind a vertex array object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml>
///
/// # Arguments
/// * `array` - Specifies the name of the vertex array to bind.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_vertex_array(VertexArray(0));
/// ```
///
/// # Description
/// [bind_vertex_array] binds the vertex array object with name `array`. `array` is the name of a
/// vertex array object previously returned from a call to [gen_vertex_arrays], or zero to break the
/// existing vertex array object binding.
///
/// If no vertex array object with name `array` exists, one is created when `array` is first bound.
/// If the bind is successful no change is made to the state of the vertex array object, and any
/// previous vertex array object binding is broken.
///
/// # Errors
/// * [Error::InvalidOperation] - if `array` is not zero or the name of a vertex array object
/// previously returned from a call to [gen_vertex_arrays].
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_vertex_array] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [delete_vertex_arrays]
/// * [enable_vertex_attrib_array]
/// * [gen_vertex_arrays]
/// * [is_vertex_array]
/// * [vertex_attrib_pointer]
pub fn bind_vertex_array(array: VertexArray) {
    let array = array.0;

    // SAFE: synchronous integer copy
    unsafe { gl::BindVertexArray(array) }
}

/// # Delete vertex array objects
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml>
///
/// # Arguments
/// * `arrays` - Specifies the slice arrays containing the names of the objects to be deleted
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// delete_vertex_arrays(&[VertexArray(42), VertexArray(7)]);
/// ```
///
/// # Description
/// [delete_vertex_arrays] deletes all vertex array objects whose names are stored in the slice
/// `arrays`. Once a vertex array object is deleted it has no contents and its name is again unused.
/// If a vertex array object that is currently bound is deleted, the binding for that object reverts
/// to zero and the default vertex array becomes current. Unused names in `arrays` are silently
/// ignored, as is the value zero.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [delete_vertex_arrays] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_vertex_arrays]
/// * [is_vertex_array]
/// * [bind_vertex_array]
pub fn delete_vertex_arrays(arrays: &[VertexArray]) {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_ptr() as *const u32;

    // SAFE: synchronous read of `array`, nothing retained
    unsafe { gl::DeleteVertexArrays(n, arrays) }
}
