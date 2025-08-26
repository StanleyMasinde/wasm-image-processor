use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Performs a fast blur on this image.
/// `sigma` is the standard deviation of the
/// (approximated) Gaussian
#[wasm_bindgen]
pub fn fast_blur(image_data: Vec<u8>, sigma: f32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image
        .fast_blur(sigma)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to blur the image: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fast_blur() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = fast_blur(test_image_data, 4.0).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image.save("test-output/fast_blur.jpg").unwrap();
    }
}
