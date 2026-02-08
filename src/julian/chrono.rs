use super::*;
use ::chrono::{NaiveDate, NaiveTime};

const NOON_TIME: NaiveTime = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

/// Calculates the time at which the sun is at its highest altitude and returns
/// the time as a Julian day.
pub(crate) fn mean_solar_noon(lon: f64, date: NaiveDate) -> f64 {
    unix_to_julian(date.and_time(NOON_TIME).and_utc().timestamp()) - lon / 360.
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    fn test_solar_noon() {
        assert_eq!(
            super::mean_solar_noon(0., NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            2440588.
        );
    }
}
