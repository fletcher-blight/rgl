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

use crate::prelude::*;
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

/// # Enable a generic vertex attribute array
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml>
///
/// # Arguments
/// `index` - Specifies the index of the generic vertex attribute to be enabled or disabled.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// enable_vertex_attrib_array(42);
/// disable_vertex_attrib_array(42);
/// enable_vertex_array_attrib(VertexArray(7), 42);
/// disable_vertex_array_attrib(VertexArray(7), 42);
/// ```
///
/// # Description
/// [enable_vertex_attrib_array] and [enable_vertex_array_attrib] enable the generic vertex
/// attribute array specified by `index`. [enable_vertex_attrib_array] uses currently bound vertex
/// array object for the operation, whereas [enable_vertex_array_attrib] updates state of the vertex
/// array object with ID `vaobj`.
///
/// [disable_vertex_attrib_array] and [disable_vertex_array_attrib] disable the generic vertex
/// attribute array specified by `index`. [disable_vertex_attrib_array] uses currently bound vertex
/// array object for the operation, whereas [disable_vertex_array_attrib] updates state of the
/// vertex array object with ID `vaobj`.
///
/// By default, all client-side capabilities are disabled, including all generic vertex attribute
/// arrays. If enabled, the values in the generic vertex attribute array will be accessed and used
/// for rendering when calls are made to vertex array commands such as [draw_arrays],
/// [draw_elements], [draw_range_elements], [multi_draw_elements], or [multi_draw_arrays].
///
/// # Errors
/// * [Error::InvalidOperation] - if no vertex array object is bound
/// * [Error::InvalidValue] - if `index` is greater than or equal to [get_max_vertex_attribs]
///
/// # Associated Gets
/// * [get_max_vertex_attribs]
/// * [is_vertex_attrib_enabled] using `index`
/// * [get_vertex_attrib_pointer] using `index`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [enable_vertex_attrib_array] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [disable_vertex_attrib_array] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [enable_vertex_array_attrib] | N | N | N | N | N | N | N | N | N | N | N | Y |
/// | [disable_vertex_array_attrib] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_attrib_location]
/// * [draw_arrays]
/// * [draw_elements]
/// * [draw_range_elements]
/// * [multi_draw_elements]
/// * [vertex_attrib]
/// * [vertex_attrib_pointer]
pub fn enable_vertex_attrib_array(index: u32) {
    // SAFE: synchronous integer copy
    unsafe { gl::EnableVertexAttribArray(index) }
}

/// # Disable a generic vertex attribute array
/// see [enable_vertex_attrib_array]
pub fn disable_vertex_attrib_array(index: u32) {
    // SAFE: synchronous integer copy
    unsafe { gl::DisableVertexAttribArray(index) }
}

/// # Enable a generic vertex attribute array
/// see [enable_vertex_attrib_array]
///
/// # Arguments
/// * `vaobj` - Specifies the name of the vertex array object
///
/// # Errors
/// * [Error::InvalidOperation] - if `vaobj` is not the name of an existing vertex array object.
pub fn enable_vertex_array_attrib(vaobj: VertexArray, index: u32) {
    let vaobj = vaobj.0;
    // SAFE: synchronous integer copy
    unsafe { gl::EnableVertexArrayAttrib(vaobj, index) }
}

/// # Disable a generic vertex attribute array
/// see [enable_vertex_attrib_array] and [enable_vertex_array_attrib]
pub fn disable_vertex_array_attrib(vaobj: VertexArray, index: u32) {
    let vaobj = vaobj.0;
    // SAFE: synchronous integer copy
    unsafe { gl::DisableVertexArrayAttrib(vaobj, index) }
}

/// # Generate vertex array object names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml>
///
/// # Arguments
/// * `arrays` - Specifies a mut slice in which the generated vertex array object names are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut vao = Default::default();
/// gen_vertex_arrays(std::slice::from_mut(&mut vao));
/// ```
///
/// # Description
/// [gen_vertex_arrays] fills all vertex array object names in `arrays`. There is no guarantee that
/// the names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_vertex_arrays].
///
/// Vertex array object names returned by a call to [gen_vertex_arrays] are not returned by
/// subsequent calls, unless they are first deleted with [delete_vertex_arrays].
///
/// The names returned in `arrays` are marked as used, for the purposes of [gen_vertex_arrays] only,
/// but they acquire state and type only when they are first bound.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [gen_vertex_arrays] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_vertex_array]
/// * [delete_vertex_arrays]
pub fn gen_vertex_arrays(arrays: &mut [VertexArray]) {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_mut_ptr() as *mut u32;

    // SAFE: synchronous write into `arrays`, nothing retained
    unsafe { gl::GenVertexArrays(n, arrays) }
}

/// # Determine if a name corresponds to a vertex array object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glIsVertexArray.xhtml>
///
/// # Arguments
/// * `array` - Specifies a value that may be the name of a vertex array object.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_vertex_array(VertexArray(42)));
/// ```
///
/// # Description
/// [is_vertex_array] returns true if `array` is currently the name of a vertex array object. If
/// `array` is zero, or if `array` is not the name of a vertex array object, or if an error occurs,
/// [is_vertex_array] returns false. If `array` is a name returned by [gen_vertex_arrays], by that
/// has not yet been bound through a call to [bind_vertex_array], then the name is not a vertex
/// array object and [is_vertex_array] returns false.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_vertex_array] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_vertex_arrays]
/// * [bind_vertex_array]
/// * [delete_vertex_arrays]
pub fn is_vertex_array(array: VertexArray) -> bool {
    let array = array.0;
    let val = unsafe { gl::IsVertexArray(array) };
    val == gl::TRUE
}
