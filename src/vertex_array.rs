//! # Vertex Arrays
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Vertex_Arrays>
//!
//! # Description
//! The core OpenGL API reference for functions that create and destroy
//! [vertex array objects](https://www.khronos.org/opengl/wiki/Vertex_Array_Object), as well as
//! modify the state within them. They use
//! [buffer objects as the source for vertex and index data](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object).
//! These are a key component in
//! [sending vertex data to the GPU](https://www.khronos.org/opengl/wiki/Vertex_Specification).

use crate::prelude::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct VertexArray(pub u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VertexAttribSize {
    Single,
    Double,
    Triple,
    Quad,
    BGRA,
}

impl From<VertexAttribSize> for i32 {
    fn from(value: VertexAttribSize) -> Self {
        match value {
            VertexAttribSize::Single => 1,
            VertexAttribSize::Double => 2,
            VertexAttribSize::Triple => 3,
            VertexAttribSize::Quad => 4,
            VertexAttribSize::BGRA => gl::BGRA as i32,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VertexAttribIntegerType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
}

impl From<VertexAttribIntegerType> for GLenum {
    fn from(value: VertexAttribIntegerType) -> Self {
        match value {
            VertexAttribIntegerType::I8 => gl::BYTE,
            VertexAttribIntegerType::U8 => gl::UNSIGNED_BYTE,
            VertexAttribIntegerType::I16 => gl::SHORT,
            VertexAttribIntegerType::U16 => gl::UNSIGNED_SHORT,
            VertexAttribIntegerType::I32 => gl::INT,
            VertexAttribIntegerType::U32 => gl::UNSIGNED_SHORT,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VertexAttribFloatType {
    Integer(VertexAttribIntegerType),
    F16,
    F32,
    F64,
    Fixed,
    I32a2b10g10r10,
    U32a2b10g10r10,
    U32b10fg11fr11f,
}

impl From<VertexAttribFloatType> for GLenum {
    fn from(value: VertexAttribFloatType) -> Self {
        match value {
            VertexAttribFloatType::Integer(ty) => GLenum::from(ty),
            VertexAttribFloatType::F16 => gl::HALF_FLOAT,
            VertexAttribFloatType::F32 => gl::FLOAT,
            VertexAttribFloatType::F64 => gl::DOUBLE,
            VertexAttribFloatType::Fixed => gl::FIXED,
            VertexAttribFloatType::I32a2b10g10r10 => gl::INT_2_10_10_10_REV,
            VertexAttribFloatType::U32a2b10g10r10 => gl::UNSIGNED_INT_2_10_10_10_REV,
            VertexAttribFloatType::U32b10fg11fr11f => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }
}

/// # Bind a vertex array object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml>
///
/// # Arguments
/// * `array` - Specifies the name of the vertex array to bind.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_vertex_array(VertexArray(0));
/// ```
///
/// # Description
/// [bind_vertex_array] binds the vertex array object with name `array`. `array` is the name of a
/// vertex array object previously returned from a call to [gen_vertex_arrays], or zero to break the
/// existing vertex array object binding.
///
/// If no vertex array object with name `array` exists, one is created when `array` is first bound.
/// If the bind is successful no change is made to the state of the vertex array object, and any
/// previous vertex array object binding is broken.
///
/// # Errors
/// * [Error::InvalidOperation] - if `array` is not zero or the name of a vertex array object
/// previously returned from a call to [gen_vertex_arrays].
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_vertex_array] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [delete_vertex_arrays]
/// * [enable_vertex_attrib_array]
/// * [gen_vertex_arrays]
/// * [is_vertex_array]
/// * [vertex_attrib_float_pointer]
pub fn bind_vertex_array(array: VertexArray) {
    let array = array.0;

    // SAFE: synchronous integer copy
    unsafe { gl::BindVertexArray(array) }
}

/// # Delete vertex array objects
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml>
///
/// # Arguments
/// * `arrays` - Specifies the slice arrays containing the names of the objects to be deleted
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// delete_vertex_arrays(&[VertexArray(42), VertexArray(7)]);
/// ```
///
/// # Description
/// [delete_vertex_arrays] deletes all vertex array objects whose names are stored in the slice
/// `arrays`. Once a vertex array object is deleted it has no contents and its name is again unused.
/// If a vertex array object that is currently bound is deleted, the binding for that object reverts
/// to zero and the default vertex array becomes current. Unused names in `arrays` are silently
/// ignored, as is the value zero.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [delete_vertex_arrays] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_vertex_arrays]
/// * [is_vertex_array]
/// * [bind_vertex_array]
pub fn delete_vertex_arrays(arrays: &[VertexArray]) {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_ptr() as *const u32;

    // SAFE: synchronous read of `array`, nothing retained
    unsafe { gl::DeleteVertexArrays(n, arrays) }
}

/// # Enable a generic vertex attribute array
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml>
///
/// # Arguments
/// `index` - Specifies the index of the generic vertex attribute to be enabled or disabled.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// enable_vertex_attrib_array(42);
/// disable_vertex_attrib_array(42);
/// enable_vertex_array_attrib(VertexArray(7), 42);
/// disable_vertex_array_attrib(VertexArray(7), 42);
/// ```
///
/// # Description
/// [enable_vertex_attrib_array] and [enable_vertex_array_attrib] enable the generic vertex
/// attribute array specified by `index`. [enable_vertex_attrib_array] uses currently bound vertex
/// array object for the operation, whereas [enable_vertex_array_attrib] updates state of the vertex
/// array object with ID `vaobj`.
///
/// [disable_vertex_attrib_array] and [disable_vertex_array_attrib] disable the generic vertex
/// attribute array specified by `index`. [disable_vertex_attrib_array] uses currently bound vertex
/// array object for the operation, whereas [disable_vertex_array_attrib] updates state of the
/// vertex array object with ID `vaobj`.
///
/// By default, all client-side capabilities are disabled, including all generic vertex attribute
/// arrays. If enabled, the values in the generic vertex attribute array will be accessed and used
/// for rendering when calls are made to vertex array commands such as [draw_arrays],
/// [draw_elements], [draw_range_elements], [multi_draw_elements], or [multi_draw_arrays].
///
/// # Errors
/// * [Error::InvalidOperation] - if no vertex array object is bound
/// * [Error::InvalidValue] - if `index` is greater than or equal to [get_max_vertex_attribs]
///
/// # Associated Gets
/// * [get_max_vertex_attribs]
/// * [is_vertex_attrib_enabled] using `index`
/// * [get_vertex_attrib_pointer] using `index`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [enable_vertex_attrib_array] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [disable_vertex_attrib_array] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [enable_vertex_array_attrib] | N | N | N | N | N | N | N | N | N | N | N | Y |
/// | [disable_vertex_array_attrib] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_attrib_location]
/// * [draw_arrays]
/// * [draw_elements]
/// * [draw_range_elements]
/// * [multi_draw_elements]
/// * [vertex_attrib]
/// * [vertex_attrib_float_pointer]
pub fn enable_vertex_attrib_array(index: u32) {
    // SAFE: synchronous integer copy
    unsafe { gl::EnableVertexAttribArray(index) }
}

/// # Disable a generic vertex attribute array
/// see [enable_vertex_attrib_array]
pub fn disable_vertex_attrib_array(index: u32) {
    // SAFE: synchronous integer copy
    unsafe { gl::DisableVertexAttribArray(index) }
}

/// # Enable a generic vertex attribute array
/// see [enable_vertex_attrib_array]
///
/// # Arguments
/// * `vaobj` - Specifies the name of the vertex array object
///
/// # Errors
/// * [Error::InvalidOperation] - if `vaobj` is not the name of an existing vertex array object.
pub fn enable_vertex_array_attrib(vaobj: VertexArray, index: u32) {
    let vaobj = vaobj.0;
    // SAFE: synchronous integer copy
    unsafe { gl::EnableVertexArrayAttrib(vaobj, index) }
}

/// # Disable a generic vertex attribute array
/// see [enable_vertex_attrib_array] and [enable_vertex_array_attrib]
pub fn disable_vertex_array_attrib(vaobj: VertexArray, index: u32) {
    let vaobj = vaobj.0;
    // SAFE: synchronous integer copy
    unsafe { gl::DisableVertexArrayAttrib(vaobj, index) }
}

/// # Generate vertex array object names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml>
///
/// # Arguments
/// * `arrays` - Specifies a mut slice in which the generated vertex array object names are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut vao = Default::default();
/// gen_vertex_arrays(std::slice::from_mut(&mut vao));
/// ```
///
/// # Description
/// [gen_vertex_arrays] fills all vertex array object names in `arrays`. There is no guarantee that
/// the names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_vertex_arrays].
///
/// Vertex array object names returned by a call to [gen_vertex_arrays] are not returned by
/// subsequent calls, unless they are first deleted with [delete_vertex_arrays].
///
/// The names returned in `arrays` are marked as used, for the purposes of [gen_vertex_arrays] only,
/// but they acquire state and type only when they are first bound.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [gen_vertex_arrays] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_vertex_array]
/// * [delete_vertex_arrays]
pub fn gen_vertex_arrays(arrays: &mut [VertexArray]) {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_mut_ptr() as *mut u32;

    // SAFE: synchronous write into `arrays`, nothing retained
    unsafe { gl::GenVertexArrays(n, arrays) }
}

/// # Determine if a name corresponds to a vertex array object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glIsVertexArray.xhtml>
///
/// # Arguments
/// * `array` - Specifies a value that may be the name of a vertex array object.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_vertex_array(VertexArray(42)));
/// ```
///
/// # Description
/// [is_vertex_array] returns true if `array` is currently the name of a vertex array object. If
/// `array` is zero, or if `array` is not the name of a vertex array object, or if an error occurs,
/// [is_vertex_array] returns false. If `array` is a name returned by [gen_vertex_arrays], by that
/// has not yet been bound through a call to [bind_vertex_array], then the name is not a vertex
/// array object and [is_vertex_array] returns false.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_vertex_array] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_vertex_arrays]
/// * [bind_vertex_array]
/// * [delete_vertex_arrays]
pub fn is_vertex_array(array: VertexArray) -> bool {
    let array = array.0;

    // SAFE: synchronous integer copy
    let val = unsafe { gl::IsVertexArray(array) };
    val == gl::TRUE
}

/// # Define an array of generic vertex attribute data
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml>
///
/// # Arguments
/// * `index` - Specifies the index of the generic vertex attribute to be modified.
/// * `size` - Specifies the number of components per generic vertex attribute.
/// * `ty` - Specifies the data type of each component in the array.
/// * `normalised` - For [vertex_attrib_float_pointer], specifies whether fixed-point data values
/// should be normalized (true) or converted directly as fixed-point values (false) when they are
/// accessed.
/// * `stride` - Specifies the byte offset between consecutive generic vertex attributes. If stride
/// is 0, the generic vertex attributes are understood to be tightly packed in the array. The
/// initial value is 0.
/// * `offset` - Specifies a offset of the first component of the first generic vertex attribute in
/// the array in the data store of the buffer currently bound to the [BufferBindingTarget::Array]
/// target. The initial value is 0.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// vertex_attrib_float_pointer(
///     0,
///     VertexAttribSize::Triple,
///     VertexAttribFloatType::F32,
///     false,
///     (std::mem::size_of::<f32>() * 3) as u64,
///     0,
/// );
/// ```
///
/// # Description
/// [vertex_attrib_float_pointer], [vertex_attrib_integer_pointer] and [vertex_attrib_f64_pointer] specify
/// the location and data format of the array of generic vertex attributes at index `index` to use
/// when rendering. `size` specifies the number of components per attribute. `type` specifies the
/// data type of each component, and `stride` specifies the byte stride from one attribute to the
/// next, allowing vertices and attributes to be packed into a single array or stored in separate
/// arrays.
///
/// For [vertex_attrib_float_pointer], if normalized is set to true, it indicates that values stored in an
/// integer format are to be mapped to the range [-1,1] (for signed values) or [0,1] (for unsigned
/// values) when they are accessed and converted to floating point. Otherwise, values will be
/// converted to floats directly without normalization.
///
/// For [vertex_attrib_integer_pointer], values are always left as integer values.
///
/// [vertex_attrib_f64_pointer] specifies state for a generic vertex attribute array associated with
/// a shader attribute variable declared with 64-bit double precision components. `index`, `size`,
/// and stride behave as described for [vertex_attrib_float_pointer] and [vertex_attrib_integer_pointer].
///
/// `offset` is treated as a byte offset into the buffer object's data store. The buffer object
/// binding (GL_ARRAY_BUFFER_BINDING ?) is saved as generic vertex attribute array state
/// (GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING?) for index `index`.
///
/// When a generic vertex attribute array is specified, size, type, normalized, stride, and `offset`
/// are saved as vertex array state, in addition to the current vertex array buffer object binding.
///
/// To enable and disable a generic vertex attribute array, call [enable_vertex_attrib_array] and
/// [disable_vertex_attrib_array] with index. If enabled, the generic vertex attribute array is used
/// when [draw_arrays], [multi_draw_arrays], [draw_elements], [multi_draw_elements], or
/// [draw_range_elements] is called.
///
/// Each generic vertex attribute array is initially disabled and isn't accessed when
/// [draw_elements], [draw_range_elements], [draw_arrays], [multi_draw_arrays], or
/// [multi_draw_elements] is called.
///
/// # Compatability
/// * 4.4 - [VertexAttribFloatType::U32b10fg11fr11f]
///
/// # Errors
/// * [Error::InvalidValue] - if `index` is greater than or equal to [get_max_vertex_attribs].
/// * [Error::InvalidOperation] - if `size` is [VertexAttribSize::BGRA] and `ty` is not
/// [VertexAttribIntegerType::U8], [VertexAttribFloatType::I32a2b10g10r10] or
/// [VertexAttribFloatType::U32a2b10g10r10].
/// * [Error::InvalidOperation] - if `ty` is [VertexAttribFloatType::I32a2b10g10r10] or
/// [VertexAttribFloatType::U32a2b10g10r10] and `size` is not [VertexAttribSize::Quad] or
/// [VertexAttribSize::BGRA].
/// * [Error::InvalidOperation] - if `ty` is [VertexAttribFloatType::U32b10fg11fr11f] and `size` is
/// not [VertexAttribSize::Triple].
/// * [Error::InvalidOperation] - if `size` is [VertexAttribSize::BGRA] and `normalised` is false.
/// * [Error::InvalidOperation] - if zero is bound to the [BufferBindingTarget::Array] buffer object
/// binding point.
///
/// # Associated Gets
/// * [get_max_vertex_attribs]
/// * [get_vertex_attrib_array_enabled]
/// * [get_vertex_attrib_array_size]
/// * [get_vertex_attrib_array_type]
/// * [get_vertex_attrib_array_normalised]
/// * [get_vertex_attrib_array_stride]
/// * [get_vertex_attrib_array_buffer_binding]
/// * [get_array_buffer_binding]
/// * [get_vertex_attrib_pointer]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [vertex_attrib_float_pointer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [vertex_attrib_integer_pointer] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [vertex_attrib_f64_pointer] | N | N | N | N | N | N | N | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_attrib_location]
/// * [bind_buffer]
/// * [disable_vertex_attrib_array]
/// * [draw_arrays]
/// * [draw_elements]
/// * [draw_range_elements]
/// * [enable_vertex_attrib_array]
/// * [multi_draw_arrays]
/// * [multi_draw_elements]
/// * [vertex_attrib]
pub mod vertex_attrib_pointer {
    use crate::prelude::*;
    use gl::types::*;

    /// # Define an array of generic vertex attribute data
    /// see [vertex_attrib_pointer]
    pub fn vertex_attrib_float_pointer(
        index: u32,
        size: VertexAttribSize,
        ty: VertexAttribFloatType,
        normalised: bool,
        stride: u64,
        offset: u64,
    ) {
        let size = GLint::from(size);
        let type_ = GLenum::from(ty);
        let normalized = GLboolean::from(normalised);
        let stride = stride as GLsizei;
        let pointer = offset as *const std::os::raw::c_void;

        // SAFE: synchronous integer copy
        unsafe { gl::VertexAttribPointer(index, size, type_, normalized, stride, pointer) }
    }

    /// # Define an array of generic vertex attribute data
    /// see [vertex_attrib_pointer]
    pub fn vertex_attrib_integer_pointer(
        index: u32,
        size: VertexAttribSize,
        ty: VertexAttribFloatType,
        stride: u64,
        offset: u64,
    ) {
        let size = GLint::from(size);
        let type_ = GLenum::from(ty);
        let stride = stride as GLsizei;
        let pointer = offset as *const std::os::raw::c_void;

        // SAFE: synchronous integer copy
        unsafe { gl::VertexAttribIPointer(index, size, type_, stride, pointer) }
    }

    /// # Define an array of generic vertex attribute data
    /// see [vertex_attrib_pointer]
    pub fn vertex_attrib_f64_pointer(index: u32, size: VertexAttribSize, stride: u64, offset: u64) {
        let size = GLint::from(size);
        let stride = stride as GLsizei;
        let pointer = offset as *const std::os::raw::c_void;

        // SAFE: synchronous integer copy
        unsafe { gl::VertexAttribLPointer(index, size, gl::DOUBLE, stride, pointer) }
    }
}
pub use vertex_attrib_pointer::*;

/// # Modify the rate at which generic vertex attributes advance during instanced rendering
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glVertexAttribDivisor.xhtml>
///
/// # Arguments
/// * `index` - Specify the index of the generic vertex attribute.
/// * `divisor` - Specify the number of instances that will pass between updates of the generic
/// attribute at slot `index`.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// vertex_attrib_divisor(3, 1);
/// ```
///
/// # Description
/// [vertex_attrib_divisor] modifies the rate at which generic vertex attributes advance when
/// rendering multiple instances of primitives in a single draw call. If `divisor` is zero, the
/// attribute at slot `index` advances once per vertex. If `divisor` is non-zero, the attribute
/// advances once per `divisor` instances of the set(s) of vertices being rendered. An attribute is
/// referred to as instanced if its [get_vertex_attrib_divisor] value is non-zero.
///
/// # Compatability
/// 3.3 - [vertex_attrib_divisor]
///
/// # Errors
/// * [Error::InvalidValue] - if `index` is greater than or equal to [get_max_vertex_attribs]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [vertex_attrib_divisor] | N | N | N | N | N | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [vertex_attrib_float_pointer]
/// * [enable_vertex_attrib_array]
/// * [disable_vertex_attrib_array]
pub fn vertex_attrib_divisor(index: u32, divisor: u32) {
    unsafe { gl::VertexAttribDivisor(index, divisor) }
}
