use crate::*;
use gl::types::*;

/// specify the value used for depth buffer comparisons
///
/// # Description
/// [depth_func] specifies the function used to compare each incoming pixel depth value with the
/// depth value present in the depth buffer. The comparison is performed only if depth testing is
/// enabled. (See [enable] and [disable] of [Capability::DepthTest])
///
/// The initial value of func is [Less](DepthFunc::Less). Initially, depth testing is disabled. If
/// depth testing is disabled or if no depth buffer exists, it is as if the depth test always passes.
///
/// # Notes
/// Even if the depth buffer exists and the depth mask is non-zero, the depth buffer is not updated
/// if the depth test is disabled. In order to unconditionally write to the depth buffer, the depth
/// test should be enabled and set to [Always](DepthFunc::Always).
pub fn depth_func(func: DepthFunc) -> () {
    let func: GLenum = func.into();
    unsafe { gl::DepthFunc(func) }
}
