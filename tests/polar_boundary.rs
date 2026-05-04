use chrono::NaiveDate;
use sunrise::{Coordinates, SolarDay, SolarEvent};

// The hour-angle model is symmetric around solar noon, so the polar-day and
// polar-night boundaries jump directly from a day with both sunrise and sunset
// to a day with neither. The following tests pin that behaviour at 1970
// boundaries on both hemispheres; if the algorithm is ever made asymmetric to
// surface "rises but doesn't set" days, these will need to be updated.
fn assert_boundary(lat: f64, before: NaiveDate, after: NaiveDate, before_has_events: bool) {
    let coord = Coordinates::new(lat, 0.).unwrap();
    let before_sd = SolarDay::new(coord, before);
    let after_sd = SolarDay::new(coord, after);

    assert_eq!(
        before_sd.event_time(SolarEvent::Sunrise).is_some(),
        before_has_events,
        "{lat}° {before}: unexpected sunrise presence",
    );
    assert_eq!(
        before_sd.event_time(SolarEvent::Sunset).is_some(),
        before_has_events,
        "{lat}° {before}: unexpected sunset presence",
    );
    assert_eq!(
        after_sd.event_time(SolarEvent::Sunrise).is_some(),
        !before_has_events,
        "{lat}° {after}: unexpected sunrise presence",
    );
    assert_eq!(
        after_sd.event_time(SolarEvent::Sunset).is_some(),
        !before_has_events,
        "{lat}° {after}: unexpected sunset presence",
    );
}

#[test]
fn test_polar_day_onset_north() {
    assert_boundary(
        78.,
        NaiveDate::from_ymd_opt(1970, 4, 18).unwrap(),
        NaiveDate::from_ymd_opt(1970, 4, 19).unwrap(),
        true,
    );
}

#[test]
fn test_polar_day_end_north() {
    assert_boundary(
        78.,
        NaiveDate::from_ymd_opt(1970, 8, 23).unwrap(),
        NaiveDate::from_ymd_opt(1970, 8, 24).unwrap(),
        false,
    );
}

#[test]
fn test_polar_night_onset_north() {
    assert_boundary(
        78.,
        NaiveDate::from_ymd_opt(1970, 10, 26).unwrap(),
        NaiveDate::from_ymd_opt(1970, 10, 27).unwrap(),
        true,
    );
}

#[test]
fn test_polar_night_end_north() {
    assert_boundary(
        78.,
        NaiveDate::from_ymd_opt(1970, 2, 14).unwrap(),
        NaiveDate::from_ymd_opt(1970, 2, 15).unwrap(),
        false,
    );
}

#[test]
fn test_polar_day_onset_south() {
    assert_boundary(
        -78.,
        NaiveDate::from_ymd_opt(1970, 10, 22).unwrap(),
        NaiveDate::from_ymd_opt(1970, 10, 23).unwrap(),
        true,
    );
}

#[test]
fn test_polar_day_end_south() {
    assert_boundary(
        -78.,
        NaiveDate::from_ymd_opt(1970, 2, 19).unwrap(),
        NaiveDate::from_ymd_opt(1970, 2, 20).unwrap(),
        false,
    );
}

#[test]
fn test_polar_night_onset_south() {
    assert_boundary(
        -78.,
        NaiveDate::from_ymd_opt(1970, 4, 23).unwrap(),
        NaiveDate::from_ymd_opt(1970, 4, 24).unwrap(),
        true,
    );
}

#[test]
fn test_polar_night_end_south() {
    assert_boundary(
        -78.,
        NaiveDate::from_ymd_opt(1970, 8, 18).unwrap(),
        NaiveDate::from_ymd_opt(1970, 8, 19).unwrap(),
        false,
    );
}
