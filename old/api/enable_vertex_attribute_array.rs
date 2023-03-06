use crate::*;

/// Enable generic vertex attribute array
///
/// # Description
/// [enable_vertex_attribute_array] enable the generic vertex attribute array specified by `index`,
/// using currently bound vertex array object for the operation.
pub fn enable_vertex_attribute_array(index: u32) -> Result<(), Error> {
    unsafe { gl::EnableVertexAttribArray(index) };
    handle_attribute_array_error(index)
}

/// Disable generic vertex attribute array
///
/// # Description
/// [disable_vertex_attribute_array] disable the generic vertex attribute array specified by `index`.
/// using currently bound vertex array object for the operation.
pub fn disable_vertex_attribute_array(index: u32) -> Result<(), Error> {
    unsafe { gl::DisableVertexAttribArray(index) };
    handle_attribute_array_error(index)
}

fn handle_attribute_array_error(index: u32) -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(Error::NoVertexArrayBound),
        ErrorOpenGL::InvalidValue => Err(Error::OutOfBoundsVertexAttributeIndex(index)),
        error => Err(Error::Unreachable(error)),
    }
}

/// Enable generic vertex attribute array
///
/// # Description
/// [enable_vertex_array_attribute] enable the generic vertex attribute array specified by `index`,
/// updating state of the vertex array object with ID `array`.
pub fn enable_vertex_array_attribute(array: VertexArray, index: u32) -> Result<(), Error> {
    unsafe { gl::EnableVertexArrayAttrib(array.0, index) };
    handle_array_attribute_error(array, index)
}

/// Disable generic vertex attribute array
///
/// # Description
/// [disable_vertex_array_attribute] disable the generic vertex attribute array specified by `index`,
/// updating state of the vertex array object with ID `array`.
pub fn disable_vertex_array_attribute(array: VertexArray, index: u32) -> Result<(), Error> {
    unsafe { gl::DisableVertexArrayAttrib(array.0, index) };
    handle_array_attribute_error(array, index)
}

fn handle_array_attribute_error(array: VertexArray, index: u32) -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(Error::NonOpenGLVertexArray(array)),
        ErrorOpenGL::InvalidValue => Err(Error::OutOfBoundsVertexAttributeIndex(index)),
        error => Err(Error::Unreachable(error)),
    }
}
