use std::io::Cursor;

use image::imageops::FilterType;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Resize an image
/// Take an array of bytes, the len and the width
/// The image's aspect ratio is preserved.
/// The image is scaled to the maximum possible size that fits
/// within the bounds specified by `width` and `height`
#[wasm_bindgen]
pub fn resize(image_data: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get the image format: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image.: {err}")))?;

    let mut buf = Vec::new();
    let _ = image
        .resize(width, height, FilterType::Nearest)
        .write_to(&mut Cursor::new(&mut buf), format);

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_resize() {
        let test_image_data = include_bytes!("../sample.jpg").to_vec();
        let resized_bytes = resize(test_image_data, 512, 513).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();
        assert_eq!(resized_image.width(), 512);

        resized_image
            .save("test-output/aspect_resized.jpg")
            .unwrap();
    }
}
