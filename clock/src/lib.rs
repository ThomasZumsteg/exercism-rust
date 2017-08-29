#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn new(hour: i32 , minute: i32) -> Clock {
        let (mut h, mut m) = (hour, minute);

        while m < 0 { h -= 1; m += 60 }

        h += m / 60;
        while h < 0 { h += 24 }

        Clock { hour: h % 24, minute: m % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hour, self.minute + minutes)
    }
}
