pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut temp_hour: i32 = hours;
        let mut temp_minute: i32 = minutes;

        while temp_hour >= 24 {
            temp_hour -= 24;
        }

        while temp_minute >= 60 {
            temp_minute -= 60;
            temp_hour += 1;
            
            if temp_hour >= 24 {
                temp_hour -= 24;
            }
        }

        Clock {hour: temp_hour, minute: temp_minute}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }

    pub fn to_string(&self) -> () {
        println!("{:02}:{:02}",self.hour, self.minute);
    }
}
