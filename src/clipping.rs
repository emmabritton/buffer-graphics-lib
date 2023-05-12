use crate::clipping::ClipMode::*;
use graphics_shapes::coord::Coord;
use graphics_shapes::prelude::{Circle, Rect};
use graphics_shapes::Shape;
use log::error;
use std::mem::swap;

#[derive(Clone, Debug)]
enum ClipShape {
    Box(Rect),
    Round(Circle),
}

impl ClipShape {
    pub fn contains(&self, xy: (isize, isize)) -> bool {
        match self {
            ClipShape::Box(rect) => rect.contains(Coord::from(xy)),
            ClipShape::Round(circle) => circle.contains(Coord::from(xy)),
        }
    }
}

#[derive(Clone, Debug)]
enum ClipElement {
    Add(ClipShape),
    Remove(ClipShape),
}

#[derive(Clone, Debug)]
enum ClipMode {
    Nothing,
    Simple(ClipShape),
    Complex(Vec<ClipElement>),
    Custom(Vec<bool>),
}

/// Clip has four modes:
/// * Nothing - All pixels are valid
/// * Simple - Only pixels in the shape (rect or circle) are valid
/// * Custom - User provides a list of which pixels are valid
/// * Complex - A series of shapes adding and removing clip area
///
/// Complex starts with all pixels being valid
/// * use `add_*` to decrease the valid area
/// * use `remove_*` to increase the valid area
/// The last shape to touch a pixel determines it's validity
///
/// With complex mode a list of valid pixels is stored internally and each time the complex clip is updated the valid list is updated as well, if you're making a bulk edit call `set_auto_build_map(false)` first
pub struct Clip {
    width: usize,
    height: usize,
    mode: ClipMode,
    old_mode: ClipMode,
    valid_pixel_map: Option<Vec<bool>>,
    old_valid_pixel_map: Option<Vec<bool>>,
    auto_build_map: bool,
}

impl Clip {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            mode: Nothing,
            old_mode: Nothing,
            valid_pixel_map: None,
            old_valid_pixel_map: None,
            auto_build_map: true,
        }
    }
}

impl Clip {
    pub fn is_nothing(&self) -> bool {
        matches!(&self.mode, Nothing)
    }

    pub fn is_simple(&self) -> bool {
        matches!(&self.mode, Simple(_))
    }

    pub fn is_complex(&self) -> bool {
        matches!(&self.mode, Complex(_))
    }

    pub fn is_custom(&self) -> bool {
        matches!(&self.mode, Custom(_))
    }

    pub fn set_auto_build_map(&mut self, auto_build_map: bool) {
        self.auto_build_map = auto_build_map;
    }
}

impl Clip {
    pub fn is_valid(&self, xy: (isize, isize)) -> bool {
        let i = xy.0 + xy.1 * (self.width as isize);
        let u = i.max(0) as usize;
        match &self.mode {
            Nothing => true,
            Simple(shape) => shape.contains(xy),
            Complex(_) => {
                if let Some(map) = &self.valid_pixel_map {
                    map[u]
                } else {
                    error!("Using complex clip but pixel map hasn't been built");
                    true
                }
            }
            Custom(map) => map[u],
        }
    }
}

impl Clip {
    fn store_current_mode(&mut self) {
        swap(&mut self.old_mode, &mut self.mode);
        swap(&mut self.old_valid_pixel_map, &mut self.valid_pixel_map);
        self.valid_pixel_map = None;
    }
}

impl Clip {
    /// Clears the clip so all pixels can be drawn to
    pub fn set_all_valid(&mut self) {
        self.store_current_mode();
        self.mode = Nothing;
    }

    /// Set the valid pixels to `rect`
    pub fn set_valid_rect(&mut self, rect: Rect) {
        self.store_current_mode();
        self.mode = Simple(ClipShape::Box(rect));
    }

    /// Set the valid pixels to `circle`
    pub fn set_valid_circle(&mut self, circle: Circle) {
        self.store_current_mode();
        self.mode = Simple(ClipShape::Round(circle));
    }

    /// Set the valid pixels to `pixel_map`
    pub fn custom(&mut self, pixel_map: Vec<bool>) {
        self.store_current_mode();
        self.mode = Custom(pixel_map);
    }

    pub fn get_pixel_map(&mut self) -> Vec<bool> {
        match &self.mode {
            Simple(_) | Nothing => self.build_pixel_map(),
            Complex(_) => {
                if let Some(map) = &self.valid_pixel_map {
                    map.clone()
                } else {
                    self.update_pixel_map();
                    self.valid_pixel_map.as_ref().unwrap().clone()
                }
            }
            Custom(map) => map.clone(),
        }
    }
}

