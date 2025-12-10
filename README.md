# WASM Image Processor

> High-performance client-side image processing powered by Rust and WebAssembly

Process images entirely in the browser with near-native performance. No server uploads, works offline, respects user privacy.

**[Live Demo](https://wasm-ip-demo.vercel.app)** | **[Full Documentation](https://wip-docs.vercel.app)** | **[npm Package](https://www.npmjs.com/package/wasm-image-processor)**

---

## Quick Start

### For JavaScript Users

```bash
npm install wasm-image-processor
```

**Vite users:** Install `vite-plugin-wasm` and add it to your config (see [setup guide](https://wip-docs.vercel.app/getting-started.html#vite-configuration))

```javascript
import { resize_square, blur, grayscale } from "wasm-image-processor";

// Resize an image to 512x512
const resizedBytes = resize_square(imageUint8Array, 512);

// Apply Gaussian blur
const blurredBytes = blur(imageUint8Array, 5.0);

// Convert to grayscale
const grayBytes = grayscale(imageUint8Array);
```

**Key features:**
- Auto-initializes WASM (no manual setup)
- ~150KB gzipped bundle size
- TypeScript types included
- Supports PNG and JPEG formats
- See [complete documentation](https://wip-docs.vercel.app) for all functions

---

### For Rust Contributors

**Project structure:**
```
src/
  lib.rs          # WASM bindings & public API
  processing/     # Core image processing logic
  utils/          # Helpers
Cargo.toml
```

**Local development:**
```bash
# Build for web
wasm-pack build --target web --out-dir pkg

# Run tests
cargo test
wasm-pack test --headless --firefox

# Test in browser
cd example && npm install && npm run dev
```

**Key dependencies:**
- `image = "0.24"` for core processing
- `wasm-bindgen = "0.2"` for JS interop
- Safe Rust only, panic-safe for WASM context

**Contributing:** Fork the repo, create a feature branch, and open a PR. See [documentation repo](https://github.com/StanleyWorks/wasm-image-processor-docs) for docs contributions.

---

## Available Functions

**Core Operations:**
- `resize_square(bytes, size)` - Resize to square dimensions
- `resize(bytes, width, height)` - Resize to custom dimensions
- `crop(bytes, x, y, width, height)` - Crop to region
- `thumbnail(bytes, width, height)` - Generate thumbnail

**Filters & Adjustments:**
- `blur(bytes, sigma)` - Gaussian blur
- `fast_blur(bytes, sigma)` - Optimized blur
- `contrast(bytes, value)` - Adjust contrast (-100 to 100)
- `brighten(bytes, value)` - Adjust brightness (-100 to 100)
- `grayscale(bytes)` - Convert to grayscale
- `invert(bytes)` - Invert colors
- `hue_rotate(bytes, degrees)` - Rotate hue (0-360)

**Coming Soon:**
- `rotate(bytes, degrees)` - Rotate image

See [full API documentation](https://wip-docs.vercel.app/api.html) with live examples for each function.

---

## Why Use This?

**For end users:**
- Privacy: Images never leave your device
- Speed: Process images in milliseconds
- Offline: Works without internet connection
- Universal: Runs in any modern browser

**For developers:**
- Rust performance in JavaScript
- Simple, predictable API
- Tree-shakeable ES modules
- No server infrastructure needed

---

## Use Cases

**Profile picture editors:**
Generate multiple sizes for avatars without server round-trips. Resize to 32px, 64px, 128px, 256px instantly in the browser.

**Photo galleries & portfolios:**
Create thumbnails client-side before upload. Apply filters for preview without processing server load.

**PWA & offline apps:**
Process images when users have no internet connection. Perfect for field work apps, travel journals, or offline-first tools.

**Privacy-sensitive applications:**
Medical imaging viewers, legal document processors, or any app where images contain sensitive data that shouldn't touch external servers.

**Image compression tools:**
Build "TinyPNG alternatives" that run entirely in the browser. Users maintain full control of their files.

**Batch processing utilities:**
Resize hundreds of product photos, apply watermarks, or normalize images for e-commerce without server costs or upload time.

**Design tools & editors:**
Add real-time image adjustments (brightness, contrast, blur) to your web app without heavy JavaScript libraries.

**Form enhancements:**
Automatically resize large images before form submission to reduce upload size and improve UX.

---

## Browser Support

- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Any browser with [WebAssembly support](https://caniuse.com/wasm)

---

## Complete Example

```html
<input type="file" id="fileInput" />
<button id="downloadBtn" style="display: none;">Download Processed Image</button>

<script type="module">
  import { resize_square, blur } from "wasm-image-processor";

  const input = document.getElementById("fileInput");
  const downloadBtn = document.getElementById("downloadBtn");

  input.addEventListener("change", async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    try {
      // Read file
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      // Process: resize and blur
      const resized = resize_square(uint8Array, 512);
      const processed = blur(resized, 2.0);

      // Create downloadable blob
      const blob = new Blob([processed], { type: "image/png" });
      const url = URL.createObjectURL(blob);

      // Setup download
      downloadBtn.style.display = "block";
      downloadBtn.onclick = () => {
        const a = document.createElement("a");
        a.href = url;
        a.download = "processed-image.png";
        a.click();
        URL.revokeObjectURL(url);
      };
    } catch (error) {
      console.error("Processing failed:", error);
    }
  });
</script>
```

See [more examples](https://wip-docs.vercel.app/examples.html) including Vue, Nuxt, and React patterns.

---

## Performance

Processing happens entirely client-side using WebAssembly, providing:
- Fast processing with near-native performance
- Complete privacy - images never uploaded
- Offline capability
- Zero server costs

**Typical operations:**
- Resize 4K image: ~45ms
- Apply blur filter: ~30ms
- Grayscale conversion: ~10ms

Performance varies by device and image size.

---

## Documentation

- [Getting Started Guide](https://wip-docs.vercel.app/getting-started.html)
- [Complete API Reference](https://wip-docs.vercel.app/api.html)
- [Live Examples](https://wip-docs.vercel.app/examples.html)
- [Changelog](https://wip-docs.vercel.app/changelog.html)

---

## Common Issues

**Import errors:** Ensure your bundler supports WebAssembly. Vite users need `vite-plugin-wasm` installed and configured.

**"Failed to read image":** Only PNG and JPEG formats are currently supported.

**Memory issues:** Large images may cause problems on mobile devices. Consider resizing or adding file size limits.

See [troubleshooting guide](https://wip-docs.vercel.app/getting-started.html#troubleshooting) for more help.

---

## Contributing

Contributions welcome! To contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

For documentation improvements, contribute to the [docs repository](https://github.com/StanleyWorks/wasm-image-processor-docs).

---

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- [image-rs](https://github.com/image-rs/image) crate for image processing
- Inspired by the need for privacy-respecting browser-based tools

**Questions?** Open an [issue](../../issues) or check the [documentation](https://wip-docs.vercel.app).
