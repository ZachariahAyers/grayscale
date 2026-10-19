//! WebAssembly bindings. Kernels live in [`kernel`] and stay free of JavaScript types.

pub mod kernel;

use wasm_bindgen::prelude::*;

/// Returns a grayscale copy of 8-bit RGBA pixels, as read from `ImageData.data`.
///
/// # Errors
///
/// Throws a JavaScript `Error` when the length is not a multiple of four.
#[wasm_bindgen]
pub fn grayscale(rgba: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut pixels = rgba.to_vec();
    kernel::grayscale_rgba(&mut pixels)?;
    Ok(pixels)
}
