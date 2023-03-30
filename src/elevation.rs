// The MIT License (MIT)
//
// Copyright (c) 2018 Nathan Osman
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use anomaly::solar_mean_anomaly;
use center::equation_of_center;
use declination::declination;
use julian::julian_to_unix;
use longitude::ecliptic_longitude;
use noon::mean_solar_noon;
use transit::solar_transit;
use std::f64;

/// Calculates the times of day when the sun is at a given elevation
pub fn time_of_elevation(
    latitude: f64,
    longitude: f64,
    elevation: f64,
    year: i32,
    month: u32,
    day: u32,
) -> (i64, i64) {
    let day: f64 = mean_solar_noon(longitude, year, month, day);
    let solar_anomaly: f64 = solar_mean_anomaly(day);
    let equation_of_center: f64 = equation_of_center(solar_anomaly);
    let ecliptic_longitude: f64 = ecliptic_longitude(solar_anomaly, equation_of_center, day);
    let solar_transit: f64 = solar_transit(day, solar_anomaly, ecliptic_longitude);
    let declination: f64 = declination(ecliptic_longitude);
    let numerator: f64 = f64::sin(elevation * ::DEGREE) - (f64::sin(latitude * ::DEGREE) * f64::sin(declination * ::DEGREE));
    let denominator: f64 = f64::cos(latitude * ::DEGREE) * f64::cos(declination * ::DEGREE);
    let hour_angle: f64 = f64::acos(numerator / denominator);
    let frac = hour_angle / (f64::consts::TAU);
    let morning = solar_transit - frac;
    let evening = solar_transit + frac;
    if hour_angle.is_nan() {
        return (0, 0);
    }

    (
        julian_to_unix(morning),
        julian_to_unix(evening),
    )
}

#[cfg(test)]
mod tests {

    const SUNRISE_ELEVATION: f64= -50.0 / 60.0;

    #[test]
    fn test_prime_meridian() {
        let dates = super::time_of_elevation(43.65, -79.38, SUNRISE_ELEVATION, 2000, 1, 1);
        assert_unix(dates, (946731060, 946763436))
    }

    fn assert_unix((actual_first,actual_last): (i64, i64), (expected_first, expected_last): (i64, i64)) {
        assert!(i64::abs(actual_first-expected_first) <= 2);
        assert!(i64::abs(actual_last-expected_last) <= 2);

    }
}