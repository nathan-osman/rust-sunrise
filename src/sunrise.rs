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
use hourangle::hour_angle;
use julian::julian_to_unix;
use longitude::ecliptic_longitude;
use noon::mean_solar_noon;
use transit::solar_transit;

/// Calculates the sunrise and sunset times for the given location and date.
pub fn sunrise_sunset(
    latitude: f64,
    longitude: f64,
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
    let hour_angle: f64 = hour_angle(latitude, declination);
    let frac: f64 = hour_angle / 360.;
    (
        julian_to_unix(solar_transit - frac),
        julian_to_unix(solar_transit + frac),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prime_meridian() {
        assert_eq!(super::sunrise_sunset(0., 0., 1970, 1, 1), (21594, 65227),)
    }
}
