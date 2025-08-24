use std::{error::Error, io::Cursor};

use image::{imageops::FilterType, DynamicImage, ImageReader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(m: &str);
}

/// Resize an image by the given dimension.
/// The first parameter is an array of bytes.
/// The second parameter is the size of the sides.
/// This function returns square images. It does not respect the aspect
/// Ratio of the photo.
/// It is is ideal for icon resizing.
#[wasm_bindgen]
pub fn resize_square(image_data: Vec<u8>, side: u32) -> Vec<u8> {
    let image = read_image(image_data).unwrap();
    let mut buf = Vec::new();
    let _ = image
        .resize_exact(side, side, FilterType::Nearest)
        .write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png);

    buf
}

/// Read image file from a stream of bytes.
/// This function returns `DynamicImage` so that it can be processed.
///
/// The reason we are passing a Vec<u8> is because wasm_bindgen can pass it to JS.
/// It cannot directly pass DynamicImage to JS.
pub fn read_image(image_data: Vec<u8>) -> Result<DynamicImage, Box<dyn Error>> {
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
        let test_image_data = include_bytes!("../sample.jpg").to_vec();
        let result = read_image(test_image_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resize_square() {
        let test_image_data = include_bytes!("../sample.jpg").to_vec();
        let resized_bytes = resize_square(test_image_data, 512);

        let resized_image = image::load_from_memory(&resized_bytes).unwrap();
        assert_eq!(resized_image.width(), 512);
        assert_eq!(resized_image.height(), 512);

        resized_image.save("test-output/resized.jpg").unwrap();
    }

    #[test]
    fn test_multiple_resize_square() {
        let test_image_data = include_bytes!("../sample.jpg").to_vec();

        let pwa_sizes = vec![72, 128, 144, 192, 512];

        for size in pwa_sizes {
            let resized_bytes = resize_square(test_image_data.clone(), size);
            let resized_image = image::load_from_memory(&resized_bytes).unwrap();

            resized_image
                .save(format!("./test-output/icons/{size}x{size}.jpg"))
                .unwrap();
        }
    }
}
