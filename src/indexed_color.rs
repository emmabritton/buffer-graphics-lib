use ici_files::image::IndexedImage;
use ici_files::prelude::AnimatedIndexedImage;
use ici_files::IciColor;

pub trait ChangeColors {
    /// De/saturate color by percentage
    /// Negative amount increases saturation
    /// So `-0.1` is 10% more saturated
    fn with_saturate(&self, amount: f32) -> Self;

    /// Increase saturation by 10%
    fn saturate(&self) -> Self
    where
        Self: Sized,
    {
        self.with_saturate(-0.1)
    }

    /// Decrease saturation by 10%
    fn desaturate(&self) -> Self
    where
        Self: Sized,
    {
        self.with_saturate(0.1)
    }

    /// Change brightness to `amount`
    /// So `1.1` is 10% brighter
    fn with_brightness(&self, amount: f32) -> Self;

    fn lighten(&self) -> Self
    where
        Self: Sized,
    {
        self.with_brightness(1.1)
    }

    fn darken(&self) -> Self
    where
        Self: Sized,
    {
        self.with_brightness(0.9)
    }
}

impl ChangeColors for IciColor {
    fn with_saturate(&self, amount: f32) -> Self {
        let mut rgba = [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        ];
        let lum = 0.2989 * rgba[0] + 0.5870 * rgba[1] + 0.1140 * rgba[2];
        rgba[0] = rgba[0] + amount * (lum - rgba[0]);
        rgba[1] = rgba[1] + amount * (lum - rgba[1]);
        rgba[2] = rgba[2] + amount * (lum - rgba[2]);
        IciColor::new(
            (rgba[0] * 255.0) as u8,
            (rgba[1] * 255.0) as u8,
            (rgba[2] * 255.0) as u8,
            self.a,
        )
    }

    fn with_brightness(&self, amount: f32) -> Self {
        IciColor::new(
            ((((self.r as f32) / 255.0) * amount).min(1.0).max(0.0) * 255.0) as u8,
            ((((self.g as f32) / 255.0) * amount).min(1.0).max(0.0) * 255.0) as u8,
            ((((self.b as f32) / 255.0) * amount).min(1.0).max(0.0) * 255.0) as u8,
            self.a,
        )
    }
}

impl ChangeColors for Vec<IciColor> {
    fn with_saturate(&self, amount: f32) -> Self {
        let mut colors = vec![];
        for color in self {
            colors.push(color.with_saturate(amount));
        }
        colors
    }

    fn with_brightness(&self, amount: f32) -> Self {
        let mut colors = vec![];
        for color in self {
            colors.push(color.with_brightness(amount));
        }
        colors
    }
}

impl ChangeColors for IndexedImage {
    fn with_saturate(&self, amount: f32) -> Self {
        let mut image = self.clone();
        image
            .set_palette(&image.get_palette().to_vec().with_saturate(amount))
            .expect("Color disappeared when changing saturation, please raise an issue on GitHub buffer-graphics-lib");
        image
    }

    fn with_brightness(&self, amount: f32) -> Self {
        let mut image = self.clone();
        image
            .set_palette(&image.get_palette().to_vec().with_brightness(amount))
            .expect("Color disappeared when changing saturation, please raise an issue on GitHub buffer-graphics-lib");
        image
    }
}

impl ChangeColors for AnimatedIndexedImage {
    fn with_saturate(&self, amount: f32) -> Self {
        let mut image = self.clone();
        image
            .set_palette(&image.get_palette().to_vec().with_saturate(amount))
            .expect("Color disappeared when changing saturation, please raise an issue on GitHub buffer-graphics-lib");
        image
    }

    fn with_brightness(&self, amount: f32) -> Self {
        let mut image = self.clone();
        image
            .set_palette(&image.get_palette().to_vec().with_brightness(amount))
            .expect("Color disappeared when changing saturation, please raise an issue on GitHub buffer-graphics-lib");
        image
    }
}
