use crate::*;
use gl::types::*;

/// render primitives from array data
///
/// # Description
/// [draw_elements] specifies multiple geometric primitives with very few subroutine calls.
/// Instead of calling a GL function to pass each individual vertex, normal, texture coordinate,
/// edge flag, or color, you can pre-specify separate arrays of vertices, normals, and so on,
/// and use them to construct a sequence of primitives with a single call to [draw_elements].
///
/// When [draw_elements] is called, it uses `count` sequential elements from an enabled array,
/// starting at `offset` to construct a sequence of geometric primitives. `mode` specifies what kind
/// of primitives are constructed and how the array elements construct these primitives.
/// If more than one array is enabled, each is used.
///
/// Vertex attributes that are modified by [draw_elements] have an unspecified value after
/// [draw_elements] returns. Attributes that aren't modified maintain their previous values.
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `count` - Specifies the number of elements to be rendered
/// * `indices_type` - Specifies the type of the values in the element array buffer
/// * `offset` - Specifies the starting offset from the enabled array
///
/// # Compatability
/// - 3.2 or greater is required for: [LineStripAdjacency](RenderPrimitive::LineStripAdjacency),
/// [LinesAdjacency](RenderPrimitive::LinesAdjacency), [TriangleStripAdjacency](RenderPrimitive::TriangleStripAdjacency)
/// and [TrianglesAdjacency](RenderPrimitive::TrianglesAdjacency)
///
/// # Examples
/// ```no_run
/// # fn draw(num_indices: u32, offset: u32) -> Result<(), rgl::Error> {
/// rgl::draw_elements(
///     rgl::RenderPrimitive::Triangles,
///     num_indices,
///     rgl::IndicesType::U32,
///     offset
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn draw_elements(
    mode: RenderPrimitive,
    count: u32,
    indices_type: IndicesType,
    offset: u32,
) -> Result<(), Error> {
    let mode: GLenum = mode.into();
    let count = count as GLsizei;
    let type_: GLenum = indices_type.into();
    let indices = offset as *const std::os::raw::c_void;
    unsafe { gl::DrawElements(mode, count, type_, indices) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        error => Err(Error::Unreachable(error)),
    }
}
