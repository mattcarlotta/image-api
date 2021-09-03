# Rust Image API

A custom built ground-up API server to dynamically resize images based upon a URL query.

Try visiting:

```
http://127.0.0.1:5000/placeholder.png
http://127.0.0.1:5000/placeholder.png?ratio=50
http://127.0.0.1:5000/placeholder_20.png?ratio=90 (falls back to placeholder.png with ratio)
```

Accepted ratios:
`0, 20, 35, 50, 75, 90`

### Flow Chart

<img src="https://i.imgur.com/Uc8iUkX.png" />
