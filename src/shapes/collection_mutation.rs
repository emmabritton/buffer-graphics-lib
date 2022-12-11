use crate::drawable::Drawable;
use crate::shapes::collection::AutoShapeCollection;
use crate::shapes::collection::ShapeCollection;
use crate::shapes::CreateDrawable;
use graphics_shapes::coord::Coord;
use graphics_shapes::Shape;
use std::collections::HashMap;
use std::hash::Hash;

fn with_translation<K: Eq + PartialEq + Hash + Clone, P: Into<Coord>, T: Shape + Clone>(
    shapes: &HashMap<K, Drawable<T>>,
    delta: P,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    let delta = delta.into();
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_translation(delta)))
        .collect()
}

fn with_move<K: Eq + PartialEq + Hash + Clone, P: Into<Coord>, T: Shape + Clone>(
    shapes: &HashMap<K, Drawable<T>>,
    delta: P,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    let delta = delta.into();
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_move(delta)))
        .collect()
}

fn with_scale<K: Eq + PartialEq + Hash + Clone, T: Shape + Clone>(
    shapes: &HashMap<K, Drawable<T>>,
    delta: f32,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_scale(delta)))
        .collect()
}

fn with_rotation<K: Eq + PartialEq + Hash + Clone, T: Shape + Clone>(
    shapes: &HashMap<K, Drawable<T>>,
    degrees: isize,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_rotation(degrees)))
        .collect()
}

fn with_scale_around<K: Eq + PartialEq + Hash + Clone, T: Shape + Clone, P: Into<Coord>>(
    shapes: &HashMap<K, Drawable<T>>,
    delta: f32,
    center: P,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    let center = center.into();
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_scale_around(delta, center)))
        .collect()
}

fn with_rotation_around<K: Eq + PartialEq + Hash + Clone, T: Shape + Clone, P: Into<Coord>>(
    shapes: &HashMap<K, Drawable<T>>,
    delta: isize,
    center: P,
) -> HashMap<K, Drawable<T>>
where
    Drawable<T>: CreateDrawable<T>,
{
    let center = center.into();
    shapes
        .iter()
        .map(|(key, shape)| (key.clone(), shape.with_rotation_around(delta, center)))
        .collect()
}

macro_rules! impl_coord_mutator {
    ($list_method: ident, $param_name: ident) => {
        impl<K: Eq + PartialEq + Hash + Clone> ShapeCollection<K> {
            pub fn $list_method<P: Into<Coord>>(&self, $param_name: P) -> Self {
                let $param_name = $param_name.into();
                Self {
                    rects: $list_method(&self.rects(), $param_name),
                    circles: $list_method(&self.circles(), $param_name),
                    triangles: $list_method(&self.triangles(), $param_name),
                    lines: $list_method(&self.lines(), $param_name),
                    polygons: $list_method(&self.polygons(), $param_name),
                    ellipses: $list_method(&self.ellipses(), $param_name),
                }
            }
        }
    };
}

macro_rules! impl_number_mutator {
    ($list_method: ident, $param_name: ident, $param_type: ty) => {
        impl<K: Eq + PartialEq + Hash + Clone> ShapeCollection<K> {
            pub fn $list_method(&self, $param_name: $param_type) -> Self {
                Self {
                    rects: $list_method(&self.rects(), $param_name),
                    circles: $list_method(&self.circles(), $param_name),
                    triangles: $list_method(&self.triangles(), $param_name),
                    lines: $list_method(&self.lines(), $param_name),
                    polygons: $list_method(&self.polygons(), $param_name),
                    ellipses: $list_method(&self.ellipses(), $param_name),
                }
            }
        }
    };
}

macro_rules! impl_number_coord_mutator {
    ($list_method: ident, $param_name: ident, $param_type: ty) => {
        impl<K: Eq + PartialEq + Hash + Clone> ShapeCollection<K> {
            pub fn $list_method<P: Into<Coord>>(
                &self,
                $param_name: $param_type,
                center: P,
            ) -> Self {
                let center = center.into();
                Self {
                    rects: $list_method(&self.rects(), $param_name, center),
                    circles: $list_method(&self.circles(), $param_name, center),
                    triangles: $list_method(&self.triangles(), $param_name, center),
                    lines: $list_method(&self.lines(), $param_name, center),
                    polygons: $list_method(&self.polygons(), $param_name, center),
                    ellipses: $list_method(&self.ellipses(), $param_name, center),
                }
            }
        }
    };
}

impl_coord_mutator!(with_translation, delta);
impl_coord_mutator!(with_move, xy);
impl_number_mutator!(with_scale, factor, f32);
impl_number_mutator!(with_rotation, degrees, isize);
impl_number_coord_mutator!(with_rotation_around, degrees, isize);
impl_number_coord_mutator!(with_scale_around, factor, f32);

macro_rules! impl_coord_mutator_auto {
    ($list_method: ident, $param_name: ident) => {
        impl AutoShapeCollection {
            pub fn $list_method<P: Into<Coord>>(&self, $param_name: P) -> Self {
                let $param_name = $param_name.into();
                Self {
                    next_id: self.next_id,
                    rects: $list_method(&self.rects(), $param_name),
                    circles: $list_method(&self.circles(), $param_name),
                    triangles: $list_method(&self.triangles(), $param_name),
                    lines: $list_method(&self.lines(), $param_name),
                    polygons: $list_method(&self.polygons(), $param_name),
                    ellipses: $list_method(&self.ellipses(), $param_name),
                }
            }
        }
    };
}

macro_rules! impl_number_mutator_auto {
    ($list_method: ident, $param_name: ident, $param_type: ty) => {
        impl AutoShapeCollection {
            pub fn $list_method(&self, $param_name: $param_type) -> Self {
                Self {
                    next_id: self.next_id,
                    rects: $list_method(&self.rects(), $param_name),
                    circles: $list_method(&self.circles(), $param_name),
                    triangles: $list_method(&self.triangles(), $param_name),
                    lines: $list_method(&self.lines(), $param_name),
                    polygons: $list_method(&self.polygons(), $param_name),
                    ellipses: $list_method(&self.ellipses(), $param_name),
                }
            }
        }
    };
}

macro_rules! impl_number_coord_mutator_auto {
    ($list_method: ident, $param_name: ident, $param_type: ty) => {
        impl AutoShapeCollection {
            pub fn $list_method<P: Into<Coord>>(
                &self,
                $param_name: $param_type,
                center: P,
            ) -> Self {
                let center = center.into();
                Self {
                    next_id: self.next_id,
                    rects: $list_method(&self.rects(), $param_name, center),
                    circles: $list_method(&self.circles(), $param_name, center),
                    triangles: $list_method(&self.triangles(), $param_name, center),
                    lines: $list_method(&self.lines(), $param_name, center),
                    polygons: $list_method(&self.polygons(), $param_name, center),
                    ellipses: $list_method(&self.ellipses(), $param_name, center),
                }
            }
        }
    };
}

impl_coord_mutator_auto!(with_translation, delta);
impl_coord_mutator_auto!(with_move, xy);
impl_number_mutator_auto!(with_scale, factor, f32);
impl_number_mutator_auto!(with_rotation, degrees, isize);
impl_number_coord_mutator_auto!(with_rotation_around, degrees, isize);
impl_number_coord_mutator_auto!(with_scale_around, factor, f32);
