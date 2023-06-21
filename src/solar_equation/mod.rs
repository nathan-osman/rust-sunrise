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

mod anomaly;
mod center;
mod declination;
mod hourangle;
mod longitude;
mod perihelion;
mod transit;

use std::f64::consts::PI;

use self::anomaly::solar_mean_anomaly;
use self::center::equation_of_center;
use self::declination::declination;
use self::hourangle::hour_angle;
use self::longitude::ecliptic_longitude;
use self::transit::solar_transit;
use crate::event::SolarEvent;
use crate::julian::{julian_to_unix, mean_solar_noon};

/// Represent a full day at specific location, which allows to compute the exact date & time of any
/// solar event during this day.
///
/// # Example
///
/// ```
/// use sunrise::{sunrise_sunset, SolarDay, SolarEvent, DawnType};
///
/// // January 1, 2016 in Toronto
/// let solar_day = SolarDay::new(43.6532, -79.3832, 2016, 1, 1);
/// let sunrise_time = solar_day.event_time(SolarEvent::Sunrise);
/// let dawn_time = solar_day.event_time(SolarEvent::Dawn(DawnType::Civil));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SolarDay {
    lat: f64,
    altitude: f64,
    solar_transit: f64,
    declination: f64,
}

impl SolarDay {
    /// Initialize given position (in degrees) and a date.
    ///
    /// This will pre-compute some values so you should re-use this struct if it is possible.
    pub fn new(lat: f64, lon: f64, year: i32, month: u32, day: u32) -> Self {
        let day = mean_solar_noon(lon, year, month, day);
        let solar_anomaly = solar_mean_anomaly(day);
        let equation_of_center = equation_of_center(solar_anomaly);
        let ecliptic_longitude = ecliptic_longitude(solar_anomaly, equation_of_center, day);
        let solar_transit = solar_transit(day, solar_anomaly, ecliptic_longitude);
        let declination = declination(ecliptic_longitude);

        Self {
            lat,
            altitude: 0.,
            solar_transit,
            declination,
        }
    }

    /// Specify the altitude (in meters) of the observer, in meters. This defaults to 0 if not
    /// specified.
    pub fn with_altitude(mut self, altitude: f64) -> Self {
        self.altitude = altitude;
        self
    }

    /// Get the UNIX timestamp for when the input event will happen.
    pub fn event_time(&self, event: SolarEvent) -> i64 {
        let hour_angle = hour_angle(self.lat, self.declination, self.altitude, event);
        let frac = hour_angle / (2. * PI);
        julian_to_unix(self.solar_transit + frac)
    }
}
