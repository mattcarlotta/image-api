# Rust Image API

A custom built API server to dynamically resize images based upon a URL query.

### Usage

```
http://127.0.0.1:5000/file.ext (original image)
http://127.0.0.1:5000/file.ext?ratio=0 (falls back to 100%)
http://127.0.0.1:5000/file.ext?ratio=50 (downsamples the image to 50% of its width/height)
http://127.0.0.1:5000/file_20.ext?ratio=90 (falls back to resized image via ratio)
http://127.0.0.1:5000/a////b////c/d/e//file.ext (normalizes to a/b/c/d/e/file.ext)
http://127.0.0.1:5000/../file.ext (normalizes to file.ext)
http://127.0.0.1:5000/file.ext?ext=webp (converts file to file.webp)
http://127.0.0.1:5000/file.ext?ext=webp&ratio=10 (downsamples image to 10% and saves as file_ratio.webp)
```

### How it works

By adding a `?ratio=n` query to a URL, where `n` is one of the accepted integers below, this tells the API to read the original image, create a new image based upon the requested ratio and return the result to the client.

Accepted integer ratios (each integer below represents a **percentage of the original width/height** of an image):

```
10 (10%)
20 (20%)
35 (35%)
50 (50%)
75 (75%)
90 (90%)
```

By adding a `?ext=webp` query to a URL, where it can be included with a `ratio` (for example: `?ratio=10&ext=webp`) or standalone, this tells the API to downsample the original image (saves this downsample to disk), then takes the downsampled image and converts it to webp and then serves the result to the client.

### What's the motivation?

Because window aspect ratios vary from device to device, serving a 3840×2160 pixel image to a 640x480 client viewport is a waste of bandwidth and may result in image distortion. Utilizing a HTML [picture](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture) element or an image [src-set](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset) attribute, the browser can determine which image to load based upon the client's viewport width.

### Flow Chart

![flowchart](https://i.imgur.com/m7j3XOU.png)

### Future Features

- Ability to convert to any image extension on the fly

  Upstream issue: [Limited encoding support (webp has none)](https://github.com/image-rs/image#supported-image-formats)

  Status: Currently unfixable due to the [image](https://github.com/image-rs/image) crate not handling most image file type manipulations at all (for example: Can't convert to, save in, nor resize webp images)

  Workaround: [libwebp-sys](https://docs.rs/libwebp-sys/0.4.0/libwebp_sys/)

  Limitations: The workaround is limited to webp conversions and it's [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) because it taps into the [C Webp API](https://developers.google.com/speed/webp/docs/api), uses null pointers, and creates slices from memory; as such, conversions will only cover converting from png to webp for now. In testing, converting images to other image formats (like jpeg) resulted in larger file sizes! That said, since webp has [good browser support](https://caniuse.com/webp), its worthwhile to include limited support. In the future, switching to [avif](https://caniuse.com/avif) might be more suitable.
