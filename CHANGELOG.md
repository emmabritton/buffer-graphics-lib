# Changelog

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