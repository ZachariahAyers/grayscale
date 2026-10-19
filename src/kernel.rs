//! Image kernels over 8-bit RGBA pixel buffers. No JavaScript types live here.

use std::error::Error;
use std::fmt;

/// Rec. 709 luma weights in parts per ten thousand. They sum to `LUMA_SCALE`,
/// and match the weights the `image` crate uses for `grayscale()`.
const LUMA_WEIGHTS: [u32; 3] = [2126, 7152, 722];
const LUMA_SCALE: u32 = 10_000;

/// A pixel buffer whose length is not a whole number of RGBA pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotRgba {
    pub len: usize,
}

impl fmt::Display for NotRgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pixel buffer of {} bytes is not a whole number of RGBA pixels",
            self.len
        )
    }
}

impl Error for NotRgba {}

/// Luma of one sRGB-encoded pixel, truncated to an integer.
#[must_use]
pub fn luma(r: u8, g: u8, b: u8) -> u8 {
    let [wr, wg, wb] = LUMA_WEIGHTS;
    let sum = wr * u32::from(r) + wg * u32::from(g) + wb * u32::from(b);
    // The weights sum to LUMA_SCALE, so the quotient never exceeds 255.
    u8::try_from(sum / LUMA_SCALE).unwrap_or(u8::MAX)
}

/// Converts RGBA pixels to grayscale in place. Alpha is left untouched.
///
/// # Errors
///
/// Returns [`NotRgba`] without touching the buffer when its length is not a
/// multiple of four.
pub fn grayscale_rgba(pixels: &mut [u8]) -> Result<(), NotRgba> {
    if !pixels.len().is_multiple_of(4) {
        return Err(NotRgba { len: pixels.len() });
    }
    for pixel in pixels.chunks_exact_mut(4) {
        let y = luma(pixel[0], pixel[1], pixel[2]);
        pixel[..3].fill(y);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_and_white_are_fixed_points() {
        assert_eq!(luma(0, 0, 0), 0);
        assert_eq!(luma(255, 255, 255), 255);
    }

    #[test]
    fn primaries_use_rec_709_weights() {
        assert_eq!(luma(255, 0, 0), 54);
        assert_eq!(luma(0, 255, 0), 182);
        assert_eq!(luma(0, 0, 255), 18);
    }

    #[test]
    fn gray_input_stays_the_same_gray() {
        for v in 0..=255 {
            assert_eq!(luma(v, v, v), v);
        }
    }

    #[test]
    fn converts_every_pixel_and_keeps_alpha() {
        let mut pixels = [255, 0, 0, 255, 0, 255, 0, 128, 10, 20, 30, 0];
        grayscale_rgba(&mut pixels).unwrap();
        assert_eq!(pixels, [54, 54, 54, 255, 182, 182, 182, 128, 18, 18, 18, 0]);
    }

    #[test]
    fn empty_buffer_is_valid() {
        let mut pixels: [u8; 0] = [];
        assert_eq!(grayscale_rgba(&mut pixels), Ok(()));
    }

    #[test]
    fn partial_pixel_is_rejected_and_buffer_untouched() {
        let mut pixels = [255, 0, 0, 255, 9, 9];
        assert_eq!(grayscale_rgba(&mut pixels), Err(NotRgba { len: 6 }));
        assert_eq!(pixels, [255, 0, 0, 255, 9, 9]);
    }

    #[test]
    fn error_message_names_the_length() {
        assert_eq!(
            NotRgba { len: 6 }.to_string(),
            "pixel buffer of 6 bytes is not a whole number of RGBA pixels"
        );
    }
}
