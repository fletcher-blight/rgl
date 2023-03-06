//! # Buffer Objects
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Buffer_Objects>
//!
//! # Description
//! The core OpenGL API reference for functions that create, manage, and delete
//! [buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object). These functions are only
//! for getting data into and out of the buffers; the
//! [different uses of buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object#General_use)
//! are not covered by these functions.

use crate::*;
use gl::types::*;

#[derive(Debug, Copy, Clone)]
pub enum BufferError {
    Unexpected(Error),

    UnboundTarget(BufferBindingTarget),
    InvalidBuffer(Buffer),
    InvalidParameterValue(i64),
}

#[derive(Debug, Copy, Clone)]
pub enum BufferBindingTarget {
    Array,
    ElementArray,
}

impl From<BufferBindingTarget> for GLenum {
    fn from(target: BufferBindingTarget) -> Self {
        match target {
            BufferBindingTarget::Array => gl::ARRAY_BUFFER,
            BufferBindingTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

pub enum BufferAccessFrequency {
    Stream,
    Static,
    Dynamic,
}

pub enum BufferAccessNature {
    Draw,
    Read,
    Copy,
}

struct BufferUsage(BufferAccessFrequency, BufferAccessNature);
impl From<BufferUsage> for GLenum {
    fn from(BufferUsage(frequency, nature): BufferUsage) -> Self {
        match (frequency, nature) {
            (BufferAccessFrequency::Stream, BufferAccessNature::Draw) => gl::STREAM_DRAW,
            (BufferAccessFrequency::Stream, BufferAccessNature::Read) => gl::STREAM_READ,
            (BufferAccessFrequency::Stream, BufferAccessNature::Copy) => gl::STREAM_COPY,
            (BufferAccessFrequency::Static, BufferAccessNature::Draw) => gl::STATIC_DRAW,
            (BufferAccessFrequency::Static, BufferAccessNature::Read) => gl::STATIC_READ,
            (BufferAccessFrequency::Static, BufferAccessNature::Copy) => gl::STATIC_COPY,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Draw) => gl::DYNAMIC_DRAW,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Read) => gl::DYNAMIC_READ,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Copy) => gl::DYNAMIC_COPY,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Buffer(pub u32);

/// # Bind a named buffer object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml>
///
/// # Description
///
/// # Arguments
///
/// # Compatability
///
/// # Errors
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_buffer(BufferBindingTarget::Array, Buffer(0));
/// ```
pub fn bind_buffer(target: BufferBindingTarget, buffer: Buffer) {
    let target = GLenum::from(target);
    let buffer = buffer.0;

    // SAFE: integers copied by value
    unsafe { gl::BindBuffer(target, buffer) }
}

/// # Error handled [bind_buffer]
pub fn checked_bind_buffer(target: BufferBindingTarget, buffer: Buffer) -> Result<(), BufferError> {
    bind_buffer(target, buffer);
    match get_error() {
        Error::NoError => Ok(()),
        Error::InvalidValue => Err(BufferError::InvalidBuffer(buffer)),
        other => Err(BufferError::Unexpected(other)),
    }
}

/// # Creates and initializes a buffer object's data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBufferData.xhtml>
///
/// # Description
///
/// # Arguments
///
/// # Compatability
///
/// # Errors
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// buffer_data(BufferBindingTarget::Array, &[1, 2, 3], BufferAccessFrequency::Static, BufferAccessNature::Draw);
/// ```
pub fn buffer_data<DataType: Sized>(
    target: BufferBindingTarget,
    data: &[DataType],
    access_frequency: BufferAccessFrequency,
    access_nature: BufferAccessNature,
) {
    let target = GLenum::from(target);
    let size = data.len() as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;
    let usage = GLenum::from(BufferUsage(access_frequency, access_nature));

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::BufferData(target, size, data, usage) };
}

pub fn checked_buffer_data<DataType: Sized>(
    target: BufferBindingTarget,
    data: &[DataType],
    access_frequency: BufferAccessFrequency,
    access_nature: BufferAccessNature,
) -> Result<(), BufferError> {
    buffer_data(target, data, access_frequency, access_nature);
    match get_error() {
        Error::NoError => Ok(()),
        Error::InvalidEnum => checked_get_buffer_access(target).map(|_| ()),
        Error::InvalidOperation => Err(BufferError::UnboundTarget(target)),
        other => Err(BufferError::Unexpected(other)),
    }
}

fn get_buffer_parameter_i32(target: BufferBindingTarget, value: GLenum) -> i32 {
    let target = GLenum::from(target);
    let mut param: i32 = 0;
    // SAFE: `param` is an out-param and not retained
    unsafe { gl::GetBufferParameteriv(target, value, &mut param) };
    param
}

/// # Get Buffer Object Access Usage
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Description
///
/// # Arguments
///
/// # Compatability
///
/// # Errors
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(get_buffer_access(BufferBindingTarget::Array), (BufferAccessFrequency::Static, BufferAccessNature::Draw));
/// ```
pub fn get_buffer_access(
    target: BufferBindingTarget,
) -> Result<(BufferAccessFrequency, BufferAccessNature), BufferError> {
    let param = get_buffer_parameter_i32(target, gl::BUFFER_ACCESS);
    match param as u32 {
        gl::STREAM_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STREAM_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STREAM_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        other => Err(BufferError::InvalidParameterValue(other as i64)),
    }
}

pub fn checked_get_buffer_access(
    target: BufferBindingTarget,
) -> Result<(BufferAccessFrequency, BufferAccessNature), BufferError> {
    let buffer_access = get_buffer_access(target)?;
    match get_error() {
        Error::NoError => Ok(buffer_access),
        Error::InvalidOperation => Err(BufferError::UnboundTarget(target)),
        other => Err(BufferError::Unexpected(other)),
    }
}
