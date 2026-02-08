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

const SECONDS_IN_A_DAY: f64 = 86400.;
const UNIX_EPOCH_JULIAN_DAY: f64 = 2440587.5;

#[cfg(feature = "chrono")]
pub(crate) mod chrono;
#[cfg(feature = "jiff")]
pub(crate) mod jiff;

/// Converts a unix timestamp to a Julian day.
pub(crate) fn unix_to_julian(timestamp: i64) -> f64 {
    timestamp as f64 / SECONDS_IN_A_DAY + UNIX_EPOCH_JULIAN_DAY
}

/// Converts a Julian day to a unix timestamp.
pub(crate) fn julian_to_unix(day: f64) -> i64 {
    ((day - UNIX_EPOCH_JULIAN_DAY) * SECONDS_IN_A_DAY) as i64
}

#[cfg(test)]
mod tests {
    use crate::julian::UNIX_EPOCH_JULIAN_DAY;

    #[test]
    fn test_unix_to_julian() {
        assert_eq!(super::unix_to_julian(0), UNIX_EPOCH_JULIAN_DAY)
    }

    #[test]
    fn test_julian_to_unix() {
        assert_eq!(super::julian_to_unix(UNIX_EPOCH_JULIAN_DAY), 0)
    }
}
