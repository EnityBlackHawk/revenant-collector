use chrono::{DateTime, Local};
use crate::dump::dump_config_desc::DumpConfigDesc;
use crate::dump::Occurrence;
use crate::stack_entry::StackEntry;

pub fn generate_dump(config : DumpConfigDesc, stack_entries: Vec<StackEntry>) -> Result<(), std::io::Error> {

    let occ = Occurrence::new_from_dt(
        DateTime::from(Local::now()),
        stack_entries
    );
    //TODO Add user data according to config

    let dump_filename = resolve_naming_pattern(&config.naming_pattern);
    let dump_path = std::path::Path::new(&config.dump_directory).join(dump_filename);
    std::fs::write(dump_path, occ.to_string())

}

fn resolve_naming_pattern(pattern: &str) -> String {
    let now = Local::now();
    let time_str = now.format("%Y%m%d_%H%M%S").to_string();
    pattern.replace("#{TIME}", &time_str)
}

#[cfg(test)]
mod tests {
    use crate::dump::dump_config_desc::DumpConfigDesc;
    use crate::stack_entry::StackEntry;

    #[test]
    fn test_generate_dump() {
        let config = DumpConfigDesc {
            dump_directory: String::from("/tmp"),
            naming_pattern: String::from("dump_#{TIME}.dmp"),
            user_data_desc: vec![],
        };

        let stack_entries = vec![
            StackEntry::new(String::from("func1"), String::from("file1.rs"), String::from("10")),
            StackEntry::new(String::from("func2"), String::from("file2.rs"), String::from("20")),
        ];

        let result = super::generate_dump(config, stack_entries);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_naming_pattern() {
        let pattern = "dump_#{TIME}.dmp";
        let resolved = super::resolve_naming_pattern(pattern);
        assert!(resolved.starts_with("dump_"));
        assert!(resolved.ends_with(".dmp"));
        assert_eq!(resolved.len(), "dump_YYYYMMDD_HHMMSS.dmp".len());
    }

}

