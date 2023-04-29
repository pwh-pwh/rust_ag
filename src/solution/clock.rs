use std::fmt::{Display, Formatter};

#[derive(Debug,Eq,PartialEq)]
pub struct Clock(i32);

const DAY:i32 = 24*60;
const HOUR:i32 = 60;


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(
            ((hours*HOUR + minutes)%DAY +DAY) %DAY
        )
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0,self.0+minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:02}:{:02}",self.0/HOUR,self.0%HOUR)
    }
}




#[cfg(test)]
mod tests {
    use crate::solution::clock::Clock;

    #[test]
    fn cl() {
        assert_eq!(Clock::new(-25, -160).to_string(),"20:20");
    }
    #[test]
    fn test_eq() {
        assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
    }
}




