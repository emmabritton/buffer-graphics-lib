#[cfg(feature = "embedded")]
pub mod embedded_graphics_lib;

#[cfg(feature = "notosans")]
pub mod noto_sans;

#[cfg(feature = "embedded")]
pub use embedded_graphics_lib::*;

#[cfg(feature = "notosans")]
pub use noto_sans::*;
