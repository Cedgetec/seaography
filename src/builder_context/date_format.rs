use std::fmt::Display;

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub enum DateValue {
    #[cfg(feature = "with-chrono")]
    ChronoDate(NaiveDate),

    #[cfg(feature = "with-chrono")]
    ChronoTime(NaiveTime),

    #[cfg(feature = "with-chrono")]
    ChronoDateTime(NaiveDateTime),

    #[cfg(feature = "with-chrono")]
    ChronoDateTimeUtc(DateTime<Utc>),

    #[cfg(feature = "with-chrono")]
    ChronoDateTimeLocal(DateTime<Local>),

    #[cfg(feature = "with-chrono")]
    ChronoDateTimeWithTimeZone(DateTime<FixedOffset>),
}
#[cfg(any(feature = "with-chrono"))]
mod chrono_impl {
    use super::*;

    impl From<NaiveDate> for DateValue {
        fn from(value: NaiveDate) -> Self {
            Self::ChronoDate(value)
        }
    }
    
    impl From<NaiveDateTime> for DateValue {
        fn from(value: NaiveDateTime) -> Self {
            Self::ChronoDateTime(value)
        }
    }
    
    impl From<NaiveTime> for DateValue {
        fn from(value: NaiveTime) -> Self {
            Self::ChronoTime(value)
        }
    }
    
    impl From<DateTime<Utc>> for DateValue {
        fn from(value: DateTime<Utc>) -> Self {
            Self::ChronoDateTimeUtc(value)
        }
    }
    
    impl From<DateTime<Local>> for DateValue {
        fn from(value: DateTime<Local>) -> Self {
            Self::ChronoDateTimeLocal(value)
        }
    }
    
    impl From<DateTime<FixedOffset>> for DateValue {
        fn from(value: DateTime<FixedOffset>) -> Self {
            Self::ChronoDateTimeWithTimeZone(value)
        }
    }
    
    impl Display for DateValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DateValue::ChronoDate(date) => date.fmt(f),
                DateValue::ChronoDateTime(date) => date.fmt(f),
                DateValue::ChronoTime(date) => date.fmt(f),
                DateValue::ChronoDateTimeUtc(date) => date.fmt(f),
                DateValue::ChronoDateTimeLocal(date) => date.fmt(f),
                DateValue::ChronoDateTimeWithTimeZone(date) => date.fmt(f),
            }
        }
    }
}

pub type DateFormatFn = Box<dyn Fn(DateValue) -> String + Sync + Send>;