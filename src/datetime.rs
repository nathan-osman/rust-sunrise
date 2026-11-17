#[derive(PartialEq, PartialOrd, Debug)]
pub struct DateTime(i64);

impl DateTime {
    pub fn new(timestamp: i64) -> Self {
        DateTime(timestamp)
    }

    pub fn timestamp(&self) -> i64 {
        self.0
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(dt: DateTime) -> Self {
        chrono::DateTime::from_timestamp(dt.0, 0).expect("invalid result")
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for jiff::Timestamp {
    fn from(dt: DateTime) -> Self {
        jiff::Timestamp::from_second(dt.0).expect("invalid result")
    }
}

pub trait IntoDateTime {
    fn at_solar_noon(self) -> DateTime;
}

#[cfg(feature = "chrono")]
impl IntoDateTime for chrono::NaiveDate {
    fn at_solar_noon(self) -> DateTime {
        static NOON_TIME: chrono::NaiveTime = chrono::NaiveTime::from_hms_opt(12, 0, 0).unwrap();

        let timestamp = self.and_time(NOON_TIME).and_utc().timestamp();
        DateTime(timestamp)
    }
}

#[cfg(feature = "jiff")]
impl IntoDateTime for jiff::civil::Date {
    fn at_solar_noon(self) -> DateTime {
        let timestamp = self
            .at(12, 0, 0, 0)
            .to_zoned_datetime(jiff::tz::TimeZone::UTC)
            .expect("Unable to convert to UTC timestamp")
            .timestamp();
        DateTime(timestamp.as_second())
    }
}
