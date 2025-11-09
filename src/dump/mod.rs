mod dump_config_decs;

use std::fmt::{Display, Formatter};
use chrono::{DateTime, FixedOffset, NaiveDateTime, ParseResult};
use serde::{Serialize, Deserialize};
use crate::stack_entry::StackEntry;

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S %:z";

#[derive(Serialize, Deserialize, Debug)]
pub struct Occurrence {
    date_time : String,
    stack : StackEntry
}

impl Occurrence {
    pub fn new(date_time : String, stack : StackEntry) -> Occurrence {
        Occurrence{date_time, stack}
    }

    pub fn new_from_dt(date_time : DateTime<FixedOffset>, stack : StackEntry) -> Occurrence {
        Occurrence{date_time : date_time.format(DATETIME_FORMAT).to_string(), stack}
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
    pub fn stack(&self) -> &StackEntry {
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
        let occurrence = Occurrence::new(now.format(DATETIME_FORMAT).to_string(), stack);
        println!("{}", occurrence);

    }
    #[test]
    fn test_occurrence_load() {

        let data = r#"{"date_time":"2025-11-08 19:28:27 -03:00","stack":{"name":"func","file":"file.rs","line":"42"}}"#;
        let occ : Occurrence = serde_json::from_str(data).unwrap();
        assert_eq!(occ.stack.to_string(), "func on file.rs:42");
        assert_eq!(occ.date_time_obj().unwrap().hour(), 19);

    }

}