# Buffer Graphics Lib

This is a simple graphics library for drawing to a buffer, mainly designed to be used with [Rust Graphics Library](https://github.com/raybritton/rust-graphics-lib) or [Pixels](https://github.com/parasyte/pixels)

It has basic shape drawing, bitmap text and image rendering.

The `Graphics` struct needs a mutable slice to work on and so mostly likely you'll need to create the struct and pass in the buffer from the rendering library every frame. 

## Usage

### Cargo

In your `Cargo.toml` file add
```toml
buffer-graphics-lib = "0.9.4"
graphics-shapes = "0.1.10"
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

Both are enabled by default

### `image_loading`

Load files as `Image`s

#### Code
```rust
let image = load_image("resources/example.png")?;
graphics.draw_image(40, 20, &image);
```

### `serde_derive`

Adds derive `Serialize` and `Deserialize` for `Rect`
