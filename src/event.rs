// The MIT License (MIT)
//
// Copyright (c) 2023 Nathan Osman
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

use crate::DEGREE;

/// Type of dawn or dusk computation.
///
/// If you are not sure which one to pick you probably want to use `Civil`. See
/// <https://en.wikipedia.org/wiki/Dawn#Types_of_dawn> for definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DawnType {
    /// Civil dawn begins when there is enough light for most objects to be distinguishable, so
    /// that some outdoor activities can commence. Formally, it occurs when the Sun is 6 degrees
    /// below the horizon in the morning.
    Civil,
    /// Nautical twilight begins when there is enough illumination for sailors to distinguish the
    /// horizon at sea but the sky being too dark to perform outdoor activities. Formally, it
    /// begins when the Sun is 12 degrees below the horizon in the morning. The sky becomes light
    /// enough to clearly distinguish it from land and water. Nautical dawn marks the start of
    /// nautical twilight, which lasts until civil dawn.
    Nautical,
    /// Astronomical dawn begins when the Sun is 18 degrees below the horizon in the morning.
    /// Astronomical twilight follows instantly until the Sun is 12 degrees below the horizon. At
    /// this point a very small portion of the Sun's rays illuminate the sky and the fainter stars
    /// begin to disappear. Astronomical dawn is often indistinguishable from night, especially in
    /// areas with light pollution. Astronomical dawn marks the beginning of astronomical twilight,
    /// which lasts until nautical dawn.
    Astronomical,
}

impl DawnType {
    pub(crate) fn positive_angle(&self) -> f64 {
        match self {
            DawnType::Civil => 6. * DEGREE,
            DawnType::Nautical => 12. * DEGREE,
            DawnType::Astronomical => 18. * DEGREE,
        }
    }
}

/// Common solar events.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SolarEvent {
    /// Sunrise is the moment when the upper rim of the Sun appears on the horizon in the morning.
    Sunrise,
    /// Sunset, is the daily disappearance of the Sun below the horizon in the evening.
    Sunset,
    /// Dusk is the time that marks the end of twilight after sunset.
    Dawn(DawnType),
    /// Dawn is the time that marks the beginning of twilight before sunrise.
    Dusk(DawnType),
    /// The point in time where the sun reaches a given elevation.
    Elevation {
        /// Sun's elevation, in radians.
        elevation: f64,
        /// `true` if this is the morning, `false` otherwise.
        morning: bool,
    },
}

impl SolarEvent {
    pub(crate) fn angle(&self) -> f64 {
        match self {
            SolarEvent::Sunrise | SolarEvent::Sunset => 5. * DEGREE / 6.,
            SolarEvent::Dusk(t) | SolarEvent::Dawn(t) => t.positive_angle(),
            SolarEvent::Elevation { elevation, .. } => *elevation,
        }
    }

    pub(crate) fn is_morning(&self) -> bool {
        matches!(
            self,
            SolarEvent::Sunrise | SolarEvent::Dusk(_) | SolarEvent::Elevation { morning: true, .. }
        )
    }
}
