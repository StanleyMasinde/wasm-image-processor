use std::io::Cursor;

use image::{DynamicImage, ImageError, ImageReader};

/// Read image file from a stream of bytes.
/// This function returns `DynamicImage` so that it can be processed.
///
/// The reason we are passing a Vec<u8> is because wasm_bindgen can pass it to JS.
/// It cannot directly pass DynamicImage to JS.
pub fn read_image(image_data: Vec<u8>) -> Result<DynamicImage, ImageError> {
    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()?
        .decode()?;

    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_image() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let result = read_image(test_image_data);
        assert!(result.is_ok());
    }
}
