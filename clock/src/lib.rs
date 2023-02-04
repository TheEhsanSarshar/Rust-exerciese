use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let actual_hours = hours % 24;
        let total_minutes = (((24 + actual_hours).abs() * 60) + minutes) % (24 * 60);
        Clock {
            minutes: total_minutes.abs(),
        }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = (self.minutes + minutes).abs() % (24 * 60);
        Clock {
            minutes: total_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:0width$}:{:0width$}", hours, minutes, width = 2)
    }
}
