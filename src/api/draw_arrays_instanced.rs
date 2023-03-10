use crate::*;
use gl::types::*;

/// draw multiple instances of a range of elements
///
/// # Description
/// [draw_arrays_instanced] behaves identically to [draw_arrays] except that `instance_count` instances
/// of the range of elements are executed and the value of the internal counter `instanceID` advances
/// for each iteration. `instanceID` is an internal 32-bit integer counter that may be read by a
/// vertex shader as `gl_InstanceID`.
///
/// [draw_arrays_instanced] has the same effect as:
/// ```no_run
/// # fn draw_arrays_instanced(
/// #     mode: rgl::RenderPrimitive,
/// #     first: u32,
/// #     count: u32,
/// #     instance_count: u32) -> Result<(), rgl::Error> {
/// for instance_id in 0..instance_count {
///     rgl::draw_arrays(mode, first, count)?;
/// }
/// #    Ok(())
/// # }
/// ```
///
/// # Examples
/// ```no_run
/// # fn draw(start: u32, num_vertices: u32, num_instances: u32) -> Result<(), rgl::Error> {
/// rgl::draw_arrays_instanced(
///     rgl::RenderPrimitive::Triangles,
///     start,
///     num_vertices,
///     num_instances
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn draw_arrays_instanced(
    mode: RenderPrimitive,
    first: u32,
    count: u32,
    instance_count: u32,
) -> Result<(), Error> {
    let mode: GLenum = mode.into();
    let first = first as GLint;
    let count = count as GLsizei;
    let instancecount = instance_count as GLsizei;
    unsafe { gl::DrawArraysInstanced(mode, first, count, instancecount) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        error => Err(Error::Unreachable(error)),
    }
}
