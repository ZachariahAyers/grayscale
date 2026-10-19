//! The exported API across the JavaScript boundary. Run with `wasm-pack test --node`.
#![cfg(target_arch = "wasm32")]

use grayscale::grayscale;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn returns_grayscale_copy() {
    let input = [255, 0, 0, 255, 0, 0, 255, 64];
    assert_eq!(
        grayscale(&input).unwrap(),
        [54, 54, 54, 255, 18, 18, 18, 64]
    );
    assert_eq!(input, [255, 0, 0, 255, 0, 0, 255, 64]);
}

#[wasm_bindgen_test]
fn throws_on_partial_pixel() {
    assert!(grayscale(&[1, 2, 3]).is_err());
}
