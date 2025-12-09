# WASM Image Processor
> Process images offline in the browser. 
> It uses WASM to expose a Rust image processing crate. 

A high-performance image processing toolkit built with Rust and WebAssembly. Process images entirely client-side with no server uploads required.

## Features

- **Offline Processing**: All image operations happen in your browser
- **High Performance**: Powered by Rust and WebAssembly
- **PWA Icon Generation**: Create complete icon sets for Progressive Web Apps
- **Multiple Formats**: Support for JPEG, PNG, WebP, and more

## Demo

Try the live demo at: [https://wasm-ip-demo.vercel.app](https://wasm-ip-demo.vercel.app)

## Current Features

### Image Resizing
- Resize images to any dimensions up to 5000x5000 pixels
- Maintain aspect ratio or force specific dimensions
- High-quality filtering algorithms


## Installation
```shell
npm i wasm-image-processor
```

### Usage

Include the WASM module in your web page:

```javascript
import { resize_square } from "wasm-image-processor";

// Example: resize an image uploaded via <input type="file">
const fileInput = document.querySelector<HTMLInputElement>("#fileInput")!;

fileInput.addEventListener("change", () => {
  const file = fileInput.files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    const arrayBuffer = reader.result as ArrayBuffer
    const uint8Array = new Uint8Array(arrayBuffer)

    // Resize to 512x512
    const resizedBytes = resize_square(uint8Array, 512)

    // Create a new File from the resized bytes
    const resizedImage = new File([resizedBytes], "resized.png", {
      type: "image/png",
    })

    console.log("Resized image:", resizedImage)
  }

  reader.readAsArrayBuffer(file)
})
```

## Roadmap

**Near Term:**
- [ ] Stable API design
- [x] npm package publication
- [x] Comprehensive documentation
- [x] CI/CD pipeline

**Future Features:**
- [ ] Image format conversion (PNG ↔ JPEG ↔ WebP)
- [ ] Image compression with quality settings
- [ ] Batch processing for multiple images
- [ ] Image filters (blur, sharpen, brightness, contrast)
- [ ] Custom crop functionality
- [ ] Image rotation and flipping
- [ ] Metadata preservation and editing
- [ ] Advanced resizing algorithms

## Browser Support

- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Any browser with WebAssembly support

## Performance

Processing is done entirely client-side using WebAssembly, providing:
- **Fast processing**: Near-native performance
- **Privacy**: Images never leave your device
- **Offline capability**: Works without internet connection
- **Scalability**: No server resources required

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- Image processing powered by the [image](https://github.com/image-rs/image) crate
- Inspired by the need for client-side image processing tools
