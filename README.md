[![Crates.io](https://img.shields.io/crates/v/buffer-graphics-lib)](https://crates.io/crates/buffer-graphics-lib "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/buffer-graphics-lib)](https://docs.rs/buffer-graphics-lib "Documentation")

# Buffer Graphics Lib

This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Pixels Graphics](https://github.com/emmabritton/pixel-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)

It has basic shape drawing, bitmap text and image rendering.

The `Graphics` struct needs a mutable slice to work on and so mostly likely you'll need to create the struct and pass in the buffer from the rendering library every frame but this should be fine as the struct is nearly empty. 

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
buffer-graphics-lib = "0.10.9"
```

### Code

Setup a graphics instance
```rust
let mut buffer: [u8; 1920000] = [0; 800 * 600 * 4]; //800 x 600 RGBA 
let mut graphics = Graphics::new(&mut buffer, 800, 600)?;
```

Drawing is then quite simple:
```rust
let text = Text::new("Some text", (1,1), (WHITE, Large));
graphics.draw(&text);
graphics.draw_image(20, 20, &image);
let shape = Rect::new((10,10),(50,50));
let drawable = Drawable::from_obj(shape, stroke(BLUE));
graphics.draw(&drawable);
```

## Features

### `ici`

Load ici image files as `IndexedImage`s

```rust
let image: IndexedImage...
graphics.draw_indexed_image(coord, &image);
```

### `image_loading`

Load png, bmp, etc image files as `Image`s

#### Code
```rust
let image = load_image("resources/example.png")?;
graphics.draw_image(40, 20, &image);
```

### `serde_derive`

Enabled by default

Adds derive `Serialize` and `Deserialize` for `DrawType` and `Color`
