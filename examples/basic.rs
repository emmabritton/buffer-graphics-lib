use std::fs;
use anyhow::Result;
use graphics_shapes::shape_box::ShapeBox;
use buffer_graphics_lib::prelude::*;
use buffer_graphics_lib::renderable_image::RenderableImage;
use buffer_graphics_lib::renderable_macros::DrawOffset;
use buffer_graphics_lib::text::PixelFont::*;

fn main() -> Result<()> {
    let mut buffer = Graphics::create_buffer(400, 400);
    let mut graphics = Graphics::new(&mut buffer, 400, 400)?;

    draw_some_text(&mut graphics);
    draw_some_shapes(&mut graphics);
    draw_to_image(&mut graphics);
    draw_some_images(&mut graphics)?;
    draw_reversed(&mut graphics);

    Ok(())
}

fn draw_some_text(graphics: &mut Graphics) {
    //Text can be drawn in several ways

    //1 directly using `draw_text`
    //this is the least efficient way
    graphics.draw_text("This is some text", TextPos::Px(20, 20), (WHITE, Standard6x7));
    //draws the string "This is some text" at pixel 20x20 using font Standard6x7 in white

    //2 using `draw` with pre measured text
    let text = Text::new("More example text", TextPos::ColRow(6, 5), (RED, Standard4x5, WrappingStrategy::SpaceBeforeCol(8)));
    graphics.draw(&text);
    //draws the string "More example text" at column 6, row 5 using font Standard4x5 in red, wrapping at the first space before column 14 (6 + 8)
    //columns and rows are based on the font size so ColRow(2,2) for Standard6x7 and Standard4x5 would be in different locations on screen

    //The last param of text is `TextFormat`
    //it can be created with `TextFormat::new(...)` or with
    //(Color)
    //(Color, PixelFont)
    //(Color, PixelFont, Positioning)
    //(Color, PixelFont, WrappingStrategy)
    //(Color, PixelFont, WrappingStrategy, Positioning)
    //(Color, PixelFont, WrappingStrategy, f32 [line height])
    //(Color, PixelFont, WrappingStrategy, f32 [line height], f32 [char width])
    //(Color, PixelFont, WrappingStrategy, f32 [line height], f32 [char width], Positioning)
}

fn draw_some_shapes(graphics: &mut Graphics) {
    //Shapes can also be drawn in several ways

    //1 directly with `draw_rect`, `draw_circle`
    graphics.draw_triangle(Triangle::new((40, 40), (80, 40), (80, 40)), stroke(YELLOW));

    //2 using `Drawable`, stores the color with the shape
    let rect = Rect::new((10, 10), (80, 80));
    let drawable = Drawable::from_obj(rect, fill(ORANGE));
    graphics.draw(&drawable);

    //3 `ShapeBox`, these are used so shapes can be treated generically
    let shape_box = ShapeBox::from(Circle::new((60, 60), 40));
    let drawable_box = Drawable::from_obj(shape_box, stroke(GREEN));
    graphics.draw(&drawable_box);

    //3 in bulk using a `ShapeCollection`
    let mut collection = ShapeCollection::default();
    InsertShape::insert(&mut collection, 0, Line::new((14, 12), (78, 23)), stroke(MID_GRAY));
    let drawable = Drawable::from_obj(ShapeBox::from(Circle::new((14, 12), 10)), stroke(BLUE));
    InsertDrawable::insert(&mut collection, 0, drawable);
    graphics.draw(&collection);
}

#[allow(unused_variables)]
fn draw_to_image(graphics: &mut Graphics) {
    //The canvas can be written to `Image` or `IndexedImage`

    let image = graphics.copy_to_image();
    //this can then be written to disk or drawn back on the buffer
}

fn draw_some_images(graphics: &mut Graphics) -> Result<()> {
    //Image are normal images such as png or jpeg
    //IndexedImage are ici (from `ici-files`)

    //1 Images
    let png = open_image("examples/sample.png")?;
    graphics.draw_image((40,40), &png);

    //2 IndexedImages
    let (ici,_) = IndexedImage::from_file_contents(&fs::read("examples/sample.ici")?)?;
    graphics.draw_indexed_image((50,50), &ici);

    //3 These can also be stored as `RenderableImage` or `RenderableIndexedImage`
    let image = RenderableImage::new(png, (67,12), DrawOffset::TopLeft);
    image.render(graphics);

    Ok(())
}

fn draw_reversed(graphics: &mut Graphics) {
    //Most things can also be drawn by passing them `Graphics` instead

    let drawable = Drawable::from_obj(Line::new((0, 0), (100, 100)), stroke(PURPLE));
    drawable.render(graphics);

    let text = Text::new("SAMPLE TEXT", TextPos::px(coord!(60,60)), (MAGENTA, Limited3x5));
    text.render(graphics);
}