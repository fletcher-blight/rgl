//! # Framebuffer Objects
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Framebuffer_Objects>
//!
//! # Description
//! The core OpenGL API reference for functions that create, manage, bind, and destroy
//! [framebuffer objects](https://www.khronos.org/opengl/wiki/Framebuffer_Object), as well as
//! [renderbuffer objects](https://www.khronos.org/opengl/wiki/Renderbuffer_Object).

use crate::prelude::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Framebuffer(pub u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FramebufferBindingTarget {
    Read,
    Draw,
    ReadDraw,
}

impl From<FramebufferBindingTarget> for GLenum {
    fn from(value: FramebufferBindingTarget) -> Self {
        match value {
            FramebufferBindingTarget::Read => gl::READ_FRAMEBUFFER,
            FramebufferBindingTarget::Draw => gl::DRAW_FRAMEBUFFER,
            FramebufferBindingTarget::ReadDraw => gl::FRAMEBUFFER,
        }
    }
}

/// # Bind a framebuffer to a framebuffer target
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindFramebuffer.xhtml>
///
/// # Arguments
/// * `target` - Specifies the framebuffer target of the binding operation.
/// * `framebuffer` - Specifies the name of the framebuffer object to bind.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_framebuffer(FramebufferBindingTarget::ReadDraw, Framebuffer(42));
/// ```
///
/// # Description
/// [bind_framebuffer] binds the framebuffer object with name `framebuffer` to the framebuffer
/// target specified by `target`. If a framebuffer object is bound to
/// [FramebufferBindingTarget::Draw] or [FramebufferBindingTarget::Read], it becomes the target for
/// rendering or readback operations, respectively, until it is deleted or another framebuffer is
/// bound to the corresponding bind point. Calling [bind_framebuffer] with target set to
/// [FramebufferBindingTarget::ReadDraw] binds framebuffer to both the read and draw framebuffer
/// targets. `framebuffer` is the name of a framebuffer object previously returned from a call to
/// [gen_framebuffers], or zero to break the existing binding of a framebuffer object to target.
///
/// # Errors
/// * [Error::InvalidOperation] - if `framebuffer` is not zero of the name of a
/// previously returned from a call to [gen_framebuffers].
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_framebuffer] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_framebuffers]
/// * [framebuffer_renderbuffer]
/// * [framebuffer_texture]
/// * [framebuffer_texture_1d]
/// * [framebuffer_texture_2d]
/// * [framebuffer_texture_3d]
/// * [framebuffer_texture_layer]
/// * [delete_framebuffers]
/// * [is_framebuffer]
pub fn bind_framebuffer(target: FramebufferBindingTarget, framebuffer: Framebuffer) {
    let target = GLenum::from(target);
    let framebuffer = framebuffer.0;
    unsafe { gl::BindFramebuffer(target, framebuffer) }
}

/// # Delete framebuffer objects
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDeleteFramebuffers.xhtml>
///
/// # Arguments
/// * `framebuffers` - A slice containing framebuffer objects to be deleted.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// delete_framebuffers(&[Framebuffer(42)]);
/// ```
///
/// # Description
/// [delete_framebuffers] deletes all framebuffer objects whose names are stored in the slice
/// `framebuffers`. The name zero is reserved by the GL and is silently ignored, should it occur in
/// `framebuffers`, as are other unused names. Once a framebuffer object is deleted, its name is
/// again unused and it has no attachments. If a framebuffer that is currently bound to one or more
/// of the targets [FramebufferBindingTarget::Draw] or [FramebufferBindingTarget::Read] is deleted,
/// it is as though [bind_framebuffer] had been executed with the corresponding target and
/// framebuffer zero.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [delete_framebuffers] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_framebuffers]
/// * [bind_framebuffer]
/// * [check_framebuffer_status]
pub fn delete_framebuffers(framebuffers: &[Framebuffer]) {
    let n = framebuffers.len() as GLsizei;
    let framebuffers = framebuffers.as_ptr() as *const GLuint;
    unsafe { gl::DeleteFramebuffers(n, framebuffers) }
}
