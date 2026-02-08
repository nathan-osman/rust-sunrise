use super::*;
use ::jiff::{civil::Date, tz::TimeZone};

/// Calculates the time at which the sun is at its highest altitude and returns
/// the time as a Julian day.
pub(crate) fn mean_solar_noon(lon: f64, date: Date) -> f64 {
    unix_to_julian(
        date.at(12, 0, 0, 0)
            .to_zoned(TimeZone::UTC)
            .expect("Unable to convert to UTC timestamp")
            .timestamp()
            .as_second(),
    ) - lon / 360.
}

#[cfg(test)]
mod tests {
    use jiff::civil::Date;

    #[test]
    fn test_solar_noon() {
        assert_eq!(
            super::mean_solar_noon(0., Date::constant(1970, 1, 1)),
            2440588.
        );
    }
}
