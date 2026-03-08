use std::fmt;

#[derive(PartialEq, Debug, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minute_hours = minutes.div_euclid(60);
        let minutes = minutes.rem_euclid(60);
        let hours = hours + minute_hours;
        let hours = hours.rem_euclid(24);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes;
        Self::new(0, total_minutes + minutes)
    }
}
