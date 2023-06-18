use chrono::*;
use std::default::Default;
use std::fmt::Display;

/// The reason for using u32s is to not restrict the amount of traits that can be used
const MIN_IN_HOUR: u32 = 60;
const HOURS_IN_DAY: u32 = 24;

pub trait ClockTraits:
    Default + Display + num_traits::int::PrimInt + std::convert::From<u32>
{
}

pub struct Clock<T: ClockTraits> {
    hour: T,
    minute: T,
}

impl<T: ClockTraits> Default for Clock<T> {
    fn default() -> Self {
        let now: chrono::DateTime<chrono::Utc> = chrono::Local::now().into();
        Clock::new(now.hour().into(), now.minute().into())
    }
}

impl<T: ClockTraits> Clock<T> {
    pub fn new(hours: T, minutes: T) -> Self {
        Clock {
            hour: hours,
            minute: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: T) -> Self {
        let hours = minutes / MIN_IN_HOUR.into();
        let days = carry_hours / HOURS_IN_DAY.into();
        if carry_days > 0 {}
    }
}
