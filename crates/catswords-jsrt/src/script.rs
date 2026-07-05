use crate::error::{ok, Result};
use crate::guard::Guard;
use crate::value::Value;
use catswords_jsrt_sys as sys;

pub fn eval(_guard: &Guard<'_>, code: &str) -> Result<Value> {
    unsafe {
        let mut script_val: sys::JsValueRef = std::ptr::null_mut();
        let mut url_val: sys::JsValueRef = std::ptr::null_mut();

        ok(sys::JsCreateString(
            code.as_ptr(),
            code.len(),
            &mut script_val,
        ))?;

        let url = "eval.js";
        ok(sys::JsCreateString(
            url.as_ptr(),
            url.len(),
            &mut url_val,
        ))?;

        let mut out: sys::JsValueRef = std::ptr::null_mut();
        ok(sys::JsRun(
            script_val,
            0 as sys::JsSourceContext,
            url_val,
            sys::JsParseScriptAttributes::None,
            &mut out,
        ))?;

        Ok(Value { raw: out })
    }
}
