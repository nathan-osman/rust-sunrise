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

use core::f64::consts::PI;

use chrono::{DateTime, Datelike, NaiveDate, Utc};

use crate::Coordinates;
use crate::event::SolarEvent;
use crate::julian::{julian_to_unix, mean_solar_noon};

use self::anomaly::solar_mean_anomaly;
use self::center::equation_of_center;
use self::declination::declination;
use self::hourangle::hour_angle;
use self::longitude::ecliptic_longitude;
use self::transit::solar_transit;

/// Represent a full day at specific location, which allows to compute the exact date & time of any
/// solar event during this day.
///
/// # Example
///
/// ```
/// use chrono::NaiveDate;
/// use sunrise::{Coordinates, DawnType, SolarDay, SolarEvent};
///
/// // January 1, 2016 in Toronto
/// let date = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap();
/// let coord = Coordinates::new(43.6532, -79.3832).unwrap();
///
/// let dawn = SolarDay::new(coord, date)
///     .with_altitude(54.)
///     .event_time(SolarEvent::Dawn(DawnType::Civil));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SolarDay {
    lat: f64,
    altitude: f64,
    solar_transit: f64,
    declination: f64,
}

impl SolarDay {
    /// Initialize given position and a date.
    ///
    /// This will pre-compute some values so you should re-use this struct if it is possible.
    pub fn new(coord: Coordinates, date: NaiveDate) -> Self {
        let day = mean_solar_noon(coord.lon(), date);
        let solar_anomaly = solar_mean_anomaly(day);
        let equation_of_center = equation_of_center(solar_anomaly);
        let ecliptic_longitude = ecliptic_longitude(solar_anomaly, equation_of_center, day);
        let solar_transit = solar_transit(day, solar_anomaly, ecliptic_longitude);
        let declination = declination(ecliptic_longitude);

        Self {
            lat: coord.lat(),
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

    /// Get the time for when the input event will happen.
    ///
    /// Returns `None` if the event does not happen (e.g., sunset in a polar day).
    pub fn event_time(&self, event: SolarEvent) -> Option<DateTime<Utc>> {
        let hour_angle = hour_angle(self.lat, self.declination, self.altitude, event);
        if hour_angle.is_nan() {
            return None;
        }

        let frac = hour_angle / (2. * PI);
        let timestamp = julian_to_unix(self.solar_transit + frac);
        Some(DateTime::from_timestamp(timestamp, 0).expect("invalid result"))
    }

    /// Whether it's currently day, as defined by the [`SolarEvent`].
    ///
    /// Either the start or end variants of `SolarEvent` can be used as they will
    /// be built into the matching pair of the start and end of the day.
    ///
    /// For days during the polar day/night, uses a simple month and hemisphere
    /// based heuristic to determine whether it's currently day or night.
    ///
    /// Using a time that is not on the same date as the one used for building the
    /// struct will give non-sensical results.
    pub fn is_day(&self, event: SolarEvent, time: DateTime<Utc>) -> bool {
        let (dawn, dusk) = event.dawn_dusk();
        match (self.event_time(dawn), self.event_time(dusk)) {
            (Some(dawn_time), Some(dusk_time)) => time >= dawn_time && time < dusk_time,
            (None, None) => is_summer(self.lat, time.date_naive()),

            // This invariant is enforced by tests, therefore it's not documented
            // as a potential panic (as that panic would be a bug in the implementation).
            _ => unreachable!("a day must always have both a sunset and a sunrise, or neither"),
        }
    }

    /// Reverse of [`Self::is_day`], see it's documentation for details.
    pub fn is_night(&self, event: SolarEvent, time: DateTime<Utc>) -> bool {
        !self.is_day(event, time)
    }
}

fn is_summer(lat: f64, date: NaiveDate) -> bool {
    let is_northern_hemisphere = lat >= 0.0;
    let summer_in_northern_hemisphere = matches!(date.month(), 4..=9);

    if is_northern_hemisphere {
        summer_in_northern_hemisphere
    } else {
        !summer_in_northern_hemisphere
    }
}
