#![cfg(feature = "_rdfox")]
use std::mem::ManuallyDrop;

#[repr(C)]
pub(crate) struct CParametersHandle(ManuallyDrop<*mut rdfox_sys::CParameters>);

impl Drop for CParametersHandle {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        unsafe {
            rdfox_sys::CParameters_destroy(self.0.cast());
        }
    }
}

impl std::fmt::Debug for CParametersHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CParametersHandle({:p})",
            self.0.cast::<*mut CParametersHandle>()
        )
    }
}

impl CParametersHandle {
    pub fn new() -> Result<Self, ekg_error::Error> {
        let mut c_params: *mut rdfox_sys::CParameters = std::ptr::null_mut();
        rdfox_sys::database_call!(
            "Allocating parameters",
            rdfox_sys::CParameters_newEmptyParameters(&mut c_params)
        )?;
        let result = CParametersHandle(ManuallyDrop::new(c_params));
        // tracing::trace!(target: LOG_TARGET_DATABASE, "Allocated {:?}",
        // result);
        Ok(result)
    }

    pub fn cast_const(&self) -> *const rdfox_sys::CParameters { self.0.cast_const() }

    pub fn cast_mut(&mut self) -> *mut rdfox_sys::CParameters { self.0.cast() }
}
