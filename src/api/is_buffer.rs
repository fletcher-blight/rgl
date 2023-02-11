use crate::*;

/// determine if a name corresponds to a buffer object
///
/// [is_buffer] returns true if `buffer` is currently the name of a buffer object. If `buffer` is zero,
/// or is a non-zero value that is not currently the name of a buffer object, or if an error occurs,
/// [is_buffer] returns false.
///
/// A name returned by [gen_buffers], but not yet associated with a buffer object by calling
/// [bind_buffer], is not the name of a buffer object.
///
/// # Arguments
/// * `buffer` - Specifies a value that may be the name of a buffer object
pub fn is_buffer(buffer: Buffer) -> bool {
    (unsafe { gl::IsBuffer(buffer.id) }) == gl::TRUE
}
