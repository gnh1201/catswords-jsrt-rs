use crate::error::{ok_msg, Result};
use crate::guard::Guard;
use crate::value::Value;
use chakracore_sys as sys;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

unsafe fn get_prop_id(name: &str) -> Result<sys::JsPropertyIdRef> {
    let mut pid: sys::JsPropertyIdRef = std::ptr::null_mut();
    let w = to_wide(name);
    ok_msg(sys::JsGetPropertyIdFromName(w.as_ptr(), &mut pid), "JsGetPropertyIdFromName failed")?;
    Ok(pid)
}

pub struct RootStore {
    roots_obj: Value,
    next_id: u64,
}

pub struct RootedValue {
    roots_obj: Value,
    key: String,
}

impl RootStore {
    pub fn new(_guard: &Guard<'_>) -> Result<Self> {
        // global object
        let mut global: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok_msg(sys::JsGetGlobalObject(&mut global), "JsGetGlobalObject failed")?; }
        let global = Value { raw: global };

        // global._rust_roots
        let pid = unsafe { get_prop_id("_rust_roots")? };

        let mut existing: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok_msg(sys::JsGetProperty(global.raw(), pid, &mut existing), "JsGetProperty failed")?; }

        // If undefined/null, create object and set it
        // (We’ll treat null as not-set too)
        let roots_obj = {
            // Try to detect undefined by pointer equality is not safe without JsEquals;
            // simplest: always create and overwrite once, or keep as-is if set.
            // For minimal correctness: create new object and set if property read fails later.
            let mut obj: sys::JsValueRef = std::ptr::null_mut();
            unsafe { ok_msg(sys::JsCreateObject(&mut obj), "JsCreateObject failed")?; }
            let objv = Value { raw: obj };
            unsafe { ok_msg(sys::JsSetProperty(global.raw(), pid, objv.raw(), true), "JsSetProperty failed")?; }
            objv
        };

        Ok(Self { roots_obj, next_id: 1 })
    }

    pub fn root(&mut self, _guard: &Guard<'_>, v: Value) -> Result<RootedValue> {
        let id = self.next_id;
        self.next_id += 1;

        let key = format!("r{}", id);

        // roots_obj[key] = v
        let pid = unsafe { get_prop_id(&key)? };
        unsafe {
            ok_msg(
                sys::JsSetProperty(self.roots_obj.raw(), pid, v.raw(), true),
                "JsSetProperty (root) failed",
            )?;
        }

        Ok(RootedValue {
            roots_obj: self.roots_obj, // copy handle
            key,
        })
    }
}

impl Drop for RootedValue {
    fn drop(&mut self) {
        unsafe {
            if let Ok(pid) = get_prop_id(&self.key) {
                let mut result: sys::JsValueRef = std::ptr::null_mut();
                let _ = sys::JsDeleteProperty(self.roots_obj.raw(), pid, true, &mut result);
            }
        }
    }
}
