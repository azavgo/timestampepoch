#[cfg(test)]
mod tests;

// Provides date data type, and constructors for the date data type
#[derive(Debug, PartialEq)]
pub struct Date {
    day: usize, 
    month: usize, 
    year: usize,
}

impl Date {
    pub fn new(day: usize, month: usize, year: usize) -> Self {
        Self {
            day: day, 
            month: month, 
            year: year,
        }
    }
    
    pub fn day(self: &Self) -> usize {
        self.day
    }

    pub fn month(self: &Self) -> usize {
        self.month
    }

    pub fn year(self: &Self) -> usize {
        self.year
    }

    pub fn to_string(self: &Self) -> String {
        let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        self.day().to_string() + " " + month[self.month() - 1] + " " + &self.year().to_string()
    }

    pub fn is_leap_year(self: &Self) -> bool {
        if self.year % 100 != 0 && self.year % 4 == 0 {
            true
        } else if self.year % 400 == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_leap(year: usize) -> bool {
        if year % 100 != 0 && year % 4 == 0 {
            true
        } else if year % 400 == 0 {
            true
        } else {
            false
        }
    }

    pub fn is_epoch_year(self: &Self) -> bool {
        if self.year < 1970 {
            false
        } else {
            true
        }
    }

    pub fn leap_years(self: &Self) -> Option<usize> { //Number of leap years between 1970 and the date
        if self.is_epoch_year() == true {
            let mut i: usize = 0;
            if self.year() == 1970 {
                Some(i)
            } else {
                for _year in 1971..= self.year() {
                    match Self::is_leap(_year - 1) {
                        true  => i += 1, 
                        false => continue,
                    }
                }
                Some(i)
            }
        } else {
            None
        }   
    }

    pub fn date_timestamp(timestamp: usize) -> Self {

        let scy: usize = 31536000; 
        let sly: usize = 31622400; 
    
        let mut seconds: usize = 0; 
        let mut year: usize = 1970;
    
        if timestamp >= scy {
            while seconds <= timestamp { 
                match Self::is_leap(year) {
                    true  => seconds += sly, 
                    false => seconds += scy,
                }
                year += 1;
            }   
            year -= 1;    
        }
    
        match Self::is_leap(year) {
            true  => if seconds >= sly {seconds -= sly}, 
            false => if seconds >= scy {seconds -= scy},
        }
    
        let m28: usize = 86400 * 28; 
        let m29: usize = 86400 * 29;
        let m30: usize = 86400 * 30;
        let m31: usize = 86400 * 31;
        
        // jan = 31, feb = 28 or 29, mar = 31, apr = 30, may = 31, jun = 30, 
        // jul = 31, aug = 31, sep = 30, oct = 31, nov = 30, dec = 31 
    
        let mut month = 1;  
        
        match Self::is_leap(year) {
            true => {
                if seconds + m31 > timestamp { 
                    month = 1;
                } 
                else if seconds + m31 + m29 > timestamp {
                    seconds = seconds + m31; 
                    month += 1;
                } else if seconds + m31 + m29 + m31 > timestamp {
                    seconds = seconds + m31 + m29;
                    month += 2;
                } else if seconds + m31 + m29 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m29 + m31;
                    month += 3;
                } else if seconds + m31 + m29 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30;
                    month += 4; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31;
                    month += 5; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30;
                    month += 6; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31;
                    month += 7; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31;
                    month += 8; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30;
                    month += 9; 
                } else if seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31;
                    month += 10; 
                } else {
                    seconds = seconds + m31 + m29 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 + m30;
                    month += 11; 
                }
            }, 
            false => {
                if seconds + m31 > timestamp { 
                    month = 1; 
                } else if seconds + m31 + m28 > timestamp {
                    seconds = seconds + m31; 
                    month += 1; 
                } else if seconds + m31 + m28 + m31 > timestamp {
                    seconds = seconds + m31 + m28;
                    month += 2; 
                } else if seconds + m31 + m28 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m28 + m31;
                    month += 3; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30;
                    month += 4; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31;
                    month += 5; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30;
                    month += 6; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31;
                    month += 7; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31;
                    month += 8; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30;
                    month += 9; 
                } else if seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 + m30 > timestamp {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31;
                    month += 10; 
                } else {
                    seconds = seconds + m31 + m28 + m31 + m30 + m31 + m30 + m31 + m31 + m30 + m31 + m30;
                    month += 11; 
                }
            }
        }
    
        let day: usize = 1 + (((timestamp - seconds) as f32) / 86400.0) as usize;
        Self::new(day, month, year)
    }



    pub fn timestamp(self: &Self) -> Option<usize> {
        // Number of seconds in a common year = 31536000 
        // Number of seconds in a leap year  = 31622400 
        // Number of seconds in a day = 86400
        match self.is_epoch_year() {
            false => None, 
            true  => {
                        // number of leap years between date and 1970
                        let ly: usize = match self.leap_years() {
                            None => 0,
                            Some(value) => value,
                        }; 
    
                        let cy: usize = self.year() - 1970 - ly; // number of common years between date and 1970
                        let yts: usize = ly * 31622400 + cy * 31536000; // number of seconds in years between the date and 1970
                        
                        // number of seconds in months in the date year
                        let mts: usize = match self.is_leap_year() {
                            true  => { 
                                        match self.month() {
                                            1  => 0, 
                                            2  => 2678400,
                                            3  => 5184000, 
                                            4  => 7862400, 
                                            5  => 10454400, 
                                            6  => 13132800, 
                                            7  => 15724800, 
                                            8  => 18403200, 
                                            9  => 21081600, 
                                            10 => 23673600, 
                                            11 => 26352000, 
                                            12 => 28944000, 
                                            _  => 0,
                                        }                                                      
                            },
                            false => {     
                                        match self.month() {
                                            1  => 0, 
                                            2  => 2678400,
                                            3  => 5097600, 
                                            4  => 7776000, 
                                            5  => 10368000, 
                                            6  => 13046400, 
                                            7  => 15638400, 
                                            8  => 18316800, 
                                            9  => 20995200, 
                                            10 => 23587200, 
                                            11 => 26265600, 
                                            12 => 28857600, 
                                            _  => 0,
                                        }
                                     
                            },
                        };
                        Some(&yts + &mts + (self.day() - 1) * 86400)                
            },
        }
    }
}







