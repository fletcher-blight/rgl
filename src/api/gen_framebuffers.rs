use crate::*;
use gl::types::*;

/// generate framebuffer object names
///
/// [gen_framebuffers] fills all framebuffer object names in `ids`. There is no guarantee that the
/// names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_framebuffers].
///
/// Framebuffer object names returned by a call to [gen_framebuffers] are not returned by subsequent
/// calls, unless they are first deleted with [delete_framebuffers].
///
/// The names returned in ids are marked as used, for the purposes of [gen_framebuffers] only,
/// but they acquire state and type only when they are first bound.
///
/// # Arguments
/// * `ids` - Specifies an array in which the generated framebuffer object names are storeda
pub fn gen_framebuffers(ids: &mut [Framebuffer]) -> () {
    let n = ids.len() as GLsizei;
    let ids = ids.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenFramebuffers(n, ids) }
}
