use std::io::Cursor;

use wasm_bindgen::JsValue;

use crate::utils::read_image::read_image;

/// Add grayscale effect to image
pub fn grayscale(image_data: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get image type: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image: {err}")))?;

    let mut buf = Vec::new();

    image
        .grayscale()
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to resize the image: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grayscale() {
        let test_image_data = include_bytes!("../sample.jpg").to_vec();
        let resized_bytes = grayscale(test_image_data).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();

        resized_image.save("test-output/grayscale.jpg").unwrap();
    }
}
