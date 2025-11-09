mod dump_config_desc_ffi;
mod r_vector;
mod user_data_desc_ffi;


pub trait CWrapperFFI<T> {
    fn to_rust(&self) -> T;
}