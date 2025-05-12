use core::fmt::{self, Display, Formatter};

/// A valid pair of geographic coordinates.
///
/// See <https://en.wikipedia.org/wiki/Geographic_coordinate_system>
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Coordinates {
    lat: f64,
    lon: f64,
}

impl Coordinates {
    /// Validate a pair of latitude / longitude (in degrees).
    ///
    /// Return `None` if values are out of range (`abs(lat) > 90` or
    /// `abs(lon) > 180`).
    pub const fn new(lat: f64, lon: f64) -> Option<Self> {
        if lat.is_nan() || lon.is_nan() || lat < -90.0 || lat > 90.0 || lon < -180.0 || lon > 180.0
        {
            return None;
        }

        Some(Self { lat, lon })
    }

    /// Get latitude component.
    pub fn lat(&self) -> f64 {
        self.lat
    }

    /// Get longitude component.
    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.lat, self.lon)
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinates::Coordinates;
    use approx::assert_relative_eq;

    #[test]
    fn invalid() {
        assert!(Coordinates::new(f64::NAN, 10.0).is_none());
        assert!(Coordinates::new(10.0, f64::NAN).is_none());
        assert!(Coordinates::new(-120.0, 0.0).is_none());
        assert!(Coordinates::new(0.0, -240.0).is_none());
    }

    #[test]
    fn extract() {
        let coord = Coordinates::new(10.0, 36.35).unwrap();
        assert_relative_eq!(coord.lat(), 10.0);
        assert_relative_eq!(coord.lon(), 36.35);
    }

    #[test]
    #[cfg(feature = "std")]
    fn display() {
        use std::string::ToString;
        let coord = Coordinates::new(10.0, 36.35).unwrap();
        assert_eq!(coord.to_string(), "(10.0, 36.35)");
    }
}
