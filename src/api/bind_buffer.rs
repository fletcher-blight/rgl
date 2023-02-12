use crate::*;
use gl::types::*;

/// bind a named buffer object
///
/// # Description
///
/// Binds a buffer object to the specified buffer binding point. Calling [bind_buffer] with `buffer`
/// set to the name of a buffer object binds that buffer object name to the `target`. If no buffer
/// object with name `buffer` exists, one is created with that name. When a buffer object is bound
/// to a target, the previous binding for that target is automatically broken.
///
/// Buffer object names are unsigned integers. The value zero (None) is reserved, but there is no default
/// buffer object for each buffer object target. Instead, `buffer` set to None effectively unbinds
/// any buffer object previously bound, and restores client memory usage for that buffer object target
/// (if supported for that target). Buffer object names and the corresponding buffer object contents
/// are local to the shared object space of the current GL rendering context;
/// two rendering contexts share buffer object names only if they explicitly enable sharing between
/// contexts through the appropriate GL windows interfaces api.
///
/// [gen_buffers] must be used to generate a set of unused buffer object names.
///
/// The state of a buffer object immediately after it is first bound is an unmapped zero-sized memory
/// buffer with [ReadWrite] access and
/// [Static](BufferUsageFrequency::Static)[Draw](BufferUsageNature::Draw) usage.
///
/// While a non-zero buffer object name is bound, GL operations on the target to which it is bound
/// affect the bound buffer object, and queries of the target to which it is bound return state from
/// the bound buffer object. While buffer object name None is bound, as in the initial state,
/// attempts to modify or query state on the target to which it is bound generates an
/// [InvalidOperation](ErrorOpenGL::InvalidOperation) error.
///
/// A buffer object binding created with [bind_buffer] remains active until a different buffer
/// object name is bound to the same target, or until the bound buffer object is deleted with
/// [delete_buffers].
///
/// Once created, a named buffer object may be re-bound to any target as often as needed. However,
/// the GL implementation may make choices about how to optimize the storage of a buffer object based
/// on its initial binding target.
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `buffer` - Specifies the name of a buffer object
///
/// # Target Usage
/// - When Some(`buffer`) object is bound to the [Array](BufferBindingTarget::Array) target,
/// the vertex array pointer parameter is interpreted as an offset within the buffer object measured
/// in basic machine units.
/// - While a Some(`buffer`) object is bound to the [ElementArray](BufferBindingTarget::ElementArray)
/// target, the indices parameter of [draw_elements], [draw_elements_instanced], [draw_elements_base_vertex],
/// [draw_range_elements], [draw_range_elements_base_vertex], [multi_draw_elements], or
/// [multi_draw_elements_base_vertex] is interpreted as an offset within the buffer object measured
/// in basic machine units.
///
/// # Compatability
/// - 3.1 or greater is required for: [CopyRead](BufferBindingTarget::CopyRead),
/// [Uniform](BufferBindingTarget::Uniform), [Texture](BufferBindingTarget::Texture)
/// - 4.2 or greater is required for: [AtomicCounter](BufferBindingTarget::AtomicCounter)
/// - 4.3 or greater is required for: [DispatchIndirect](BufferBindingTarget::DispatchIndirect) and
/// [ShaderStorage](BufferBindingTarget::ShaderStorage)
/// - 4.4 or greater is required for: [Query](BufferBindingTarget::Query)
pub fn bind_buffer(target: BufferBindingTarget, buffer: Option<Buffer>) -> Result<(), Error> {
    let buffer = buffer.unwrap_or(Buffer(0));
    let target: GLenum = target.into();
    unsafe { gl::BindBuffer(target, buffer.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLBuffer(buffer)),
        error => Err(Error::Unreachable(error)),
    }
}
