use crate::*;

/// modify the rate at which generic vertex attributes advance during instanced rendering
///
/// # Description
/// [vertex_attribute_divisor] modifies the rate at which generic vertex attributes advance when
/// rendering multiple instances of primitives in a single draw call. If divisor is zero, the attribute
/// at slot `index` advances once per vertex. If divisor is non-zero, the attribute advances once
/// per divisor instances of the set(s) of vertices being rendered. An attribute is referred to as
/// instanced if its [VertexAttributeArrayDivisor] value is non-zero.
///
/// `index` must be less than the value of [MaxVertexAttributes].
///
/// # Arguments
/// * `index` - Specify the index of the generic vertex attribute
/// * `divisor` - Specify the number of instances that will pass between updates of the generic
/// attribute at slot `index`
pub fn vertex_attribute_divisor(index: u32, divisor: u32) -> Result<(), Error> {
    unsafe { gl::VertexAttribDivisor(index, divisor) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::OutOfBoundsVertexAttributeIndex(index)),
        error => Err(Error::Unreachable(error)),
    }
}