impl Clip {
    fn swap_to_complex(&mut self) {
        if !self.is_complex() {
            self.store_current_mode();
            self.mode = Complex(vec![]);
        }
    }

    fn add(&mut self, element: ClipElement) {
        self.swap_to_complex();
        if let Complex(list) = &mut self.mode {
            list.push(element);
        }
        if self.auto_build_map {
            self.update_pixel_map();
        }
    }

    /// Set the mode to `complex` (clearing any other mode)
    /// Set any pixels in `rect` to valid
    pub fn add_rect(&mut self, rect: Rect) {
        self.add(ClipElement::Add(ClipShape::Box(rect)));
    }

    /// Set the mode to `complex` (clearing any other mode)
    /// Set any pixels in `rect` to invalid
    pub fn remove_rect(&mut self, rect: Rect) {
        self.add(ClipElement::Remove(ClipShape::Box(rect)));
    }

    /// Set the mode to `complex` (clearing any other mode)
    /// Set any pixels in `circle` to valid
    pub fn add_circle(&mut self, circle: Circle) {
        self.add(ClipElement::Add(ClipShape::Round(circle)));
    }

    /// Set the mode to `complex` (clearing any other mode)
    /// Set any pixels in `circle` to invalid
    pub fn remove_circle(&mut self, circle: Circle) {
        self.add(ClipElement::Remove(ClipShape::Round(circle)));
    }
}

impl Clip {
    pub fn update_pixel_map(&mut self) {
        if self.is_complex() {
            self.valid_pixel_map = Some(self.build_pixel_map())
        } else {
            self.valid_pixel_map = None;
        }
    }

    fn build_pixel_map(&self) -> Vec<bool> {
        match &self.mode {
            Nothing => vec![true; self.width * self.height],
            Simple(shape) => self.build_simple_map(shape),
            Complex(elements) => self.build_complex_map(elements),
            Custom(map) => map.clone(),
        }
    }

    fn build_simple_map(&self, shape: &ClipShape) -> Vec<bool> {
        let mut output = vec![false; self.width * self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                let contains = shape.contains((x as isize, y as isize));
                if contains {
                    output[i] = true;
                }
            }
        }
        output
    }

    fn build_complex_map(&self, elements: &[ClipElement]) -> Vec<bool> {
        let mut output = vec![true; self.width * self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                let mut valid = true;
                for element in elements {
                    match element {
                        ClipElement::Add(shape) => {
                            if shape.contains((x as isize, y as isize)) {
                                valid = true;
                            }
                        }
                        ClipElement::Remove(shape) => {
                            if shape.contains((x as isize, y as isize)) {
                                valid = false;
                            }
                        }
                    }
                }
                let i = x + y * self.width;
                if i >= output.len() {
                    break;
                }
                output[x + y * self.width] = valid;
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use crate::clipping::Clip;
    use graphics_shapes::rect::Rect;

    #[test]
    fn check_all_pixels_valid_for_none() {
        let mut clip = Clip::new(4, 4);
        assert_eq!(clip.get_pixel_map(), vec![true; 16]);
    }

    #[test]
    fn check_pixels_valid_for_square() {
        let mut clip = Clip::new(4, 4);
        clip.set_valid_rect(Rect::new((1, 1), (2, 2)));
        let expected = vec![
            false, false, false, false, false, true, true, false, false, true, true, false, false,
            false, false, false,
        ];
        assert_eq!(clip.get_pixel_map(), expected);
    }

    #[test]
    fn check_pixels_split_horz() {
        let mut clip = Clip::new(4, 4);
        clip.set_valid_rect(Rect::new((2, 0), (3, 3)));
        let expected = vec![
            false, false, true, true, false, false, true, true, false, false, true, true, false,
            false, true, true,
        ];
        assert_eq!(clip.get_pixel_map(), expected);
    }

    #[test]
    fn complex() {
        let mut clip = Clip::new(4, 4);

        clip.add_rect(Rect::new((0, 0), (3, 3))); //set all pixels to valid
        let expected = vec![true; 16];
        assert_eq!(clip.get_pixel_map(), expected);

        clip.remove_rect(Rect::new((2, 0), (3, 3))); //set right hand side to invalid
        let expected = vec![
            true, true, false, false, true, true, false, false, true, true, false, false, true,
            true, false, false,
        ];
        assert_eq!(clip.get_pixel_map(), expected);

        clip.add_rect(Rect::new((3, 0), (3, 3))); //set last column to valid
        let expected = vec![
            true, true, false, true, true, true, false, true, true, true, false, true, true, true,
            false, true,
        ];
        assert_eq!(clip.get_pixel_map(), expected);
    }
}
