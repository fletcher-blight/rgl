use crate::*;
use gl::types::*;

/// render primitives from array data
///
/// # Description
/// [draw_arrays] specifies multiple geometric primitives with very few subroutine calls. Instead of
/// calling a GL procedure to pass each individual vertex, normal, texture coordinate, edge flag,
/// or color, you can prespecify separate arrays of vertices, normals, and colors and use them to
/// construct a sequence of primitives with a single call to [draw_arrays].
///
/// When [draw_arrays] is called, it uses `count` sequential elements from each enabled array to
/// construct a sequence of geometric primitives, beginning with element `first`. `mode` specifies
/// what kind of primitives are constructed and how the array elements construct those primitives.
///
/// Vertex attributes that are modified by [draw_arrays] have an unspecified value after
/// [draw_arrays] returns. Attributes that aren't modified remain well defined.
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `first` - Specifies the starting index in the enabled arrays
/// * `count` - Specifies the number of indices to be rendered
///
/// # Notes
/// - 3.2 or greater is required for: [LineStripAdjacency](RenderPrimitive::LineStripAdjacency),
/// [LinesAdjacency](RenderPrimitive::LinesAdjacency), [TriangleStripAdjacency](RenderPrimitive::TriangleStripAdjacency)
/// and [TrianglesAdjacency](RenderPrimitive::TrianglesAdjacency)
pub fn draw_arrays(mode: RenderPrimitive, first: u32, count: u32) -> Result<(), Error> {
    let mode: GLenum = mode.into();
    let first = first as GLint;
    let count = count as GLsizei;
    unsafe { gl::DrawArrays(mode, first, count) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        error => Err(Error::Unreachable(error)),
    }
}
