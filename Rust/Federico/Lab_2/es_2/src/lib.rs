

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

impl std::ops::Add<i32> for Clock{
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self.add_minutes(rhs)
    }

}

impl std::ops::Add<Self> for Clock{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_minutes(rhs.minutes)
    }

}

impl std::ops::Sub<i32> for Clock{
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self.add_minutes(-rhs)
    }

}

impl std::ops::Sub<Self> for Clock{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add_minutes(-rhs.minutes)
    }

}

