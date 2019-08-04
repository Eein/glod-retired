use chrono::Duration;

pub struct DurationFormatter;
impl DurationFormatter {
    pub fn time_container(time: Duration) -> (i64, i64, i64, i64, bool) {
        let duration = time.num_milliseconds();
        let milliseconds = duration % 1000;
        let seconds = (duration / 1000) % 60;
        let minutes = (duration / (1000*60)) % 60;
        let hours = (duration / (1000 * 60 * 60)) % 24;
        let mut positive = true;
        if milliseconds < 0 { positive = false }
        (hours.abs(), minutes.abs(), seconds.abs(), milliseconds.abs(), positive)
    }

    pub fn to_short(time: Duration) -> String {
        let tc = DurationFormatter::time_container(time);
        let formatted_hours = DurationFormatter::format_hours(tc);
        let formatted_minutes = DurationFormatter::format_minutes(tc);
        let formatted_seconds = DurationFormatter::format_seconds(tc);
        let formatted_positive = DurationFormatter::format_positive(tc);
        format!("{}{}{}{}", formatted_positive, formatted_hours, formatted_minutes, formatted_seconds)
    }

    pub fn to_string(time: Duration) -> String {
        let tc = DurationFormatter::time_container(time);
        let formatted_hours = DurationFormatter::format_hours(tc);
        let formatted_minutes = DurationFormatter::format_minutes(tc);
        let formatted_seconds = DurationFormatter::format_seconds(tc);
        let formatted_milliseconds = DurationFormatter::format_milliseconds(tc);
        let formatted_positive = DurationFormatter::format_positive(tc);
        format!("{}{}{}{}{}", formatted_positive, formatted_hours, formatted_minutes, formatted_seconds, formatted_milliseconds)
    }

    pub fn format_positive(tc: (i64,i64,i64,i64,bool)) -> String {
        match tc.4 {
            true => String::from(""),
            false => String::from("-")
        }
    }

    pub fn format_hours(tc: (i64,i64,i64,i64,bool)) -> String {
        let (h,_m,_s,_ms,_p) = tc;
        match h {
            0 => String::from(""),
            _ => format!("{}:", h)
        }
    }

    pub fn format_minutes(tc: (i64,i64,i64,i64,bool)) -> String {
        let (h,m,_s,_ms,_p) = tc;
        match m {
            0 => {
                match h {
                    0 => String::from(""),
                    _ => String::from("00:")
                }
            },
            1...9 => {
                match h {
                    0 => format!("{}:", m),
                    _ => format!("0{}:", m),
                }
            }
            _ => {
                format!("{}:", m)
            } 
        }
    }

    pub fn format_seconds(tc: (i64,i64,i64,i64,bool)) -> String {
        let (_h,m,s,_ms,_p) = tc;
        match s {
            0 => {
                match tc {
                    (0, 0, 0, _, _) => String::from("0"),
                    _ => String::from("00")
                }
            },
            1...9 => {
                match m {
                    0 => format!("{}", s),
                    _ => format!("0{}", s),
                }
            }
            _ => {
                format!("{}", s)
            } 
        }
    }

    pub fn format_milliseconds(tc: (i64,i64,i64,i64,bool)) -> String {
        let (_h,_m,_s,ms,_p) = tc;
        match ms {
            0 => String::from(".000"),
            100...999 => format!(".{}", ms),
            1...9 => format!(".00{}", ms),
            _ => format!(".0{}", ms)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ms_no_second_ms() {
        let dur = Duration::minutes(1);
        assert_eq!(DurationFormatter::to_string(dur), "1:00.000");
        assert_eq!(DurationFormatter::to_short(dur), "1:00");
        let dur = Duration::hours(1);
        assert_eq!(DurationFormatter::to_string(dur), "1:00:00.000");
        assert_eq!(DurationFormatter::to_short(dur), "1:00:00");
    }

    #[test]
    fn test_milliseconds_formatting() {
        assert_eq!(DurationFormatter::format_milliseconds((0,0,0,10,true)), ".010");
        assert_eq!(DurationFormatter::format_milliseconds((0,0,1,0,true)), ".000");
        assert_eq!(DurationFormatter::format_milliseconds((0,0,1,1,true)), ".001");
        assert_eq!(DurationFormatter::format_milliseconds((0,0,1,10,true)), ".010");
        assert_eq!(DurationFormatter::format_milliseconds((0,1,0,110,true)), ".110");
    }

    #[test]
    fn test_seconds_formatting() {
        assert_eq!(DurationFormatter::format_seconds((0,0,10,0,true)), "10");
        assert_eq!(DurationFormatter::format_seconds((0,0,0,0,true)), "0");
        assert_eq!(DurationFormatter::format_seconds((0,1,0,0,true)), "00");
        assert_eq!(DurationFormatter::format_seconds((0,1,1,0,true)), "01");
        assert_eq!(DurationFormatter::format_seconds((0,1,10,0,true)), "10");
        assert_eq!(DurationFormatter::format_seconds((0,1,0,123,true)), "00");
    }

    #[test]
    fn test_minutes_formatting() {
        assert_eq!(DurationFormatter::format_minutes((0,10,0,0,true)), "10:");
        assert_eq!(DurationFormatter::format_minutes((0,0,0,0,true)), "");
        assert_eq!(DurationFormatter::format_minutes((1,0,0,0,true)), "00:");
        assert_eq!(DurationFormatter::format_minutes((1,10,0,0,true)), "10:");
        assert_eq!(DurationFormatter::format_minutes((1,1,0,0,true)), "01:");
    }

    #[test]
    fn test_hours_formatting() {
        assert_eq!(DurationFormatter::format_hours((10,0,0,0,true)), "10:");
        assert_eq!(DurationFormatter::format_hours((0,0,0,0,true)), "");
        assert_eq!(DurationFormatter::format_hours((1,0,0,0,true)), "1:");
    }
}
