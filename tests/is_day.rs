use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use sunrise::{Coordinates, DawnType, SolarDay, SolarEvent};

fn dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().to_utc()
}

fn equator_1970() -> SolarDay {
    SolarDay::new(
        Coordinates::new(0., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    )
}

#[test]
fn typical_day_at_equator() {
    // Sunrise=05:59:54Z, sunset=18:07:08Z (see integration_test::test_sunrise).
    let sd = equator_1970();

    // Pre-dawn night.
    assert!(!sd.is_day(SolarEvent::Sunrise, dt("1970-01-01T03:00:00Z")));
    assert!(sd.is_night(SolarEvent::Sunrise, dt("1970-01-01T03:00:00Z")));

    // Mid-day.
    assert!(sd.is_day(SolarEvent::Sunrise, dt("1970-01-01T12:00:00Z")));
    assert!(!sd.is_night(SolarEvent::Sunrise, dt("1970-01-01T12:00:00Z")));

    // Post-sunset night.
    assert!(!sd.is_day(SolarEvent::Sunrise, dt("1970-01-01T21:00:00Z")));
    assert!(sd.is_night(SolarEvent::Sunrise, dt("1970-01-01T21:00:00Z")));
}

#[test]
fn boundary_around_sunrise() {
    let sd = equator_1970();
    let sunrise = sd.event_time(SolarEvent::Sunrise).unwrap();

    // The interval [sunrise, sunset) is treated as day: sunrise is day, the
    // instant before is still night.
    assert!(!sd.is_day(SolarEvent::Sunrise, sunrise - TimeDelta::seconds(1)));
    assert!(sd.is_day(SolarEvent::Sunrise, sunrise));
    assert!(sd.is_day(SolarEvent::Sunrise, sunrise + TimeDelta::seconds(1)));
}

#[test]
fn boundary_around_sunset() {
    let sd = equator_1970();
    let sunset = sd.event_time(SolarEvent::Sunset).unwrap();

    // Sunset itself is night (half-open interval), the second before is day.
    assert!(sd.is_day(SolarEvent::Sunrise, sunset - TimeDelta::seconds(1)));
    assert!(!sd.is_day(SolarEvent::Sunrise, sunset));
    assert!(!sd.is_day(SolarEvent::Sunrise, sunset + TimeDelta::seconds(1)));
}

#[test]
fn either_pair_member_works() {
    // Passing Sunset should produce the same result as Sunrise: dawn_dusk()
    // normalizes to the same (Sunrise, Sunset) pair.
    let sd = equator_1970();
    for hour in 0..24 {
        let t = dt(&format!("1970-01-01T{hour:02}:00:00Z"));
        assert_eq!(
            sd.is_day(SolarEvent::Sunrise, t),
            sd.is_day(SolarEvent::Sunset, t),
            "Sunrise and Sunset variants must agree at {t}",
        );
    }
}

#[test]
fn civil_dawn_window_is_wider_than_sunrise_window() {
    // At (0,0) on 2023-01-01: civil dawn=05:37:08, sunrise=~05:59 (well after),
    // so 05:45 is night by the sunrise definition but day by the civil one.
    let sd = SolarDay::new(
        Coordinates::new(0., 0.).unwrap(),
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
    );
    let t = dt("2023-01-01T05:45:00Z");
    assert!(sd.is_day(SolarEvent::Dawn(DawnType::Civil), t));
    assert!(!sd.is_day(SolarEvent::Sunrise, t));

    // Before civil dawn it's still night under either definition.
    let pre = dt("2023-01-01T04:00:00Z");
    assert!(!sd.is_day(SolarEvent::Dawn(DawnType::Civil), pre));
    assert!(!sd.is_day(SolarEvent::Sunrise, pre));
}

#[test]
fn is_night_negates_is_day() {
    let sd = equator_1970();
    for hour in 0..24 {
        let t = dt(&format!("1970-01-01T{hour:02}:00:00Z"));
        assert_ne!(
            sd.is_day(SolarEvent::Sunrise, t),
            sd.is_night(SolarEvent::Sunrise, t),
            "is_day and is_night must disagree at {t}",
        );
    }
}

#[test]
fn polar_day_north_is_always_day() {
    // 85N on 1970-07-01: no sunrise/sunset, summer heuristic => day.
    let sd = SolarDay::new(
        Coordinates::new(85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 7, 1).unwrap(),
    );
    assert_eq!(sd.event_time(SolarEvent::Sunrise), None);
    for hour in [0, 6, 12, 18, 23] {
        let t = dt(&format!("1970-07-01T{hour:02}:00:00Z"));
        assert!(sd.is_day(SolarEvent::Sunrise, t));
        assert!(!sd.is_night(SolarEvent::Sunrise, t));
    }
}

#[test]
fn polar_night_north_is_always_night() {
    // 85N on 1970-01-01: no events, winter heuristic => night.
    let sd = SolarDay::new(
        Coordinates::new(85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    );
    assert_eq!(sd.event_time(SolarEvent::Sunrise), None);
    for hour in [0, 6, 12, 18, 23] {
        let t = dt(&format!("1970-01-01T{hour:02}:00:00Z"));
        assert!(!sd.is_day(SolarEvent::Sunrise, t));
        assert!(sd.is_night(SolarEvent::Sunrise, t));
    }
}

#[test]
fn polar_day_south_is_always_day() {
    // 85S on 1970-01-01: southern summer => day.
    let sd = SolarDay::new(
        Coordinates::new(-85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    );
    assert_eq!(sd.event_time(SolarEvent::Sunrise), None);
    for hour in [0, 6, 12, 18, 23] {
        let t = dt(&format!("1970-01-01T{hour:02}:00:00Z"));
        assert!(sd.is_day(SolarEvent::Sunrise, t));
        assert!(!sd.is_night(SolarEvent::Sunrise, t));
    }
}

#[test]
fn polar_night_south_is_always_night() {
    // 85S on 1970-07-01: southern winter => night.
    let sd = SolarDay::new(
        Coordinates::new(-85., 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 7, 1).unwrap(),
    );
    assert_eq!(sd.event_time(SolarEvent::Sunrise), None);
    for hour in [0, 6, 12, 18, 23] {
        let t = dt(&format!("1970-07-01T{hour:02}:00:00Z"));
        assert!(!sd.is_day(SolarEvent::Sunrise, t));
        assert!(sd.is_night(SolarEvent::Sunrise, t));
    }
}

#[test]
fn near_24h_day_just_below_polar_cliff() {
    // 65.726N on the 1970 summer solstice produces the longest non-polar day:
    // sunrise just after midnight, sunset just after the following midnight,
    // total day length ~23h56m.
    let sd = SolarDay::new(
        Coordinates::new(65.726, 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 6, 21).unwrap(),
    );
    let sunrise = sd.event_time(SolarEvent::Sunrise).unwrap();
    let sunset = sd.event_time(SolarEvent::Sunset).unwrap();
    assert!(
        (sunset - sunrise).num_seconds() > 23 * 3600 + 50 * 60,
        "expected a day longer than 23h50m, got {}s",
        (sunset - sunrise).num_seconds(),
    );

    // The narrow night window sits just after midnight, before sunrise.
    assert!(sd.is_night(SolarEvent::Sunrise, dt("1970-06-21T00:00:00Z")));
    assert!(sd.is_night(SolarEvent::Sunrise, sunrise - TimeDelta::seconds(1)));

    // Mid-day and late evening on the computation date are both inside the day
    // window because sunset spills onto the following calendar day.
    assert!(sd.is_day(SolarEvent::Sunrise, sunrise));
    assert!(sd.is_day(SolarEvent::Sunrise, dt("1970-06-21T12:00:00Z")));
    assert!(sd.is_day(SolarEvent::Sunrise, dt("1970-06-21T23:59:00Z")));
}

#[test]
fn just_inside_polar_day_cliff_uses_heuristic() {
    // One step further north (65.73N) on the same solstice: the symmetric
    // hour-angle model returns no events, so is_day falls back to the
    // hemisphere/month heuristic -- June in the north is summer.
    let sd = SolarDay::new(
        Coordinates::new(65.73, 0.).unwrap(),
        NaiveDate::from_ymd_opt(1970, 6, 21).unwrap(),
    );
    assert_eq!(sd.event_time(SolarEvent::Sunrise), None);
    assert_eq!(sd.event_time(SolarEvent::Sunset), None);

    for hour in [0, 12, 23] {
        let t = dt(&format!("1970-06-21T{hour:02}:00:00Z"));
        assert!(sd.is_day(SolarEvent::Sunrise, t));
        assert!(!sd.is_night(SolarEvent::Sunrise, t));
    }
}
