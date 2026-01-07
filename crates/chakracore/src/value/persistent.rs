use crate::error::{ok_msg, Result};
use crate::value::Value;
use chakracore_sys as sys;

pub struct PersistentValue {
    raw: sys::JsValueRef,
}

impl PersistentValue {
    pub fn new(v: Value) -> Result<Self> {
        let mut count: u32 = 0;
        unsafe {
            ok_msg(sys::JsAddRef(v.raw(), &mut count), "JsAddRef failed")?;
        }
        Ok(Self { raw: v.raw() })
    }

    pub fn as_value(&self) -> Value {
        Value { raw: self.raw }
    }
}

impl Drop for PersistentValue {
    fn drop(&mut self) {
        let mut count: u32 = 0;
        unsafe {
            let _ = sys::JsRelease(self.raw, &mut count);
        }
    }
}
