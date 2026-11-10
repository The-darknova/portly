use crate::error::CoreError;

pub fn terminate_process(pid: u32, force: bool) -> Result<(), CoreError> {
    crate::os::terminate_process_impl(pid, force)
}

pub fn terminate_with_elevation(pid: u32, force: bool) -> Result<(), CoreError> {
    crate::os::terminate_with_elevation_impl(pid, force)
}
