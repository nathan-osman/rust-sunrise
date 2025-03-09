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

use chrono::NaiveDate;
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
        solar_day(1970).event_time(SolarEvent::Sunrise),
        21594 // 01/01/1970 06:59:54
    );

    assert_eq!(
        solar_day(1970).event_time(SolarEvent::Sunset),
        65228 // 01/01/1970 19:07:08
    );
}

#[test]
fn test_altitude() {
    assert_eq!(
        solar_day(1970)
            .with_altitude(123.)
            .event_time(SolarEvent::Sunrise),
        21494 // 01/01/1970 06:58:14
    );

    assert_eq!(
        solar_day(1970)
            .with_altitude(-10.)
            .event_time(SolarEvent::Sunrise),
        21622 // 01/01/1970 07:00:22
    );
}

#[test]
fn test_civil() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Civil)),
        1672551428 // 01/01/2023 06:37:08
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Civil)),
        1672597758 // 01/01/2023 19:29:18
    );
}

#[test]
fn test_nautical() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Nautical)),
        1672549860 // 01/01/2023 06:11:00
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Nautical)),
        1672599327 // 01/01/2023 19:55:27
    );
}

#[test]
fn test_astronomical() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dusk(DawnType::Astronomical)),
        1672548285 // 01/01/2023 05:44:45
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Dawn(DawnType::Astronomical)),
        1672600902 // 01/01/2023 20:21:42
    );
}

#[test]
fn test_elevation() {
    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Elevation {
            elevation: PI / 4.0,
            morning: true
        }),
        1672540944 // 01/01/2023 02:42:24
    );

    assert_eq!(
        solar_day(2023).event_time(SolarEvent::Elevation {
            elevation: PI / 4.0,
            morning: false
        }),
        1672608242 // 01/01/2023 21:24:02
    );
}
