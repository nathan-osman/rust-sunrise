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

use core::f64::consts::PI;

use chrono::{DateTime, NaiveDate};
use sunrise::{Coordinates, DawnType, SolarDay, SolarEvent};

#[allow(deprecated)]
use sunrise::sunrise_sunset;

fn solar_day(year: i32) -> SolarDay {
    SolarDay::new(
        Coordinates::new(0., 0.).unwrap(),
        NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
    )
}

#[test]
#[allow(deprecated)]
fn test_sunrise() {
    assert_eq!(sunrise_sunset(0., 0., 1970, 1, 1), (21594, 65228));

    assert_eq!(
        solar_day(1970).event_time(SolarEvent::Sunrise).unwrap(),
        DateTime::parse_from_rfc3339("1970-01-01T05:59:54Z").unwrap()
    );

    assert_eq!(
        solar_day(1970).event_time(SolarEvent::Sunset).unwrap(),
        DateTime::parse_from_rfc3339("1970-01-01T18:07:08Z").unwrap()
    );
}

#[test]
fn test_altitude() {
    assert_eq!(
        solar_day(1970)
            .with_altitude(123.)
            .event_time(SolarEvent::Sunrise)
            .unwrap(),
        DateTime::parse_from_rfc3339("1970-01-01T05:58:14Z").unwrap()
    );

    assert_eq!(
        solar_day(1970)
            .with_altitude(-10.)
            .event_time(SolarEvent::Sunrise)
            .unwrap(),
        DateTime::parse_from_rfc3339("1970-01-01T06:00:22Z").unwrap()
    );
}

#[test]
fn test_civil() {
    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dawn(DawnType::Civil))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T05:37:08Z").unwrap()
    );

    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dusk(DawnType::Civil))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T18:29:18Z").unwrap()
    );
}

#[test]
fn test_nautical() {
    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dawn(DawnType::Nautical))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T05:11:00Z").unwrap()
    );

    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dusk(DawnType::Nautical))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T18:55:27Z").unwrap()
    );
}

#[test]
fn test_astronomical() {
    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dawn(DawnType::Astronomical))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T04:44:45Z").unwrap()
    );

    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Dusk(DawnType::Astronomical))
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T19:21:42Z").unwrap()
    );
}

#[test]
fn test_elevation() {
    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Elevation {
                elevation: PI / 4.0,
                morning: true
            })
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T02:42:24Z").unwrap()
    );

    assert_eq!(
        solar_day(2023)
            .event_time(SolarEvent::Elevation {
                elevation: PI / 4.0,
                morning: false
            })
            .unwrap(),
        DateTime::parse_from_rfc3339("2023-01-01T21:24:02Z").unwrap()
    );
}

#[test]
fn test_order() {
    let sd = {
        SolarDay::new(
            Coordinates::new(2.0, 10.0).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 23).unwrap(),
        )
        .with_altitude(100.0)
    };

    let events_time = [
        sd.event_time(SolarEvent::Dawn(DawnType::Astronomical)),
        sd.event_time(SolarEvent::Dawn(DawnType::Nautical)),
        sd.event_time(SolarEvent::Dawn(DawnType::Civil)),
        sd.event_time(SolarEvent::Sunrise),
        sd.event_time(SolarEvent::Elevation {
            elevation: -0.1,
            morning: true,
        }),
        sd.event_time(SolarEvent::Elevation {
            elevation: -0.1,
            morning: false,
        }),
        sd.event_time(SolarEvent::Sunset),
        sd.event_time(SolarEvent::Dusk(DawnType::Civil)),
        sd.event_time(SolarEvent::Dusk(DawnType::Nautical)),
        sd.event_time(SolarEvent::Dusk(DawnType::Astronomical)),
    ];

    assert!(events_time.is_sorted());
}

#[test]
fn test_polar_day() {
    let arctic_polar_day = SolarDay::new(
        Coordinates::new(85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 8, 1).unwrap(),
    );
    assert_eq!(arctic_polar_day.event_time(SolarEvent::Sunrise), None);
    assert_eq!(arctic_polar_day.event_time(SolarEvent::Sunset), None);

    let antarctic_polar_night = SolarDay::new(
        Coordinates::new(-85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 8, 1).unwrap(),
    );
    assert_eq!(antarctic_polar_night.event_time(SolarEvent::Sunrise), None);
    assert_eq!(antarctic_polar_night.event_time(SolarEvent::Sunset), None);
}
