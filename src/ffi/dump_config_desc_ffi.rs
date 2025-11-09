use crate::dump::dump_config_desc::{DumpConfigDesc, UserDataDesc};
use crate::ffi::CWrapperFFI;
use crate::ffi::r_vector::RVector;
use crate::ffi::user_data_desc_ffi::UserDataDescFFI;

pub struct DumpConfigDescFFI {
    pub dump_directory: *const std::ffi::c_char,
    pub naming_pattern: *const std::ffi::c_char,
    pub user_data_desc: RVector<UserDataDescFFI>,
}

impl CWrapperFFI<DumpConfigDesc> for DumpConfigDescFFI {
    fn to_rust(&self) -> DumpConfigDesc {
        use std::ffi::CStr;

        let dump_directory_cstr = unsafe { CStr::from_ptr(self.dump_directory) };
        let dump_directory_str = dump_directory_cstr.to_string_lossy().into_owned();

        let naming_pattern_cstr = unsafe { CStr::from_ptr(self.naming_pattern) };
        let naming_pattern_str = naming_pattern_cstr.to_string_lossy().into_owned();

        let user_data_desc_rust: Vec<UserDataDesc> = self
            .user_data_desc
            .to_rust()
            .into_iter()
            .map(|udi_ffi| udi_ffi.to_rust())
            .collect();

        DumpConfigDesc {
            dump_directory: dump_directory_str,
            naming_pattern: naming_pattern_str,
            user_data_desc: user_data_desc_rust,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn converts_valid_ffi_to_rust() {
        let dump_directory = CString::new("/tmp/dump").unwrap();
        let naming_pattern = CString::new("pattern_*").unwrap();
        let user_data_desc = RVector::from(vec![]);

        let ffi = DumpConfigDescFFI {
            dump_directory: dump_directory.as_ptr(),
            naming_pattern: naming_pattern.as_ptr(),
            user_data_desc,
        };

        let rust_struct = ffi.to_rust();

        assert_eq!(rust_struct.dump_directory, "/tmp/dump");
        assert_eq!(rust_struct.naming_pattern, "pattern_*");
        assert_eq!(rust_struct.user_data_desc.len(), 0);
    }

}