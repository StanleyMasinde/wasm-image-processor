use std::io::Cursor;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::read_image::read_image;

/// Scale this image down to fit within a specific size.
/// Returns a new image. The image's aspect ratio is preserved.
/// The image is scaled to the maximum possible size that fits
/// within the bounds specified by `nwidth` and `nheight`.
///
/// This method uses a fast integer algorithm where each source
/// pixel contributes to exactly one target pixel.
/// May give aliasing artifacts if new size is close to old size.
#[wasm_bindgen]
pub fn thumbnail(image_data: Vec<u8>, width: u32, height: u32) -> Result<Vec<u8>, JsValue> {
    let format = image::guess_format(&image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to get the image format: {err}")))?;

    let image = read_image(image_data)
        .map_err(|err| JsValue::from_str(&format!("Failed to read image.: {err}")))?;

    let mut buf = Vec::new();
    image
        .thumbnail(width, height)
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|err| JsValue::from_str(&format!("Failed to create the thumbnail: {err}")))?;

    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_thumbnail() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let resized_bytes = thumbnail(test_image_data, 512, 513).unwrap();

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();
        assert_eq!(resized_image.width(), 512);

        resized_image.save("test-output/thumbnail_512.jpg").unwrap();
    }
}
