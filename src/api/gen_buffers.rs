use crate::*;
use gl::types::*;

/// generate buffer object names
///
/// # Description
/// [gen_buffers] fills all buffer object names in `buffers`. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that none of the returned names
/// was in use immediately before the call to [gen_buffers].
///
/// Buffer object names returned by a call to [gen_buffers] are not returned by subsequent calls,
/// unless they are first deleted with [delete_buffers].
///
/// No buffer objects are associated with the returned buffer object names until they are first
/// bound by calling [bind_buffer].
///
/// # Arguments
/// * `buffers` - Specifies an array in which the generated buffer object names are stored
pub fn gen_buffers(buffers: &mut [Buffer]) -> () {
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenBuffers(n, buffers) }
}
