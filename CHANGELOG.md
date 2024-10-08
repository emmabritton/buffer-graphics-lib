# Changelog

### Version 0.19.0
### Breaking
- To support other buffer types (`u8` and `u32`):
  - Remove `Graphics::new` and `Graphics::create_buffer`
  - Add `Graphics::new_u8_rgba`, `Graphics::create_buffer_u8`, `Graphics::new_u32_rgba`, `Graphics::new_u32_argb`, `Graphics::create_buffer_u32`
- Migration
```
let mut buffer = Graphics::create_buffer(320, 200);
let graphics = Graphics::new(&mut buffer, 320, 200);
```
becomes
```
let mut buffer = Graphics::create_buffer_u8(320, 200);
let graphics = Graphics::new_u8_rgba(&mut buffer, 320, 200);
```
- `Image::pixels()` now returns a reference rather than a copy
- Remove exact dep versions

### Version 0.18.2
- Update deps
- Add more docs

### Version 0.18.1
- Update deps
- Tidy up some letters

### Version 0.18.0
- Add `create_image` method
- Update ici lib

### Version 0.17.0
- Add convenience method `Graphics::create_buffer`
- Fix bug if WrappingStrategy was passed `0`
- New font 'Limited 3x5' (no lower case, some symbols look bad)

### Version 0.16.2
- Update ici lib

### Version 0.16.1
- Fix import issues 

### Version 0.16.0
- Replaced `TextSize` with `PixelFont`
  - `Small` becomes `Standard4x5`
  - `Normal` becomes `Standard6x7`
  - `Large` becomes `Standard8x10`
  - Add `Stanard4x4`, `Outline7x9`, `Script8x8`
- Fix `char_width` and `line_height` in formatting
- Update ici lib
  - Color and scaling has been moved
  - `IndexedImage` can be rotated, scaled, tinted
  - ICI feature removed
- Fix new line support in `WrappingStrategy::SpaceBeforeCol`

### Version 0.15.1
- Add `renderable` and `sized_renderable` macros
- Add support for `ImageWrapper`
- Update ici lib

### Version 0.15.0
- Update ici lib
- Remove `Graphics::get_text_size()`, use `Textsize::measure()` instead
- Tidy up code

### Version 0.14.0
- Add `Image` from `IndexedImage` method
- Shift Font Small `0` right by 1px
- Add new line support for text rendering
- Fix `WrappingStrategy::AtColWithHyphen`
- Update shapes lib

### Version 0.13.0
- Rename feature `serde_derive` to `serde`
- Add feature based `Serialize` and `Deserialize` to most structs and enums
- Add feature `mint`

### Version 0.12.7
- Update shapes lib

### Version 0.12.6
- Fix prelude/imports
- Fix crash from offscreen drawing

### Version 0.12.5
- Update shapes lib

### Version 0.12.4
- Add `ShapeCollection::bounds()`

### Version 0.12.3
- Update shapes lib

### Version 0.12.2
- Update shapes lib

### Version 0.12.1
- Add check char to font `✓`
- Add support for custom replacement glyphs
  - For example, this allows 'a' to be printed as '@' (or any custom pattern)

### Version 0.12.0
- `Image`
  - Change internals to use bytes
  - Speed up drawing of opaque images
  - Add `blend_pixel()`
  - Add `is_transparent(): bool`
  - Add `pixels(): Vec<Color>`
  - Add `rotate_cw()`, `rotate_ccw()`
  - Change `blend()` to change self rather than creating a new image (for consistency with other methods)

### Version 0.11.4
- Fix invalid chars in `SUPPORTED_SYMBOLS`

### Version 0.11.3
- Update ici-files

### Version 0.11.2
- Update ici-files

### Version 0.11.1
- Update graphics-shapes
  - Added Ellipse

### Version 0.11.0
- Add clipping
- Add `Graphics::clear_aware` for clear that uses translate and clipping
- Move `ShapeBox` to `graphics-shapes`
  - It now holds a shape directly, and needs to be wrapped in `Drawable` to render
  - `ShapeCollection` now holds `Drawable<ShapeBox>`
- Update graphics-shapes to 0.2.0
  - Removed `Ellipse`

### Version 0.10.11
- Fix bug in ICI rendering

### Version 0.10.10
- Update deps

### Version 0.10.9
- Update deps
- Change rendering internals

### Version 0.10.8
- Update deps

### Version 0.10.7
- Update deps

### Version 0.10.6
- Update deps
- Add `is_transparent` to `Color`

### Version 0.10.5 
- Add to ici and color for color and ici

### Version 0.10.4
- Add width and height to IndexedImage

### Version 0.10.3
- Add brightness and saturation methods for `Color`, `IciColor`, `IndexedImage`, `AnimatedIndexedImage`

### Version 0.10.2
- Fix `image_loading` feature

