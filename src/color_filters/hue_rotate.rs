use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Hue rotate the supplied image.
/// `value` is the degrees to rotate each pixel by.
/// 0 and 360 do nothing, the rest rotates by the given degree value.
/// just like the css webkit filter hue-rotate(180)
#[wasm_bindgen]
pub fn hue_rotate(image_data: Vec<u8>, degrees: i32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image
        .huerotate(degrees)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to hue rotate: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hue_rotate_100() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = hue_rotate(test_image_data, 100).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image
            .save("test-output/hue_rotate_100.jpg")
            .unwrap();
    }
}
