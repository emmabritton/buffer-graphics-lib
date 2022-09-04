# Changelog

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