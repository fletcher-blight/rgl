use crate::*;
use gl::types::*;

/// delete framebuffer objects
///
/// [delete_framebuffers] deletes all framebuffer objects whose names are stored in the array
/// addressed by `frame_buffers`. The name zero is reserved by the GL and is silently ignored,
/// should it occur in `frame_buffers`, as are other unused names. Once a framebuffer object is deleted,
/// its name is again unused and it has no attachments. If a framebuffer that is currently bound to
/// one or more of the targets [Draw](FramebufferBindingTarget::Draw) or [Read](FramebufferBindingTarget::Read)
/// is deleted, it is as though [bind_framebuffer] had been executed with the corresponding target
/// and framebuffer zero.
///
/// # Arguments
/// * `frame_buffers` - Specifies an array of framebuffer objects to be deleted
pub fn delete_framebuffers(framebuffers: &[Framebuffer]) -> () {
    let n = framebuffers.len() as GLsizei;
    let framebuffers = framebuffers.as_ptr() as *const GLuint;
    unsafe { gl::DeleteFramebuffers(n, framebuffers) }
}
