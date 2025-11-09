use serde::{Deserialize, Serialize};

pub struct DumpConfigDesc {
    pub dump_directory: String,
    pub naming_pattern: String,
    pub user_data_desc: Vec<UserDataDesc>,
}

pub struct UserDataDesc {
    pub key: String,
    pub data_type: UserDataTypes,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum UserDataTypes {
    String = 0,
    Number,
    Boolean,
    Blob,
}


impl Default for DumpConfigDesc {
    fn default() -> DumpConfigDesc {
        DumpConfigDesc {
            dump_directory: String::from("/tmp/dumps"),
            naming_pattern: String::from("dump_#{TIME}.dmp"),
            user_data_desc: vec![],
        }
    }
}