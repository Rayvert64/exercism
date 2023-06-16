use std::default::Default;
use std::fmt::Display;
use time::Instant;

pub struct Clock<T> {
    hour: T,
    minute: T,
}

impl<T: Default + Display> Default for Clock<T> {
    fn default() -> Self {
        let now = std::time::SystemTime().now();
        let converted_time: time::PrimitiveDateTime = time::PrimitiveDateTime::new(date, time);
        Clock::new(T::default(), T::default())
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
