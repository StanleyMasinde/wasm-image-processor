use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

use crate::color_filters::{
    blur::blur, brighten::brighten, contrast::contrast, fast_blur::fast_blur, grayscale::grayscale,
    hue_rotate::hue_rotate, invert::invert,
};
use crate::transformation::{
    crop::crop, resize::resize, resize_square::resize_square, thumbnail::thumbnail,
};

// This class is here to be used in a builder pattern.
// It allows for a single image to go through multiplce modifications
#[wasm_bindgen]
#[derive(Debug, Default)]
/// Builder-style image processor for JS/Wasm usage.
///
/// ```javascript
/// // Browser usage (after wasm-bindgen or bundler setup)
/// // const { ImageProcessor } = await init();
/// const input = document.querySelector("#file");
/// const outputImg = document.querySelector("#output");
///
/// input.addEventListener("change", async (event) => {
///   const file = event.target.files[0];
///   if (!file) return;
///
///   const inputBytes = new Uint8Array(await file.arrayBuffer());
///   const outputBytes = new ImageProcessor(inputBytes)
///     .resize(512, 512)
///     .grayscale()
///     .contrast(25.0)
///     .process();
///
///   const blob = new Blob([outputBytes], { type: file.type });
///   outputImg.src = URL.createObjectURL(blob);
/// });
/// ```
///
/// ```javascript
/// // Canvas usage (after wasm-bindgen or bundler setup)
/// // const { ImageProcessor } = await init();
/// const input = document.querySelector("#file");
/// const canvas = document.querySelector("#canvas");
/// const ctx = canvas.getContext("2d");
///
/// input.addEventListener("change", async (event) => {
///   const file = event.target.files[0];
///   if (!file) return;
///
///   const inputBytes = new Uint8Array(await file.arrayBuffer());
///   const outputBytes = new ImageProcessor(inputBytes)
///     .resize(512, 512)
///     .blur(2.0)
///     .process();
///
///   const blob = new Blob([outputBytes], { type: file.type });
///   const bitmap = await createImageBitmap(blob);
///   canvas.width = bitmap.width;
///   canvas.height = bitmap.height;
///   ctx.clearRect(0, 0, canvas.width, canvas.height);
///   ctx.drawImage(bitmap, 0, 0);
/// });
/// ```
pub struct ImageProcessor {
    image: Vec<u8>,
}

fn to_js_error(context: &str, err: JsValue) -> JsError {
    let message = err
        .as_string()
        .unwrap_or_else(|| "Unknown error".to_string());
    JsError::new(&format!("{context}: {message}"))
}

#[wasm_bindgen]
impl ImageProcessor {
    /// Create a new processor from raw image bytes.
    pub fn new(image: Vec<u8>) -> Self {
        Self { image }
    }

    /// Calling this returns the final image bytes.
    pub fn process(self) -> Vec<u8> {
        self.image
    }

    pub fn resize(mut self, width: u32, height: u32) -> Result<Self, JsError> {
        let buff = resize(self.image, width, height)
            .map_err(|err| to_js_error("Failed to resize the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn resize_square(mut self, side: u32) -> Result<Self, JsError> {
        let buff = resize_square(self.image, side)
            .map_err(|err| to_js_error("Failed to resize the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn thumbnail(mut self, width: u32, height: u32) -> Result<Self, JsError> {
        let buff = thumbnail(self.image, width, height)
            .map_err(|err| to_js_error("Failed to create the thumbnail", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn crop(mut self, x: u32, y: u32, width: u32, height: u32) -> Result<Self, JsError> {
        let buff = crop(self.image, x, y, width, height)
            .map_err(|err| to_js_error("Failed to crop the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn blur(mut self, sigma: f32) -> Result<Self, JsError> {
        let buff =
            blur(self.image, sigma).map_err(|err| to_js_error("Failed to blur the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn fast_blur(mut self, sigma: f32) -> Result<Self, JsError> {
        let buff = fast_blur(self.image, sigma)
            .map_err(|err| to_js_error("Failed to blur the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn brighten(mut self, value: i32) -> Result<Self, JsError> {
        let buff = brighten(self.image, value)
            .map_err(|err| to_js_error("Failed to adjust brightness", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn contrast(mut self, value: f32) -> Result<Self, JsError> {
        let buff = contrast(self.image, value)
            .map_err(|err| to_js_error("Failed to adjust contrast", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn grayscale(mut self) -> Result<Self, JsError> {
        let buff = grayscale(self.image)
            .map_err(|err| to_js_error("Failed to convert to grayscale", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn invert(mut self) -> Result<Self, JsError> {
        let buff =
            invert(self.image).map_err(|err| to_js_error("Failed to invert the image", err))?;
        self.image = buff;
        Ok(self)
    }

    pub fn hue_rotate(mut self, degrees: i32) -> Result<Self, JsError> {
        let buff = hue_rotate(self.image, degrees)
            .map_err(|err| to_js_error("Failed to hue rotate", err))?;
        self.image = buff;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaining() {
        let test_image_data = include_bytes!("../../sample.jpg").to_vec();
        let result = ImageProcessor::new(test_image_data)
            .resize(512, 512)
            .unwrap()
            .grayscale()
            .unwrap()
            .contrast(25.0)
            .unwrap()
            .process();

        let result_image = image::load_from_memory(&result).unwrap();
        assert_eq!(result_image.width(), 512);

        result_image.save("test-output/chaining.jpg").unwrap();
    }
}
