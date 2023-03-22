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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FramebufferStatus {
    Complete,
    Undefined,
    IncompleteAttachment,
    IncompleteMissingAttachment,
    IncompleteDrawBuffer,
    IncompleteReadBuffer,
    Unsupported,
    IncompleteMultisample,
    IncompleteLayerTargets,
}

impl TryFrom<GLenum> for FramebufferStatus {
    type Error = ();
    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::FRAMEBUFFER_COMPLETE => Ok(FramebufferStatus::Complete),
            gl::FRAMEBUFFER_UNDEFINED => Ok(FramebufferStatus::Undefined),
            gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Ok(FramebufferStatus::IncompleteAttachment),
            gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Ok(FramebufferStatus::IncompleteMissingAttachment)
            }
            gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Ok(FramebufferStatus::IncompleteDrawBuffer),
            gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Ok(FramebufferStatus::IncompleteReadBuffer),
            gl::FRAMEBUFFER_UNSUPPORTED => Ok(FramebufferStatus::Unsupported),
            gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Ok(FramebufferStatus::IncompleteMultisample),
            gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                Ok(FramebufferStatus::IncompleteLayerTargets)
            }
            _ => Err(()),
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

/// # Check the completeness status of a framebuffer
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glCheckFramebufferStatus.xhtml>
///
/// # Arguments
/// * `target` - Specify the target to which the framebuffer is bound.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let status: FramebufferStatus = check_framebuffer_status(FramebufferBindingTarget::ReadDraw).expect("Error Occurred");
/// assert_eq!(status, FramebufferStatus::Complete);
/// ```
///
/// # Description
/// [check_framebuffer_status] and [check_named_framebuffer_status] return return the completeness
/// status of a framebuffer object when treated as a read or draw framebuffer, depending on the
/// value of `target`. [FramebufferBindingTarget::ReadDraw] is equivalent to
/// [FramebufferBindingTarget::Draw]. The return value is [FramebufferStatus::Complete] if the
/// specified framebuffer is complete. Otherwise, the return value is determined as follows:
///
/// | [FramebufferStatus] | Cause |
/// |---------------------|-------|
/// | [FramebufferStatus::Undefined] | the specified framebuffer is the default read or draw framebuffer, but the default framebuffer does not exist |
/// | [FramebufferStatus::IncompleteAttachment] | any of the framebuffer attachment points are framebuffer incomplete |
/// | [FramebufferStatus::IncompleteMissingAttachment] | the framebuffer does not have at least one image attached to it |
/// | [FramebufferStatus::IncompleteDrawBuffer] | the value of [get_framebuffer_attachment_object_type] is [FramebufferAttachmentObjectType::None] for any color attachment point(s) |
/// | [FramebufferStatus::IncompleteReadBuffer] | [GL_READ_BUFFER] is not [GL_NONE] and the value of [get_framebuffer_attachment_object_type] is [FramebufferAttachmentObjectType::None] for the color attachment point named by [GL_READ_BUFFER] |
/// | [FramebufferStatus::Unsupported] | the combination of internal formats of the attached images violates an implementation-dependent set of restrictions |
/// | [FramebufferStatus::IncompleteMultisample] | the value of [GL_RENDERBUFFER_SAMPLES] is not the same for all attached renderbuffers; if the value of [GL_TEXTURE_SAMPLES] is the not same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of [GL_RENDERBUFFER_SAMPLES] does not match the value of [GL_TEXTURE_SAMPLES] |
/// | [FramebufferStatus::IncompleteMultisample] | also, if the value of [GL_TEXTURE_FIXED_SAMPLE_LOCATIONS] is not the same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of [GL_TEXTURE_FIXED_SAMPLE_LOCATIONS] is not GL_TRUE for all attached textures |
/// | [FramebufferStatus::IncompleteLayerTargets] | is returned if any framebuffer attachment is layered, and any populated attachment is not layered, or if all populated color attachments are not from textures of the same target |
///
/// Additionally, if an error occurs, `None` is returned.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [check_framebuffer_status] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_framebuffers]
/// * [delete_framebuffers]
/// * [bind_framebuffer]
pub fn check_framebuffer_status(target: FramebufferBindingTarget) -> Option<FramebufferStatus> {
    let target = GLenum::from(target);
    let status = unsafe { gl::CheckFramebufferStatus(target) };
    match FramebufferStatus::try_from(status) {
        Ok(status) => Some(status),
        Err(_) => None,
    }
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

/// # Generate framebuffer object names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenFramebuffers.xhtml>
///
/// # Arguments
/// * `framebuffers` - Specifies a slice in which the generated framebuffer object names are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut framebuffer = Framebuffer::default();
/// gen_framebuffers(std::slice::from_mut(&mut framebuffer));
///
/// let mut framebuffers = [Framebuffer::default(); 1024];
/// gen_framebuffers(&mut framebuffers);
/// ```
///
/// # Description
/// [gen_framebuffers] generates framebuffer object names in `framebuffers`. There is no guarantee
/// that the names form a contiguous set of integers; however, it is guaranteed that none of the
/// returned names was in use immediately before the call to [gen_framebuffers].
///
/// Framebuffer object names returned by a call to [gen_framebuffers] are not returned by subsequent
/// calls, unless they are first deleted with [delete_framebuffers].
///
/// The names returned in ids are marked as used, for the purposes of [gen_framebuffers] only, but
/// they acquire state and type only when they are first bound.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [gen_framebuffers] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_framebuffer]
/// * [delete_framebuffer]
pub fn gen_framebuffers(framebuffers: &mut [Framebuffer]) {
    let n = framebuffers.len() as GLsizei;
    let framebuffers = framebuffers.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenFramebuffers(n, framebuffers) }
}
