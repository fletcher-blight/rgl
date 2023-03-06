use crate::*;
use gl::types::*;

/// generate a framebuffer object name
///
/// # Description
/// [gen_framebuffer] generates a framebuffer object name. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that the returned name was not in
/// use immediately before the call to [gen_framebuffer].
///
/// The framebuffer object name returned by a call to [gen_framebuffer] is not returned by subsequent
/// calls, unless they are first deleted with [delete_framebuffers].
///
/// The name returned are marked as used, for the purposes of [gen_framebuffer] only,
/// but they acquire state and type only when they are first bound.
///
/// # Examples
/// ```no_run
/// let framebuffer: rgl::Framebuffer = rgl::gen_framebuffer();
/// ```
///
/// For safety, framebuffer must be generated from [gen_framebuffer]
/// ```compile_fail
/// let framebuffer = rgl::Framebuffer(42);
/// ```
pub fn gen_framebuffer() -> Framebuffer {
    let mut framebuffer = Framebuffer(0);
    let framebuffers = &mut framebuffer.0 as *mut GLuint;
    unsafe { gl::GenFramebuffers(1, framebuffers) };
    framebuffer
}
