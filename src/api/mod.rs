mod active_texture;
mod attach_shader;
mod bind_buffer;
mod bind_framebuffer;
mod bind_texture;
mod bind_vertex_array;
pub mod buffer_data;
mod clear;
mod clear_colour;
mod compile_shader;
mod create_program;
mod create_shader;
mod delete_buffers;
mod delete_framebuffers;
mod delete_program;
mod delete_shader;
mod delete_textures;
mod delete_vertex_arrays;
mod detach_shader;
mod draw_arrays;
mod draw_arrays_instanced;
mod draw_elements;
mod draw_elements_instanced;
mod enable_vertex_attribute_array;
mod gen_buffer;
mod gen_framebuffer;
mod gen_texture;
mod gen_vertex_array;
mod get_error;
mod get_program;
mod get_program_info_log;
mod get_shader;
mod get_shader_info_log;
mod get_uniform_location;
mod is_buffer;
mod is_program;
mod is_shader;
mod link_program;
mod shader_source;
pub mod texture_parameter;
pub mod uniform;
mod use_program;
mod vertex_attribute_divisor;
pub mod vertex_attribute_pointer;

#[doc(inline)]
pub use active_texture::*;
#[doc(inline)]
pub use attach_shader::*;
#[doc(inline)]
pub use bind_buffer::*;
#[doc(inline)]
pub use bind_framebuffer::*;
#[doc(inline)]
pub use bind_texture::*;
#[doc(inline)]
pub use bind_vertex_array::*;
pub use buffer_data::*;
#[doc(inline)]
pub use clear::*;
#[doc(inline)]
pub use clear_colour::*;
#[doc(inline)]
pub use compile_shader::*;
#[doc(inline)]
pub use create_program::*;
#[doc(inline)]
pub use create_shader::*;
#[doc(inline)]
pub use delete_buffers::*;
#[doc(inline)]
pub use delete_framebuffers::*;
#[doc(inline)]
pub use delete_program::*;
#[doc(inline)]
pub use delete_shader::*;
#[doc(inline)]
pub use delete_textures::*;
#[doc(inline)]
pub use delete_vertex_arrays::*;
#[doc(inline)]
pub use detach_shader::*;
#[doc(inline)]
pub use draw_arrays::*;
#[doc(inline)]
pub use draw_arrays_instanced::*;
#[doc(inline)]
pub use draw_elements::*;
#[doc(inline)]
pub use draw_elements_instanced::*;
#[doc(inline)]
pub use enable_vertex_attribute_array::*;
#[doc(inline)]
pub use gen_buffer::*;
#[doc(inline)]
pub use gen_framebuffer::*;
#[doc(inline)]
pub use gen_texture::*;
#[doc(inline)]
pub use gen_vertex_array::*;
#[doc(inline)]
pub use get_error::*;
#[doc(inline)]
pub use get_program::*;
#[doc(inline)]
pub use get_program_info_log::*;
#[doc(inline)]
pub use get_shader::*;
#[doc(inline)]
pub use get_shader_info_log::*;
#[doc(inline)]
pub use get_uniform_location::*;
#[doc(inline)]
pub use is_buffer::*;
#[doc(inline)]
pub use is_program::*;
#[doc(inline)]
pub use is_shader::*;
#[doc(inline)]
pub use link_program::*;
#[doc(inline)]
pub use shader_source::*;
pub use texture_parameter::*;
pub use uniform::*;
#[doc(inline)]
pub use use_program::*;
#[doc(inline)]
pub use vertex_attribute_divisor::*;
pub use vertex_attribute_pointer::*;
