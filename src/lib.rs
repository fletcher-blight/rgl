//! Rust OpenGL
//!
//! # Overview
//! This is library is intended to provide one-to-one usage of raw opengl for safe rust. See the
//! accompanying [learnopenrgl](https://github.com/fletcher-blight/learnopenrgl.git) for the classic
//! [learnopengl](https://learnopengl.com) rewritten in rust.
//!
//! This is not intended to make OpenGL any more abstract and safer to use, it is designed as a
//! stepping stone for further abstractions and levels of indirection. Allowing anyone learning
//! opengl to simply use this API instead of the unsafe [gl](https://docs.rs/gl/latest/gl/) crate.
//! With that mentality in mind, all the documentation and api specifications came directly from the
//! [khronos](https://www.khronos.org/opengl/wiki/Category:Core_API_Reference) Opengl 4 reference.
//!
//! Hopefully this can be a one-stop place for using rust with raw opengl with accompanying
//! reference documentation.
//!
//! # Examples
//!
//! ## Hello Triangle
//! ```no_run
//! use rgl::prelude as rgl; // include all rgl sub-modules at root level
//!
//! // Safe function-dependent enum usage
//! let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
//! // using `&str` where possible instead of any `&std::ffi::CStr`
//! rgl::shader_source(vertex_shader, r#"
//! #version 330 core
//! layout (location=0) in vec2 position;
//! void main() {
//!     gl_Position = vec4(position, 0.0, 1.0);
//! }
//! "#);
//! rgl::compile_shader(vertex_shader);
//! assert!(rgl::get_shader_compile_status(vertex_shader));
//!
//! let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
//! rgl::shader_source(fragment_shader, r#"
//! #version 330 core
//! out vec4 colour;
//! void main() {
//!     colour = vec4(1.0, 0.0, 0.0, 1.0);
//! }"#);
//! rgl::compile_shader(fragment_shader);
//! assert!(rgl::get_shader_compile_status(fragment_shader));
//!
//! let shader_program = rgl::create_program();
//! rgl::attach_shader(shader_program, vertex_shader);
//! rgl::attach_shader(shader_program, fragment_shader);
//! rgl::link_program(shader_program);
//! assert!(rgl::get_program_link_status(shader_program));
//!
//! rgl::detach_shader(shader_program, vertex_shader);
//! rgl::detach_shader(shader_program, fragment_shader);
//! rgl::delete_shader(vertex_shader);
//! rgl::delete_shader(fragment_shader);
//!
//! let mut vao = Default::default();
//! // Same (annoying out param style) API to maintain 100% feature compatability
//! rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));
//!
//! let mut vbo = Default::default();
//! rgl::gen_buffers(std::slice::from_mut(&mut vbo));
//!
//! rgl::bind_vertex_array(vao);
//! rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);
//! rgl::buffer_data(
//!     rgl::BufferBindingTarget::Array,
//!     &[[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]],
//!     rgl::BufferUsageFrequency::Static,
//!     rgl::BufferUsageNature::Draw,
//! );
//! rgl::enable_vertex_attrib_array(0);
//! rgl::vertex_attrib_float_pointer(
//!     0,
//!     rgl::VertexAttribSize::Double,
//!     rgl::VertexAttribFloatType::F32,
//!     false,
//!     (std::mem::size_of::<f32>() * 2) as u64,
//!     0,
//! );
//! rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer::default());
//!
//! // ... main render loop ...
//! loop {
//!     rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
//!     rgl::use_program(shader_program);
//!     rgl::bind_vertex_array(vao);
//!     rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 3);
//!     assert_eq!(rgl::get_error(), rgl::Error::NoError);
//! }
//! ```
//!
//! ## Error Handling
//! The above the minimum needed abstractions to count as safe rust, just enum conversions and
//! unsigned integer types where negatives are an error. However, there is a *more safe* opt-in
//! abstraction designed to handle errors in a more rust style:
//! ```no_run
//! use rgl::prelude as rgl;
//! fn my_gpu_work() -> Result<(), rgl::BufferError> {
//!     rgl::bind_buffer_checked(rgl::BufferBindingTarget::Array, rgl::Buffer(42))?;
//!     Ok(())
//! }
//! ```
//! every OpenGL function will have a `_checked` counter part, which will call
//! [get_error](rgl::prelude::get_error), and transform the vague
//! [opengl error enum](rgl::prelude::Error), with a module-specific error (e.g.
//! [BufferError](rgl::prelude::BufferError)) that is returned through the rust `Result` idiom to
//! accurately describe the per-function error that occurred.
//!
//! # TODO
//! * implement all remaining `_checked` functions. (Currently just some in [buffer](rgl::buffer))
//! * add the entire API reference
//! * off-screen rendering unit-tests, which could act as an opengl implementation test
//! * CI/CD automation
//! * github integration
//! * cleanup `FROM` implementations. Use privates so a public `u32::from(rgl::SomeEnum)` would fail
//! * merge comparison funcs? ([TextureCompareFunc], [DepthFunc], [StencilFunc])

pub mod buffer;
pub mod get;
pub mod masks;
pub mod post;
pub mod prelude;
pub mod shader_creation;
pub mod shader_query;
pub mod shader_state;
pub mod texture;
pub mod vertex_array;
pub mod vertex_render;

use gl::types::*;

pub use gl::load_with;

/// # Template Function Documentation
/// <http://kronos/docs>
///
/// # Arguments
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// ```
///
/// # Description
///
/// # Compatability
///
/// # Errors
///
/// # Associated Gets
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [my_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
#[allow(unused)]
struct Stub;
