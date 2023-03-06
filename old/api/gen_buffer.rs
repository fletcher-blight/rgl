use crate::*;
use gl::types::*;

/// generate a buffer object name
///
/// # Description
/// [gen_buffer] generates a buffer object name. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that the returned name was not in
/// use immediately before the call to [gen_buffer].
///
/// The buffer object name returned by a call to [gen_buffer] is not returned by subsequent calls,
/// unless they are first deleted with [delete_buffers].
///
/// No buffer objects are associated with the returned buffer object name until they are first
/// bound by calling [bind_buffer].
///
/// # Examples
/// ```no_run
/// let buffer: rgl::Buffer = rgl::gen_buffer();
/// ```
///
/// For safety, buffer must be generated from [gen_buffer]
/// ```compile_fail
/// let buffer = rgl::Buffer(42);
/// ```
pub fn gen_buffer() -> Buffer {
    let mut buffer = Buffer(0);
    let buffers = &mut buffer.0 as *mut GLuint;
    unsafe { gl::GenBuffers(1, buffers) };
    buffer
}
