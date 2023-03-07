//! # Vertex Rendering
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Vertex_Rendering>
//!
//! # Description
//! The core OpenGL API reference for functions that
//! [render](https://www.khronos.org/opengl/wiki/Vertex_Rendering)
//! [vertex arrays](https://www.khronos.org/opengl/wiki/Vertex_Specification) or change non-VAO
//! state used to rendering of vertex data.

use crate::*;
use gl::types::*;

pub enum DrawIndexType {
    U8,
    U16,
    U32,
}

impl From<DrawIndexType> for u32 {
    fn from(value: DrawIndexType) -> Self {
        match value {
            DrawIndexType::U8 => gl::UNSIGNED_BYTE,
            DrawIndexType::U16 => gl::UNSIGNED_SHORT,
            DrawIndexType::U32 => gl::UNSIGNED_INT,
        }
    }
}

pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl From<DrawMode> for u32 {
    fn from(value: DrawMode) -> Self {
        match value {
            DrawMode::Points => gl::POINTS,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::LinesAdjacency => gl::LINES_ADJACENCY,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            DrawMode::Patches => gl::PATCHES,
        }
    }
}

/// # Render primitives from array data
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml>
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render.
/// * `first` - Specifies the starting index in the enabled arrays.
/// * `count` - Specifies the number of indices to be rendered.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// draw_arrays(DrawMode::Triangles, 0, 36);
/// ```
///
/// # Description
/// [draw_arrays] specifies multiple geometric primitives with very few subroutine calls. Instead of
/// calling a GL procedure to pass each individual vertex, normal, texture coordinate, edge flag, or
/// color, you can prespecify separate arrays of vertices, normals, and colors and use them to
/// construct a sequence of primitives with a single call to [draw_arrays].
///
/// When [draw_arrays] is called, it uses `count` sequential elements from each enabled array to
/// construct a sequence of geometric primitives, beginning with element `first`. `mode` specifies
/// what kind of primitives are constructed and how the array elements construct those primitives.
///
/// Vertex attributes that are modified by [draw_arrays] have an unspecified value after
/// [draw_arrays] returns. Attributes that aren't modified remain well defined.
///
/// # Compatability
/// * 3.2 - [DrawMode::LineStripAdjacency], [DrawMode::LinesAdjacency],
/// [DrawMode::TriangleStripAdjacency] and [DrawMode::Triangles]
///
/// # Errors
/// * [Error::InvalidOperation] - if a non-zero buffer object name is bound to an enabled array and
/// the buffer object's data store is currently mapped.
/// * [Error::InvalidOperation] - if a geometry shader is active and mode is incompatible with the
/// input primitive type of the geometry shader in the currently installed program object.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [draw_arrays] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [draw_arrays_instanced]
/// * [draw_elements]
/// * [draw_range_elements]
pub fn draw_arrays(mode: DrawMode, first: u64, count: u64) {
    let mode = GLenum::from(mode);
    let first = first as GLint;
    let count = count as GLsizei;

    // SAFE: synchronous integer copy
    unsafe { gl::DrawArrays(mode, first, count) }
}

/// # Render primitives from array data
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDrawElements.xhtml>
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render.
/// * `count` - Specifies the number of elements to be rendered.
/// * `index_type` - Specifies the type of the values in the indices buffer
/// * `offset` - Specifies an offset to the location where the indices are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// draw_elements(DrawMode::Triangles, 36, DrawIndexType::U32, 0);
/// ```
///
/// # Description
/// [draw_elements] specifies multiple geometric primitives with very few subroutine calls. Instead
/// of calling a GL function to pass each individual vertex, normal, texture coordinate, edge flag,
/// or color, you can prespecify separate arrays of vertices, normals, and so on, and use them to
/// construct a sequence of primitives with a single call to [draw_elements].
///
/// When [draw_elements] is called, it uses `count` sequential elements from an enabled array,
/// starting at `offset` to construct a sequence of geometric primitives. `mode` specifies what kind
/// of primitives are constructed and how the array elements construct these primitives. If more
/// than one array is enabled, each is used.
///
/// Vertex attributes that are modified by [draw_elements] have an unspecified value after
/// [draw_elements] returns. Attributes that aren't modified maintain their previous values.
///
/// # Compatability
/// * 3.2 - [DrawMode::LineStripAdjacency], [DrawMode::LinesAdjacency],
/// [DrawMode::TriangleStripAdjacency] and [DrawMode::Triangles]
///
/// # Errors
/// * [Error::InvalidOperation] - if a geometry shader is active and mode is incompatible with the
/// input primitive type of the geometry shader in the currently installed program object.
/// * [Error::InvalidOperation] - if a non-zero buffer object name is bound to an enabled array or
/// the element array and the buffer object's data store is currently mapped.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [draw_elements] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [draw_arrays]
/// * [draw_elements_instanced]
/// * [draw_elements_base_vertex]
/// * [draw_range_elements]
pub fn draw_elements(mode: DrawMode, count: u64, index_type: DrawIndexType, offset: u64) {
    let mode = GLenum::from(mode);
    let count = count as GLsizei;
    let type_ = GLenum::from(index_type);
    let indices = offset as *const std::os::raw::c_void;

    unsafe { gl::DrawElements(mode, count, type_, indices) }
}
