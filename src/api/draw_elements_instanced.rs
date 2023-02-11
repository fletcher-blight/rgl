use crate::*;
use gl::types::*;

/// draw multiple instances of a set of elements
///
/// [draw_elements_instanced] behaves identically to [draw_elements] except that `instance_count`
/// instances of the set of elements are executed and the value of the internal counter `instanceID`
/// advances for each iteration. `instanceID` is an internal 32-bit integer counter that may be
/// read by a vertex shader as `gl_InstanceID`.
///
/// [draw_elements_instanced] has the same effect as:
/// ```
/// # use rgl::*;
/// # fn draw_elements_instanced(
/// #     mode: RenderPrimitive,
/// #     count: u32,
/// #     indices_type: IndicesType,
/// #     offset: u32,
/// #     instance_count: u32) -> Result<(), Error> {
/// for instance_id in 0..instance_count {
///     draw_elements(mode, count, indices_type, offset)?;
/// }
/// #    Ok(())
/// # }
/// ```
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `count` - Specifies the number of elements to be rendered
/// * `indices_type` - Specifies the type of the values in the element array buffer
/// * `offset` - Specifies the starting offset from the enabled array
/// * `instance_count` - Specifies the number of instances of the specified range of indices to be rendered
///
/// # Compatability
/// - 3.1 or greater is required for [draw_elements_instanced]
/// - 3.2 or greater is required for: [LineStripAdjacency](RenderPrimitive::LineStripAdjacency),
/// [LinesAdjacency](RenderPrimitive::LinesAdjacency), [TriangleStripAdjacency](RenderPrimitive::TriangleStripAdjacency)
/// and [TrianglesAdjacency](RenderPrimitive::TrianglesAdjacency)
pub fn draw_elements_instanced(
    mode: RenderPrimitive,
    count: u32,
    indices_type: IndicesType,
    offset: u32,
    instance_count: u32,
) -> Result<(), Error> {
    let mode: GLenum = mode.into();
    let count = count as GLsizei;
    let type_: GLenum = indices_type.into();
    let indices = offset as *const std::os::raw::c_void;
    let instancecount = instance_count as GLsizei;
    unsafe { gl::DrawElementsInstanced(mode, count, type_, indices, instancecount) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        error => Err(Error::Unreachable(error)),
    }
}
