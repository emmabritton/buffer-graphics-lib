use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PolylineError {
    PolylineAlreadyClosed,
    InvalidPolyline,
}
impl Display for PolylineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PolylineError::PolylineAlreadyClosed => write!(f, "Polyline has already been closed"),
            PolylineError::InvalidPolyline => write!(f, "Polyline has invalid segments"),
        }
    }
}

impl Error for PolylineError {}
