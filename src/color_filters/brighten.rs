use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Brighten image
/// The value is -100 to 100
#[wasm_bindgen]
pub fn brighten(image_data: Vec<u8>, value: i32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image
        .brighten(value)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to adjust brightness: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_brighten_100() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = brighten(test_image_data, 100).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image.save("test-output/brighten_100.jpg").unwrap();
    }
}
