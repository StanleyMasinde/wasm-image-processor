use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Invert the colors on this image.
#[wasm_bindgen]
pub fn invert(image_data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let mut image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image.invert();

    image
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to invert the colors: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invert() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = invert(test_image_data).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image.save("test-output/invert.jpg").unwrap();
    }
}
