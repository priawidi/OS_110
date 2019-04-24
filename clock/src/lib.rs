//Mentor :: BAGUS NUGRAHA
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    
    hours: i32,
    minutes : i32,
}

impl Clock {

    pub fn convert(hours: i32, minutes: i32) -> i32{
        let max_minute = 60*24;
        let  mut temp = hours*60 + minutes;
        temp = (temp % max_minute + max_minute) % max_minute;
        temp
        
    }
    pub fn new(hours: i32, minutes: i32) -> Self {
        let conv_minute = Clock::convert(hours, minutes);
        let  jam = conv_minute/60;
        let  menit = conv_minute % 60;      
        Clock{hours: jam, minutes: menit}

    }
   

    pub fn add_minutes(& self, minutes: i32) -> Self {
        //unimplemented!("Add {} minutes to existing Clock time", minutes);
        //let min = self.minutes + minutes;
        //let  mut temp = (self.hours*60 + minutes) + minutes;
        //let  jam = (temp/60)%24;
        //let  menit = temp % 60;
        Clock::new(self.hours, self.minutes+minutes)
    }

  
    //println!("{}", hours);

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}    

/*impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.hours, self.minutes)
    }
}    

impl PartialEq for Clock{
    fn eq(&self, other: &Clock) -> bool{
        true
    }    
}*/

//trash
/*let mut temp_menit = (60 + minutes % 60) %60;
        let mut temp_jam = (24 + hours % 24)%24;
        let mut temp = 0;
        
        if temp_menit == 60{
                temp += 1
            }
        if temp_menit > 60{
            
            temp += temp_menit % 60;

        }
        

        let menit :i32 = temp_menit;
        let jam :i32 = temp_jam + temp;
       */