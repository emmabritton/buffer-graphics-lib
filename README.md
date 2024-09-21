[![Crates.io](https://img.shields.io/crates/v/buffer-graphics-lib)](https://crates.io/crates/buffer-graphics-lib "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/buffer-graphics-lib)](https://docs.rs/buffer-graphics-lib "Documentation")

# Buffer Graphics Lib

This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Pixels Graphics](https://github.com/emmabritton/pixel-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)

It has basic shape drawing, bitmap text and image rendering.

The `Graphics` struct needs a mutable buffer to work on and so mostly likely you'll need to create the struct and pass in the buffer from the rendering library every frame but this should be fine for performance as the struct is nearly empty. 

[ICI Tools](https://github.com/emmabritton/ici_tools) can be useful when working with ICI files

[ICI Image editor](https://github.com/emmabritton/ici-image-editor) is a MSPaint like program for ICI and ICA files

[Graphics tests](https://github.com/emmabritton/graphics-tester) has tests for this crate, and provides some examples

[Integration tests](https://github.com/emmabritton/integration_tests) shows how to use some third party libraries

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
buffer-graphics-lib = "0.18.2"
```

### Code

Setup a graphics instance
```rust
let mut buffer: [u8; 1920000] = Graphics::create_buffer(800,600); //800 x 600 RGBA 
let mut graphics = Graphics::new(&mut buffer, 800, 600)?;
```

Drawing is then quite simple:
```rust
let text = Text::new("Some text", (1,1), (WHITE, PixelFont::Standard6x7));
graphics.draw(&text);
graphics.draw_image(20, 20, &image);
graphics.draw_rect(Rect::new((40, 50), (100, 100)), stroke(BLUE));
```

## Features

> Default features: "serde"

### `image_loading`

* Load png, bmp, etc image files as `Image`s

#### Code
```rust
let image = open_image("resources/example.png")?;
graphics.draw_image(40, 20, &image);
```

### `serde`

* Adds derive `Serialize` and `Deserialize` for most structs and enums
* Enables `graphics-shapes/serde`

### `mint`

* Enables `graphics-shapes/mint` 