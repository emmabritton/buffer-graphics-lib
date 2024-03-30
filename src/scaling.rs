use crate::image::Image;

pub(crate) fn scale_nearest_neighbor(image: &Image, x_scale: usize, y_scale: usize) -> Image {
    let new_width = image.width() * x_scale;
    let new_height = image.height() * y_scale;
    let mut new_image = Image::new_blank(new_width, new_height);
    let x_scale = 1.0 / x_scale as f32;
    let y_scale = 1.0 / y_scale as f32;
    for y in 0..new_height {
        for x in 0..new_width {
            let px = (x as f32 * x_scale).floor() as usize;
            let py = (y as f32 * y_scale).floor() as usize;
            new_image.set_pixel(x, y, image.get_pixel(px, py));
        }
    }
    new_image
}

pub(crate) fn scale_epx(image: &Image) -> Image {
    let new_width = image.width() * 2;
    let new_height = image.height() * 2;
    let mut new_image = Image::new_blank(new_width, new_height);
    for x in 0..image.width() {
        for y in 0..image.height() {
            let mut p1 = image.get_pixel(x, y);
            let mut p2 = p1;
            let mut p3 = p1;
            let mut p4 = p1;
            let a = image.get_pixel(x, if y > 0 { y - 1 } else { y });
            let c = image.get_pixel(if x > 0 { x - 1 } else { x }, y);
            let b = image.get_pixel(if x < image.width() - 2 { x + 1 } else { x }, y);
            let d = image.get_pixel(x, if y < image.height() - 2 { y + 1 } else { y });

            if c == a && c != d && a != b {
                p1 = a
            }
            if a == b && a != c && b != d {
                p2 = b
            }
            if d == c && d != b && c != a {
                p3 = c
            }
            if b == d && b != a && d != c {
                p4 = d
            }

            let nx = x * 2;
            let ny = y * 2;
            new_image.set_pixel(nx, ny, p1);
            new_image.set_pixel(nx + 1, ny, p2);
            new_image.set_pixel(nx, ny + 1, p3);
            new_image.set_pixel(nx + 1, ny + 1, p4);
        }
    }
    new_image
}
