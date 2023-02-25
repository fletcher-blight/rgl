//! creates and initializes a buffer object's data store
//!
//! # Description
//! [buffer_data()], [buffer_data_reserved], [named_buffer_data] and [named_buffer_data_reserved]
//! create a new data store for a buffer object. In case of [buffer_data()] and [buffer_data_reserved],
//! the buffer object currently bound to `target` is used. For [named_buffer_data] and
//! [named_buffer_data_reserved], a buffer object associated with ID specified by the caller in
//! `buffer` will be used instead.
//!
//! While creating the new storage, any pre-existing data store is deleted. The new data store is
//! created with the specified `size` or `data.len()` in bytes and `usage`. If not using `*_reserved`,
//! the data store is initialized from `data`. In its initial state, the new data store is not mapped,
//! it has a None mapped pointer, and its mapped access is [ReadWrite].
//!
//! # Arguments
//! * `target` - Specifies the target to which the buffer object is bound for [buffer_data()] and
//! [buffer_data_reserved]
//! * `buffer` - Specifies the name of the buffer object for [named_buffer_data] and
//! [named_buffer_data_reserved] function
//! * `size` - Specifies the size in bytes of the buffer object's new data store
//! * `data` - Specifies data that will be copied into the data store for initialization
//! * `usage` - Specifies the expected usage pattern of the data store
//!
//! # Notes
//! If using `*_reserved`, a data store of the specified `size` is still created, but its contents
//! remain uninitialized and thus undefined.
//!
//! Clients must align data elements consistently with the requirements of the client platform, with
//! an additional base-level requirement that an offset within a buffer to a datum comprising N bytes
//! be a multiple of N.
//!
//! # Compatability
//! - 4.2 or greater is required for: [AtomicCounter](BufferBindingTarget::AtomicCounter)
//! - 4.3 or greater is required for: [DispatchIndirect](BufferBindingTarget::DispatchIndirect) and
//! [ShaderStorage](BufferBindingTarget::ShaderStorage)
//! - 4.4 or greater is required for: [Query](BufferBindingTarget::Query)

use crate::*;
use gl::types::*;

/// creates and initializes `target`'s buffer object's data store with `data`
///
/// See [mod@buffer_data] for more details
pub fn buffer_data<Data>(
    target: BufferBindingTarget,
    data: &[Data],
    usage: BufferUsage,
) -> Result<(), Error> {
    let target: GLenum = target.into();
    let size = (data.len() * std::mem::size_of::<Data>()) as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;
    let usage: GLenum = usage.into();
    unsafe { gl::BufferData(target, size, data, usage) };
    handle_error()
}

/// reserves `size` bytes of uninitialised data for a `target`'s buffer object's data store
///
/// See [mod@buffer_data] for more details
pub fn buffer_data_reserved(
    target: BufferBindingTarget,
    size: u32,
    usage: BufferUsage,
) -> Result<(), Error> {
    let target: GLenum = target.into();
    let size = size as GLsizeiptr;
    let data = std::ptr::null() as *const std::os::raw::c_void;
    let usage: GLenum = usage.into();
    unsafe { gl::BufferData(target, size, data, usage) };
    handle_error()
}

/// creates and initializes `buffer`'s buffer object's data store with `data`
///
/// See [mod@buffer_data] for more details
pub fn named_buffer_data<Data>(
    buffer: Buffer,
    data: &[Data],
    usage: BufferUsage,
) -> Result<(), Error> {
    let size = (data.len() * std::mem::size_of::<Data>()) as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;
    let usage: GLenum = usage.into();
    unsafe { gl::NamedBufferData(buffer.0, size, data, usage) };
    handle_error()
}

/// reserves `size` bytes of uninitialised data for a `buffer`'s buffer object's data store
///
/// See [mod@buffer_data] for more details
pub fn named_buffer_data_reserved(
    buffer: Buffer,
    size: u32,
    usage: BufferUsage,
) -> Result<(), Error> {
    let size = size as GLsizeiptr;
    let data = std::ptr::null() as *const std::os::raw::c_void;
    let usage: GLenum = usage.into();
    unsafe { gl::NamedBufferData(buffer.0, size, data, usage) };
    handle_error()
}

fn handle_error() -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        ErrorOpenGL::OutOfMemory => Err(Error::OpenGL(ErrorOpenGL::OutOfMemory)),
        error => Err(Error::Unreachable(error)),
    }
}
