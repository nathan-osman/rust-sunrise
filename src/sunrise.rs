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

use std::f64::consts::PI;

use crate::anomaly::solar_mean_anomaly;
use crate::center::equation_of_center;
use crate::declination::declination;
use crate::event::SolarEvent;
use crate::hourangle::hour_angle;
use crate::julian::julian_to_unix;
use crate::longitude::ecliptic_longitude;
use crate::noon::mean_solar_noon;
use crate::transit::solar_transit;

/// Represent a full day at specific location, which allows to compute the exact date & time of any
/// solar event during this day.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SolarDay {
    lat: f64,
    solar_transit: f64,
    declination: f64,
}

impl SolarDay {
    /// Initialize given position (in degrees) and a date.
    ///
    /// This will precompute some values so you should re-use this struct if it is possible.
    pub fn new(lat: f64, lon: f64, year: i32, month: u32, day: u32) -> Self {
        let day = mean_solar_noon(lon, year, month, day);
        let solar_anomaly = solar_mean_anomaly(day);
        let equation_of_center = equation_of_center(solar_anomaly);
        let ecliptic_longitude = ecliptic_longitude(solar_anomaly, equation_of_center, day);
        let solar_transit = solar_transit(day, solar_anomaly, ecliptic_longitude);
        let declination = declination(ecliptic_longitude);

        Self {
            lat,
            solar_transit,
            declination,
        }
    }

    /// Get the UNIX timestamp for when the input event will happen.
    pub fn event_time(&self, event: SolarEvent) -> i64 {
        let hour_angle = hour_angle(self.lat, self.declination, event);
        let frac = hour_angle / (2. * PI);
        julian_to_unix(self.solar_transit + frac)
    }
}

/// Calculates the sunrise and sunset times for the given location and date.
pub fn sunrise_sunset(
    latitude: f64,
    longitude: f64,
    year: i32,
    month: u32,
    day: u32,
) -> (i64, i64) {
    let solar_day = SolarDay::new(latitude, longitude, year, month, day);

    (
        solar_day.event_time(SolarEvent::Sunrise),
        solar_day.event_time(SolarEvent::Sunset),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prime_meridian() {
        assert_eq!(super::sunrise_sunset(0., 0., 1970, 1, 1), (21594, 65228))
    }
}
