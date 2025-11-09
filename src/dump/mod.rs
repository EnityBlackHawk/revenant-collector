pub mod dump_config_desc;
pub mod dump_generator;

use std::fmt::{Display, Formatter};
use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::dump::dump_config_desc::UserDataTypes;
use crate::stack_entry::StackEntry;

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S %:z";

#[derive(Serialize, Deserialize, Debug)]
pub struct Occurrence {
    date_time : String,
    stack : Vec<StackEntry>,
    user_data: Vec<UserData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    key: String,
    data_type: UserDataTypes,
    value: Value,
}

impl Occurrence {
    pub fn new(date_time : String, stack : Vec<StackEntry>) -> Occurrence {
        Occurrence{date_time, stack, user_data: vec![]}
    }

    pub fn new_from_dt(date_time : DateTime<FixedOffset>, stack : Vec<StackEntry>) -> Occurrence {
        Occurrence { date_time: date_time.format(DATETIME_FORMAT).to_string(), stack, user_data: vec![] }
    }

    pub fn date_time(&self) -> &str {
        &self.date_time
    }
    pub fn date_time_obj(&self ) -> Result<DateTime<FixedOffset>, &str> {
        match DateTime::parse_from_str(&self.date_time, DATETIME_FORMAT) {
            Ok(dt) => Ok(dt),
            Err(x) => Err(self.date_time())
        }

    }
    pub fn stack(&self) -> &Vec<StackEntry> {
        &self.stack
    }
}

impl Display for Occurrence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use chrono::{Timelike, Utc};
    use crate::dump::Occurrence;

    #[test]
    fn test_occurrence_print() {
        use super::*;
        let stack = StackEntry::new("func".to_string(), "file.rs".to_string(), "42".to_string());
        let brt = FixedOffset::west_opt(3 * 3600).unwrap();
        let now = Utc::now().with_timezone(&brt);
        let occurrence = Occurrence::new(now.format(DATETIME_FORMAT).to_string(), vec!(stack));
        println!("{}", occurrence);

    }
    #[test]
    fn test_occurrence_load() {

        let data = r#"{"date_time":"2025-11-08 19:28:27 -03:00","stack":[{"name":"func","file":"file.rs","line":"42"}], "user_data": []}"#;
        let occ : Occurrence = serde_json::from_str(data).unwrap();
        assert_eq!(occ.stack[0].to_string(), "func on file.rs:42");
        assert_eq!(occ.date_time_obj().unwrap().hour(), 19);

    }

}