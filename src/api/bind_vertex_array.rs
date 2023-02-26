use crate::*;

/// bind a vertex array object
///
/// # Description
/// Binds the vertex array object with name `array`. `array` is the name of a vertex array object
/// previously returned from a call to [gen_vertex_array], or None to break the existing
/// vertex array object binding.
///
/// If no vertex array object with name `array` exists, one is created when array is first bound.
/// If the bind is successful no change is made to the state of the vertex array object,
/// and any previous vertex array object binding is broken.
///
/// # Arguments
/// * `array` - Specifies the name of the vertex array to bind
///
/// # Examples
/// ```no_run
/// # fn setup_vertex_array(vertex_array: rgl::VertexArray) -> Result<(), rgl::Error> {
/// rgl::bind_vertex_array(Some(vertex_array))?;
/// // ... setup logic ...
/// rgl::bind_vertex_array(None)?;
/// # Ok(())
/// # }
/// ```
pub fn bind_vertex_array(array: Option<VertexArray>) -> Result<(), Error> {
    let array = array.unwrap_or(VertexArray(0));
    unsafe { gl::BindVertexArray(array.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(Error::NonOpenGLVertexArray(array)),
        error => Err(Error::Unreachable(error)),
    }
}
