use graphics_shapes::coord::Coord;

#[macro_export]
macro_rules! renderable {
    ($name:ident, $struct_name:ty, $render:expr) => {
        #[derive(Debug)]
        pub struct $name {
            xy: Coord,
            item: $struct_name,
        }

        impl $name {
            pub fn set_position<P: Into<Coord>>(&mut self, new_position: P) {
                self.xy = new_position.into();
            }

            pub fn update_position<P: Into<Coord>>(&mut self, delta: P) {
                self.xy = self.xy + delta.into();
            }
        }

        impl Renderable<$struct_name> for $name {
            fn render(&self, graphics: &mut Graphics) {
                #[allow(clippy::redundant_closure_call)]
                graphics.with_translate(self.xy, |g| $render(g, &self.item));
            }
        }
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawOffset {
    TopLeft,
    Center,
    Custom(Coord),
}

#[macro_export]
macro_rules! sized_renderable {
    ($name:ident, $struct_name:ty, $size:expr, $render:expr) => {
        #[derive(Debug)]
        pub struct $name {
            xy: Coord,
            offset: $crate::renderable_macros::DrawOffset,
            item: $struct_name,
        }

        impl $name {
            pub fn new<P: Into<Coord>>(
                item: $struct_name,
                pos: P,
                draw_offset: $crate::renderable_macros::DrawOffset,
            ) -> Self {
                $name {
                    xy: pos.into(),
                    offset: draw_offset,
                    item,
                }
            }
        }

        impl $name {
            pub fn set_position<P: Into<Coord>>(&mut self, new_position: P) {
                self.xy = new_position.into();
            }

            pub fn update_position<P: Into<Coord>>(&mut self, delta: P) {
                self.xy = self.xy + delta.into();
            }

            pub fn set_offset(&mut self, offset: $crate::renderable_macros::DrawOffset) {
                self.offset = offset;
            }
        }

        impl Renderable<$struct_name> for $name {
            fn render(&self, graphics: &mut Graphics) {
                use std::ops::Neg;

                #[allow(clippy::redundant_closure_call)]
                let (width, height): (usize, usize) = $size(&self.item);
                let offset = match self.offset {
                    $crate::renderable_macros::DrawOffset::TopLeft => (0, 0).into(),
                    $crate::renderable_macros::DrawOffset::Center => {
                        (((width / 2) as isize).neg(), ((height / 2) as isize).neg()).into()
                    }
                    $crate::renderable_macros::DrawOffset::Custom(coord) => coord,
                };

                #[allow(clippy::redundant_closure_call)]
                graphics.with_translate(self.xy + offset, |g| $render(g, &self.item));
            }
        }
    };
}
