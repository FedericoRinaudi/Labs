

use std::fmt;
use std::fmt::Formatter;


#[derive(Debug, Eq, PartialEq)]
pub struct Clock{
    minutes: i32,
}

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 60*24;



impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self{
            minutes: (((minutes + hours * MINUTES_PER_HOUR) % MINUTES_PER_DAY) + MINUTES_PER_DAY ) % MINUTES_PER_DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, minutes+self.minutes)
    }


}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes/60, self.minutes%60)
    }
}


