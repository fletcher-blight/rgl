use crate::*;

/// bind a vertex array object
///
/// Binds the vertex array object with name `array`. `array` is the name of a vertex array object
/// previously returned from a call to [gen_vertex_arrays], or zero to break the existing
/// vertex array object binding.
///
/// If no vertex array object with name `array` exists, one is created when array is first bound.
/// If the bind is successful no change is made to the state of the vertex array object,
/// and any previous vertex array object binding is broken.
///
/// # Arguments
/// * `array` - Specifies the name of the vertex array to bind
pub fn bind_vertex_array(array: VertexArray) -> Result<(), Error> {
    unsafe { gl::BindVertexArray(array.id) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(Error::NonOpenGLVertexArray(array)),
        error => Err(Error::Unreachable(error)),
    }
}
