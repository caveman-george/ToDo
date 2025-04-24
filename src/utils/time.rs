use chrono::TimeDelta;
use chrono::prelude::*;
use regex::Regex;

pub fn estimate_due_datetime(duration: &str) -> DateTime<Utc> {
    let re = Regex::new(r"(^[1-9][0-9]*)([mhdw])$").unwrap();
    let now = Utc::now();

    if re.is_match(duration) {
        let captures = re.captures(duration).unwrap();
        let value: i64 = captures[1].parse().unwrap();
        let unit = &captures[2];

        match unit {
            "m" => now + TimeDelta::minutes(value),
            "h" => now + TimeDelta::hours(value),
            "d" => now + TimeDelta::days(value),
            "w" => now + TimeDelta::weeks(value),
            _ => now,
        }
    } else {
        panic!("Invalid duration format");
    }
}
