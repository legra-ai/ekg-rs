#![cfg(feature = "_rdfox")]

use std::mem::ManuallyDrop;

#[repr(C)]
pub(crate) struct CPrefixesHandle(ManuallyDrop<*mut rdfox_sys::CPrefixes>);

impl Drop for CPrefixesHandle {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            rdfox_sys::CPrefixes_destroy(self.0.cast());
        }
    }
}

impl std::fmt::Debug for CPrefixesHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPrefixesHandle({:p})",
            self.0.cast::<*mut CPrefixesHandle>()
        )
    }
}

impl CPrefixesHandle {
    pub fn new() -> Result<Self, ekg_error::Error> {
        let mut c_params: *mut rdfox_sys::CPrefixes = std::ptr::null_mut();
        rdfox_sys::database_call!(
            "Allocating prefixes",
            rdfox_sys::CPrefixes_newEmptyPrefixes(&mut c_params)
        )?;
        let result = CPrefixesHandle(ManuallyDrop::new(c_params));
        // tracing::debug!(target: LOG_TARGET_DATABASE, "Allocated {:?}",
        // result);
        Ok(result)
    }

    pub fn cast_mut(&mut self) -> *mut rdfox_sys::CPrefixes { self.0.cast() }
}
