use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Return a cut-out of this image delimited by the bounding rectangle.
#[wasm_bindgen]
pub fn crop(
    image_data: Vec<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get the image format: {err}")))?;

    let mut image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();
    image
        .crop(x, y, width, height)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to crop the image: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crop_10x10() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = crop(test_image_data, 10, 10, 1000, 1000).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();
        assert_eq!(resized_image.width(), 1000);

        resized_image.save("test-output/crop_10_10.jpg").unwrap();
    }
}