### Version 0.10.1
- Fix `ici` feature so it actually exports

### Version 0.10.0
- Removed `IndexedImage`
- Add optional `ici-files` dependency and feature, defaults to active
- Adds `graphics::draw_indexed_image` and `graphics::draw_animated_image` for ICIs

### Version 0.9.7
- Fix `IndexedImage::to_image()`

### Version 0.9.6
- Update shapes library
- Add `IndexedImage` (image using indexed color that should be saved as RON)
- Add `SUPPORTED_SYMBOLS`
- Fix a few letters graphics

### Version 0.9.5
- Add `left()`, `right()`, `top()`, `bottom()` and `center()` to `ShapeCollection` and `ShapeBox`()
- Fix bug with `ShapeCollection::with_*`
- Add `ShapeCollection::insert_*` for drawables and shapes 

### Version 0.9.4
- Update shapes library
- Add prelude
- Add Orange, Brown and Purple
- Add `Polyline`
- Add `Graphics::draw_arc`

### Version 0.9.3
- Update shapes library

### Version 0.9.2
- Update shapes library

### Version 0.9.1
- Fix bug with ShapeCollection::with_rotate

### Version 0.9.0
- Add draw_rect, draw_circle, etc methods on Graphics
- Change renderable to require a generic param so it can impl'd outside this crate
- Refactor shape collection so now treated as a single renderable (rather than a data storage object)

### Version 0.8.3
- Update shapes lib (0.1.6)
- Add from Coord for TextPos
- Fix text positioning
- Add TextPos::px(Coord)
- Rename TextPos::to_px() to to_coord()
- Rename Graphics::draw_at() to draw_offset() so the name accurately describes its behaviour

### Version 0.8.2
- Add DrawOffset for RenderableImage so it can be drawn from the top left or center
- Remove now unnecessary mut in copy_to_image()

### Version 0.8.1
- Add wrapper RenderableImage which contains an image and position so that an image can be rendered like a shape

### Version 0.8.0
- Update shapes lib (0.1.5)
- Fix bug in Drawable::with_rotation()
- Add Drawable::with_scale(), Drawable::with_scale_around and Drawable::with_rotation_around()
- Add with_scale(), with_scale_around(), with_rotation(), with_rotation_around(), with_translate() and with_move() to ShapeCollection and AutoShapeCollection
- Make ShapeCollection and AutoShapeCollection implement Renderable
- Add Positioning to TextFormat
- Remove all From for TextPos, instead use TextPos::px and TextPos::cr

### Version 0.7.1
- Update shapes lib (0.1.4)

### Version 0.7.0
- Add Ellipse

### Version 0.6.4
- Fix bug in AutoShapeCollection where the same id would apply to multiple shapes

### Version 0.6.3
- Fix method access bug

### Version 0.6.2
- Add shape collection for batch rendering

### Version 0.6.1
- Update shapes lib (0.1.2)
- Expose some Shape methods on Drawable<Shape>

### Version 0.6.0
- Extract shapes to shape library
- Add yen, dollar, cent, and currency characters
- Allow custom character size

### Version 0.5.0
- Add From(num,num) for TextPos
  - signed to TextPos::Px
  - unsigned to TextPos::ColRow
- Add Text renderable
- Add TextFormat
- Add WrappingStrategy (provides various options for wrapping text - can only handle ASCII English)
- Rename normal to large (as it's double the size of small)
- Move text to it's own module
- Add ellipsis, power, backslash, at, curly braces, back tick, tilde and pound characters
- Add normal text size
- Make mint optional

### Version 0.4.1
- Add polygon shape

### Version 0.4.0
- Change shapes from structs to enum as due to Rust trait limitations which meant working with Shape and Renderable was very awkward and annoying.
  - Unfortunately this means the shapes lose their unique methods (line.len(), rect.width(), etc) and shape conversions had to be removed
- Add Triangle shape
- Add contains for every shape

### Version 0.3.0
- Replace draw_rect, draw_line, draw_circle, etc with `draw` and `draw_at` which take `Renderable` and `DrawType`
  - Renderables
    - Circle
    - Rect
    - Line
  - DrawType
    - Filled
    - Stroke
- Everything takes `Coord` instead of mint::Point2
  - Coord implements to/from Point2, (isize,isize), etc
- Also `TextPos::Coord` renamed to `ColRow` to make params clearer
- Lerp moved

### Version 0.2.1
- Fix `contains()` so container and point types don't have to be the same
- Remove `self` param from `get_px_for_char` and `get_text_size` as they didn't use it
- Add `is_on_screen` method to check if a point will be on screen after translation
- Fix issue where alpha wasn't applied when setting pixels

### Version 0.2.0
- Remove const size for Graphics as it makes using it too awkward, instead graphics will just use a slice

### Version 0.1.0
- Initial release