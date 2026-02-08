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

#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("either the `std` or `libm` feature is required");

#[cfg(not(any(feature = "chrono", feature = "jiff")))]
compile_error!("either the `chrono` or `jiff` feature is required");

// Besides for documentation, only one of the `chrono` or `jiff` features can be enabled
#[cfg(all(not(doc), feature = "chrono", feature = "jiff"))]
compile_error!("only one of the `chrono` or `jiff` features can be enabled");

mod coordinates;
mod event;
mod julian;
mod math;
mod solar_equation;

pub use crate::coordinates::Coordinates;
pub use crate::event::{DawnType, SolarEvent};
pub use crate::solar_equation::SolarDay;

/// Calculates the sunrise and sunset times for the given location and date.
///
/// # Example
///
/// ```
/// use sunrise::sunrise_sunset;
///
/// // Calculate times for January 1, 2016 in Toronto
/// let (sunrise, sunset) = sunrise_sunset(43.6532, -79.3832, 2016, 1, 1);
/// ```
#[deprecated(
    since = "1.1.0",
    note = "Use [`SolarDay`] which is panic-free, more flexible and explicit."
)]
pub fn sunrise_sunset(
    latitude: f64,
    longitude: f64,
    year: i32,
    month: u32,
    day: u32,
) -> (i64, i64) {
    #[cfg(feature = "chrono")]
    let date = chrono::NaiveDate::from_ymd_opt(year, month, day).expect("invalid date");
    #[cfg(feature = "jiff")]
    let date = jiff::civil::Date::new(
        year.try_into().expect("invalid year"),
        month.try_into().expect("invalid month"),
        day.try_into().expect("invalid day"),
    )
    .expect("invalid date");

    let solar_day = SolarDay::new(
        Coordinates::new(latitude, longitude).expect("invalid coordinates"),
        date,
    );

    let sunrise = solar_day
        .event_time(SolarEvent::Sunrise)
        .expect("no sunrise");
    let sunset = solar_day.event_time(SolarEvent::Sunset).expect("no sunset");

    #[cfg(feature = "chrono")]
    {
        (sunrise.timestamp(), sunset.timestamp())
    }
    #[cfg(feature = "jiff")]
    {
        (sunrise.as_second(), sunset.as_second())
    }
}
