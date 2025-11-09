use crate::dump::dump_config_desc::UserDataDesc;
use crate::ffi::CWrapperFFI;

#[repr(C)]
#[derive(Clone)]
pub struct UserDataDescFFI {
    pub key: *const std::ffi::c_char,
    pub data_type: i32,
}

impl CWrapperFFI<UserDataDesc> for UserDataDescFFI {
    fn to_rust(&self) -> UserDataDesc {
        use std::ffi::CStr;
        let key_cstr = unsafe { CStr::from_ptr(self.key) };
        let key_str = key_cstr.to_string_lossy().into_owned();

        let data_type = match self.data_type {
            0 => crate::dump::dump_config_desc::UserDataTypes::String,
            1 => crate::dump::dump_config_desc::UserDataTypes::Number,
            2 => crate::dump::dump_config_desc::UserDataTypes::Boolean,
            3 => crate::dump::dump_config_desc::UserDataTypes::Blob,
            _ => panic!("Invalid data_type value: {}", self.data_type),
        };

        UserDataDesc {
            key: key_str,
            data_type,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dump::dump_config_desc::{UserDataDesc, UserDataTypes};
    use crate::ffi::CWrapperFFI;

    #[test]
    fn test_user_data_desc_ffi_to_rust() {
        use super::*;
        use std::ffi::CString;

        let key = CString::new("key").unwrap();
        let data_type = 0;

        let c_type = UserDataDescFFI { key: key.as_ptr(), data_type };
        let rust_type = UserDataDesc{ key: String::from("key"), data_type: UserDataTypes::String };
        let c_rust_type = c_type.to_rust();

        assert_eq!(c_rust_type.key, rust_type.key);
        assert_eq!(c_rust_type.data_type, rust_type.data_type);

    }
}