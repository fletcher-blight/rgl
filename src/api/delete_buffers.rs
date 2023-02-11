use crate::*;
use gl::types::*;

/// delete named buffer objects
///
/// [delete_buffers] deletes all buffer objects named by `buffers`. After a buffer object is deleted,
/// it has no contents, and its name is free for reuse (for example by [gen_buffers]). If a buffer
/// object that is currently bound is deleted, the binding reverts to 0 (the absence of any buffer object).
///
/// [delete_buffers] silently ignores 0's and names that do not correspond to existing buffer objects.
///
/// # Arguments
/// * `buffers` - Specifies an array of buffer objects to be deleted
pub fn delete_buffers(buffers: &[Buffer]) -> () {
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_ptr() as *const GLuint;
    unsafe { gl::DeleteBuffers(n, buffers) }
}
