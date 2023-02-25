//! define an array of generic vertex attribute data
//!
//! # Description
//! Specify the location and data format of the array of generic vertex attributes at index `index`
//! to use when rendering. `size` specifies the number of components per attribute.
//! `data_type` specifies the data type of each component, and `stride` specifies the byte stride
//! from one attribute to the next, allowing vertices and attributes to be packed into a single
//! array or stored in separate arrays.
//!
//! If `normalized` is set to `true`, it indicates that values stored in an integer format are to be
//! mapped to the range \[-1,1\] (for signed values) or \[0,1\] (for unsigned values) when they are
//! accessed and converted to floating point. Otherwise, values will be converted to floats directly
//! without normalization.
//!
//! If `offset` is not `0`, a non-zero named buffer object must be bound to the
//! [Array](BufferBindingTarget::Array) target (see [bind_buffer]), otherwise an error is generated.
//! `offset` is treated as a byte offset into the buffer object's data store. The buffer object binding
//! ([ArrayBinding]) is saved as generic vertex attribute array state [VertexAttributeArrayBinding]
//! for index `index`.
//!
//! When a generic vertex attribute array is specified, `size`, `data_type`, `normalized`, `stride`,
//! and `offset` are saved as vertex array state, in addition to the current vertex array buffer
//! object binding.
//!
//! To enable and disable a generic vertex attribute array, call [enable_vertex_attribute_array] and
//! [disable_vertex_attribute_array] with `index`. If enabled, the generic vertex attribute array is
//! used when [draw_arrays], [multi_draw_arrays], [draw_elements], [multi_draw_elements], or
//! [draw_range_elements] is called.

use crate::*;
use gl::types::*;

/// define an array of generic vertex attribute data for integer only use
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `size` - Specifies the number of components per generic vertex attribute
/// * `data_type` - Specifies the data type of each component in the array
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
pub fn vertex_attribute_integer_pointer(
    index: u32,
    size: VertexAttributeSize,
    data_type: VertexAttributeIntegerType,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = size.into();
    let type_: GLenum = data_type.into();
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribIPointer(index, size, type_, stride, pointer) };
    handle_error(index)
}

/// define an array of generic vertex attribute data for floating point only use
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `size` - Specifies the number of components per generic vertex attribute
/// * `data_type` - Specifies the data type of each component in the array
/// * `normalised` - Specifies whether fixed-point data values should be normalized (`true`) or
/// converted directly as fixed-point values (`false`) when they are accessed
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
pub fn vertex_attribute_float_pointer(
    index: u32,
    size: VertexAttributeSize,
    data_type: VertexAttributeFloatType,
    normalised: bool,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = size.into();
    let type_: GLenum = data_type.into();
    let normalised: GLboolean = if normalised { gl::TRUE } else { gl::FALSE };
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribPointer(index, size, type_, normalised, stride, pointer) };
    handle_error(index)
}

/// define an array of generic vertex attribute data for BGRA colour use
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `sized` - `true` specifies `size` to use [VertexAttributeSize::Quad], `false` signifies BGRA  
/// * `signed` - Specifies `data_type` to use signed (`true`) or unsigned (`false`) versions of the
///colour type `2_10_10_10_REV` (Alpha, Blue, Green, Red)
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
pub fn vertex_attribute_bgra_colour_pointer(
    index: u32,
    sized: bool,
    signed: bool,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = if sized { 4 } else { gl::BGRA as GLint };
    let type_: GLenum = if signed {
        gl::INT_2_10_10_10_REV
    } else {
        gl::UNSIGNED_INT_2_10_10_10_REV
    };
    let normalised: GLboolean = gl::TRUE;
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribPointer(index, size, type_, normalised, stride, pointer) };
    handle_error(index)
}

/// define an array of generic vertex attribute data for colour byte use
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `size` - Specifies the number of components per generic vertex attribute. `None` signifies to
/// use `BGRA` with the u8 colour data
/// * `normalised` - Specifies whether fixed-point data values should be normalized (`true`) or
/// converted directly as fixed-point values (`false`) when they are accessed
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
pub fn vertex_attribute_u8_colour_pointer(
    index: u32,
    size: Option<VertexAttributeSize>,
    normalised: bool,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = size.map_or(gl::BGRA as GLint, Into::into);
    let type_: GLenum = gl::UNSIGNED_BYTE;
    let normalised: GLboolean = if normalised { gl::TRUE } else { gl::FALSE };
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribPointer(index, size, type_, normalised, stride, pointer) };
    handle_error(index)
}

/// define an array of generic vertex attribute data for 10f,11f,11f (GBR) colour use
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `normalised` - Specifies whether fixed-point data values should be normalized (`true`) or
/// converted directly as fixed-point values (`false`) when they are accessed
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
///
/// # Compatability
/// - Requires 4.4 or greater
pub fn vertex_attribute_f32_colour_pointer(
    index: u32,
    normalised: bool,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = 3;
    let type_: GLenum = gl::UNSIGNED_INT_10F_11F_11F_REV;
    let normalised: GLboolean = if normalised { gl::TRUE } else { gl::FALSE };
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribPointer(index, size, type_, normalised, stride, pointer) };
    handle_error(index)
}

/// define an array of generic vertex attribute data for f64 usage only
///
/// # Description
/// see [vertex_attribute_pointer] for more details
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified
/// * `size` - Specifies the number of components per generic vertex attribute
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If `stride`
/// is 0, the generic vertex attributes are understood to be tightly packed in the array.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [Array](BufferBindingTarget::Array)
/// target
pub fn vertex_attribute_f64_pointer(
    index: u32,
    size: VertexAttributeSize,
    stride: u32,
    offset: u32,
) -> Result<(), Error> {
    let index = index as GLuint;
    let size: GLint = size.into();
    let stride = stride as GLsizei;
    let pointer = offset as *const std::os::raw::c_void;
    unsafe { gl::VertexAttribLPointer(index, size, gl::DOUBLE, stride, pointer) };
    handle_error(index)
}

fn handle_error(index: u32) -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::OutOfBoundsVertexAttributeIndex(index)),
        ErrorOpenGL::InvalidOperation => Err(Error::NoVertexArrayBound),
        error => Err(Error::Unreachable(error)),
    }
}
