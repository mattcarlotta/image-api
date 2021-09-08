# Rust Image API

A custom built API server to dynamically resize images based upon a URL query.

### Usage

```
http://127.0.0.1:5000/placeholder.png (original image)
http://127.0.0.1:5000/placeholder.png?ratio=0 (falls back to 100%)
http://127.0.0.1:5000/placeholder.png?ratio=50 (resizes the image to 50% of its width/height)
http://127.0.0.1:5000/placeholder_20.png?ratio=90 (falls back to resized image via ratio)
```

### How it works

By adding a `?ratio=n`, where `n` is one of the accepted integers below, query to a URL, this tells the
API to read the original image, create a new image based upon the requested ratio and return the result
to the client.

Accepted integer ratios (each integer below represent a percentage of the original width/height of an image):
`10, 20, 35, 50, 75, 90`

### What's the motivation?

Because window aspect ratios vary from device to device, serving a 3840×2160 pixel image to a 640x480 client
viewport is a waste of bandwidth and may result in image distortion. Utilizing a HTML [picture](https://www.w3schools.com/TAGS/tag_picture.asp) element, the browser
can determine which image to load based upon the current viewport width.

### Flow Chart

<img src="https://i.imgur.com/JXBexyz.png" />
