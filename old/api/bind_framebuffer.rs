use crate::*;
use gl::types::*;

/// bind a framebuffer to a framebuffer target
///
/// # Description
/// [bind_framebuffer] binds the framebuffer object with name `framebuffer` to the framebuffer target
/// specified by `target`. If a framebuffer object is bound to [Draw](FramebufferBindingTarget::Draw)
/// or [Read](FramebufferBindingTarget::Read), it becomes the target for rendering or readback
/// operations, respectively, until it is deleted or another framebuffer is bound to the corresponding
/// bind point. Calling [bind_framebuffer] with target set to [ReadDraw](FramebufferBindingTarget::ReadDraw)
/// binds `framebuffer` to both the read and draw framebuffer targets. `framebuffer` is the name of
/// a framebuffer object previously returned from a call to [gen_framebuffer], or None to break the
/// existing binding of a framebuffer object to target.
///
/// # Arguments
/// * `target` - Specifies the framebuffer target of the binding operation
/// * `framebuffer` - Specifies the name of the framebuffer object to bind
///
/// # Examples
/// ```no_run
/// # fn setup_framebuffer(framebuffer: rgl::Framebuffer) -> Result<(), rgl::Error> {
/// rgl::bind_framebuffer(rgl::FramebufferBindingTarget::ReadDraw, Some(framebuffer))?;
/// // ... setup logic ...
/// rgl::bind_framebuffer(rgl::FramebufferBindingTarget::ReadDraw, None)?;
/// # Ok(())
/// # }
/// ```
pub fn bind_framebuffer(
    target: FramebufferBindingTarget,
    framebuffer: Option<Framebuffer>,
) -> Result<(), Error> {
    let framebuffer = framebuffer.unwrap_or(Framebuffer(0));
    let target: GLenum = target.into();
    unsafe { gl::BindFramebuffer(target, framebuffer.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(Error::NonOpenGLFramebuffer(framebuffer)),
        error => Err(Error::Unreachable(error)),
    }
}
