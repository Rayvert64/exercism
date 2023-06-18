use chrono::*;
use std::default::Default;
use std::fmt::Display;

pub struct Clock<T> {
    hour: T,
    minute: T,
}

impl<T: Default + Display> Default for Clock<T> {
    fn default() -> Self {
        let now: chrono::DateTime<chrono::Utc> = chrono::Local::now().into();
        Clock::new(now.hour().into(), now.minute().into())
    }
}

impl<T: Display + Display> Clock<T> {
    pub fn new(hours: T, minutes: T) -> Self {
        Clock {
            hour: hours,
            minute: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: T) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}
