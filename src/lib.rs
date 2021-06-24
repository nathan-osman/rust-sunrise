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

use std::f64;

const DEGREE: f64 = f64::consts::PI / 180.;

pub enum Azimuth {
    Official,
    Civil,
    Nautical,
    Astronomical,
}

impl Azimuth {
    pub fn angle(&self) -> f64 {
        match self {
            Azimuth::Official => 5.0 / 6.0,
            Azimuth::Civil => 6.0,
            Azimuth::Nautical => 12.0,
            Azimuth::Astronomical => 18.0,
        }
    }
}

mod anomaly;
mod center;
mod declination;
mod hourangle;
mod julian;
mod longitude;
mod noon;
mod perihelion;
mod sunrise;
mod transit;

pub use crate::sunrise::astronomical_twilight;
pub use crate::sunrise::civil_twilight;
pub use crate::sunrise::nautical_twilight;
pub use crate::sunrise::sunrise_sunset;
pub use crate::sunrise::time_of_transit;
