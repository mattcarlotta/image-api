# Rust Image API

A custom built API server to dynamically resize images based upon a URL query.

### Usage

```
http://127.0.0.1:5000/file.ext (original image)
http://127.0.0.1:5000/file.ext?ratio=0 (falls back to 100%)
http://127.0.0.1:5000/file.ext?ratio=50 (resizes the image to 50% of its width/height)
http://127.0.0.1:5000/file_20.ext?ratio=90 (falls back to resized image via ratio)
http://127.0.0.1:5000/a////b////c/d/e//file.ext (normalizes to a/b/c/d/e/file.ext)
http://127.0.0.1:5000/../file.ext (normalizes to file.ext)
```

### How it works

By adding a `?ratio=n` query to a URL, where `n` is one of the accepted integers below, this tells the
API to read the original image, create a new image based upon the requested ratio and return the result
to the client.

Accepted integer ratios (each integer below represents a **percentage of the original width/height** of an image):

```
10 (10%)
20 (20%)
35 (35%)
50 (50%)
75 (75%)
90 (90%)
```

### What's the motivation?

Because window aspect ratios vary from device to device, serving a 3840×2160 pixel image to a 640x480 client
viewport is a waste of bandwidth and may result in image distortion. Utilizing a HTML [picture](https://www.w3schools.com/TAGS/tag_picture.asp) element or an image
`src-set` attribute, the browser can determine which image to load based upon the client's viewport width.

### Flow Chart

![flowchart](https://i.imgur.com/m7j3XOU.png)

### Future Features

- Ability to convert extensions on the fly

  Upstream issue: [Webp conversion is not supported](https://github.com/image-rs/image/issues/582)

  Status: Currently unfixable due to the image crate not handling webp manipulations at all (can't convert to nor resize webp images)

  Workaround: Don't implement support for on-the-fly image conversions (supported filetypes result in larger file sizes compared to png)
