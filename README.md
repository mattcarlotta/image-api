# Rust Image API

A custom built API server to dynamically resize images based upon a URL query.

## Usage

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

## What's the motivation?

Because window aspect ratios vary from device to device, serving a 3840×2160 pixel image to a 640x480 client viewport is a waste of bandwidth and may result in image distortion. Utilizing a HTML [picture](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture) element or an image [src-set](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset) attribute, the browser can determine which image to load based upon the client's viewport width.

## How it works

By requesting an image with some additional URL queries, the API interprets the provided queries and downsamples/converts the requested image on-the-fly.

All unique image requests are stored into a [LRU cache](<https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU)>) on initial request; this means any subsequent requests will be served an encoded image from the cache instead of reading/encoding the file from disk. This assumes that the original file sizes are relatively small (8mb or less); otherwise, it's recommended to either reduce the amount allowed to be stored in the LRU cache (default: [50](https://github.com/mattcarlotta/image-api/blob/main/src/main.rs#L36)) or to just remove the cache entirely.

## URL Queries

Add one or more of the following queries to manipulate an image on the fly.

#### Ratio

By adding a `?ratio=n` query to an image request, where `n` is one of the accepted integers below, this communicates to the API to read the original image, downsample it according to the ratio, save it as a new image and to serve the result to the client.

Accepted integer ratios (each integer below represents a **percentage of the original width/height** of an image):

```
10 (10%)
20 (20%)
35 (35%)
50 (50%)
75 (75%)
90 (90%)
```

#### Ext

By adding a `?ext=webp` query to a request, where it can be included with a `ratio` (for example: `?ratio=10&ext=webp`) or standalone, this communicates to the API to downsample the original image according to the ratio †, then to take the downsampled image, compress/convert it to the `webp` file format and lastly to serve the converted result to the client. For now, only `webp` file conversions are accepted (see [Future Features](#future-features) for more info).

† The image will only be downsampled if a ratio was provided; otherwise, it'll just retain the original width and height of the image. This downsampled result will conditionally be saved to disk if it hasn't been already. While this means two images may be created from one request, this has the added benefit of not having to downsample the original image twice with the same ratio (if a future request is made without the image extension query).

### Flow Chart

![flowchart](https://i.imgur.com/m7j3XOU.png)

### Future Features

- Ability to convert to any image extension on the fly

  Upstream issue: [Limited encoding support](https://github.com/image-rs/image#supported-image-formats)

  Status: Currently unfixable due to the [image](https://github.com/image-rs/image) crate not handling most image file type manipulations at all (for example, can't convert to, save in, nor resize webp images)

  Workaround: [libwebp-sys](https://docs.rs/libwebp-sys/0.4.0/libwebp_sys/)

  Limitations: The workaround is limited to webp conversions and it's [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) because it taps into the [C Webp API](https://developers.google.com/speed/webp/docs/api), uses null pointers, and creates slices from memory; as such, conversions will only cover converting from png to webp. In testing, converting images to other image formats (like jpeg) resulted in larger file sizes! That said, since webp has [good browser support](https://caniuse.com/webp) and better file compression, its worthwhile to include limited support. In the future, switching to [avif](https://caniuse.com/avif) might be more suitable.
