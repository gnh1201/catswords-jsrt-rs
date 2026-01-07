use crate::error::{ok, ok_msg, Result};
use crate::guard::Guard;
use chakracore_sys as sys;

#[derive(Clone, Copy)]
pub struct Value {
    pub(crate) raw: sys::JsValueRef,
}

impl Value {
    pub fn raw(&self) -> sys::JsValueRef {
        self.raw
    }

    // Recommended: return Result instead of silently swallowing conversion errors.
    pub fn to_integer(&self, _guard: &Guard<'_>) -> Result<i32> {
        let mut out: i32 = 0;
        unsafe {
            ok(sys::JsNumberToInt(self.raw, &mut out))?;
        }
        Ok(out)
    }

    pub fn undefined(_guard: &Guard<'_>) -> Result<Self> {
        let mut v: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok(sys::JsGetUndefinedValue(&mut v))?; }
        Ok(Self { raw: v })
    }

    pub fn null(_guard: &Guard<'_>) -> Result<Self> {
        let mut v: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok(sys::JsGetNullValue(&mut v))?; }
        Ok(Self { raw: v })
    }
	
    pub fn string_utf8(_guard: &Guard<'_>, s: &str) -> Result<Self> {
        let mut out: sys::JsValueRef = std::ptr::null_mut();
        unsafe {
            ok_msg(sys::JsCreateString(s.as_ptr(), s.len(), &mut out), "JsCreateString failed")?;
        }
        Ok(Self { raw: out })
    }

    pub fn error_from_message(guard: &Guard<'_>, msg: &str) -> Result<Self> {
        let message = Self::string_utf8(guard, msg)?;
        let mut out: sys::JsValueRef = std::ptr::null_mut();
        unsafe {
            ok_msg(sys::JsCreateError(message.raw, &mut out), "JsCreateError failed")?;
        }
        Ok(Self { raw: out })
    }

    pub fn type_error_from_message(guard: &Guard<'_>, msg: &str) -> Result<Self> {
        let message = Self::string_utf8(guard, msg)?;
        let mut out: sys::JsValueRef = std::ptr::null_mut();
        unsafe {
            ok_msg(sys::JsCreateTypeError(message.raw, &mut out), "JsCreateTypeError failed")?;
        }
        Ok(Self { raw: out })
    }
}
