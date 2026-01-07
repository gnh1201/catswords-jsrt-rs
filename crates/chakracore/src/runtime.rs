use crate::error::{ok, Result};
use chakracore_sys as sys;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

pub struct Runtime {
    pub(crate) raw: sys::JsRuntimeHandle,

    // Keep callback_state pointers alive until runtime disposal
    callback_states: Arc<Mutex<Vec<*mut c_void>>>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let mut rt: sys::JsRuntimeHandle = std::ptr::null_mut();
        unsafe {
            ok(sys::JsCreateRuntime(
                sys::JsRuntimeAttributes::None,
                std::ptr::null_mut(),
                &mut rt,
            ))?;
        }

        Ok(Self {
            raw: rt,
            callback_states: Arc::new(Mutex::new(Vec::new())),
        })
    }

    // Register a callback_state pointer (allocated as Box<Box<Callback>> -> thin pointer)
    pub(crate) fn register_callback_state(&self, p: *mut c_void) {
        self.callback_states.lock().unwrap().push(p);
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                let _ = sys::JsDisposeRuntime(self.raw);
            }
            self.raw = std::ptr::null_mut();
        }

        // After disposing the runtime, it is safe to free callback states.
        let mut reg = self.callback_states.lock().unwrap();
        for p in reg.drain(..) {
            unsafe { crate::value::free_callback_state(p); }
        }
    }
}
