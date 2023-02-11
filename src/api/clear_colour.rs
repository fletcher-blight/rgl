// use crate::*;

/// specify clear values for the color buffers
///
/// [clear_colour] specifies the `red`, `green`, `blue`, and `alpha` values used by [clear](crate::clear) to clear
/// the color buffers. Values specified by [clear_colour] are clamped to the range \[0,1\].
pub fn clear_colour(red: f32, green: f32, blue: f32, alpha: f32) -> () {
    unsafe { gl::ClearColor(red, green, blue, alpha) }
}
