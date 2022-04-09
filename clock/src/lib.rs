use std::{cmp, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Clock{
    hour: i32,
    min: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut clock = Clock { hour: hours, min: minutes};
        clock.normalization();
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hour, self.min+minutes)
    }

    fn normalization(&mut self) {
        let mut over_hours = self.min/60;     //se ho 72 minuti aggiungo un ora
        self.min %= 60;     //se ho 72 minuti ne tengo solo 12
        if self.min<0{
            self.min+=60;
            over_hours -=1;
        }
        self.hour += over_hours;
        self.hour %= 24;
        if self.hour<0 {
            self.hour+=24;
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.min)
    }
}

impl cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hour==other.hour && self.min==other.min
    }
}
