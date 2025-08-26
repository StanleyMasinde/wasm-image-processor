use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Adjust the contrast of this image.
/// `contrast` is the amount to adjust the contrast by.
/// Negative values decrease the contrast and positive values increase the contrast.
#[wasm_bindgen]
pub fn contrast(image_data: Vec<u8>, value: f32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image
        .adjust_contrast(value)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to adjust contrast: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contrast_50() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = contrast(test_image_data, 50.0).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image.save("test-output/contrast_50.jpg").unwrap();
    }
}
