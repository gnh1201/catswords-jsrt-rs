use crate::error::{ok, ok_msg, Result};
use crate::guard::Guard;
use crate::runtime::Runtime;
use crate::value::Value;
use catswords_jsrt_sys as sys;

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub struct Context<'rt> {
    pub(crate) raw: sys::JsContextRef,
    runtime: &'rt Runtime,
}

impl<'rt> Context<'rt> {
    pub fn new(runtime: &'rt Runtime) -> Result<Self> {
        let mut cx: sys::JsContextRef = std::ptr::null_mut();
        unsafe { ok(sys::JsCreateContext(runtime.raw, &mut cx))?; }
        Ok(Self { raw: cx, runtime })
    }

    pub fn make_current(&self) -> Result<Guard<'rt>> {
        let mut prev: sys::JsContextRef = std::ptr::null_mut();
        unsafe {
            ok(sys::JsGetCurrentContext(&mut prev))?;
            ok(sys::JsSetCurrentContext(self.raw))?;
        }
        Ok(Guard {
            prev,
            current: self.raw,
            runtime: self.runtime,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn set_global(&self, _guard: &Guard<'_>, name: &str, value: &Value) -> Result<()> {
        let mut global: sys::JsValueRef = std::ptr::null_mut();
        unsafe { ok_msg(sys::JsGetGlobalObject(&mut global), "JsGetGlobalObject failed")?; }

        let w = to_wide(name);
        let mut pid: sys::JsPropertyIdRef = std::ptr::null_mut();
        unsafe {
            ok_msg(
                sys::JsGetPropertyIdFromName(w.as_ptr(), &mut pid),
                "JsGetPropertyIdFromName failed",
            )?;
            ok_msg(
                sys::JsSetProperty(global, pid, value.raw(), true),
                "JsSetProperty failed",
            )?;
        }
        Ok(())
    }

    pub(crate) fn from_raw(runtime: &'rt Runtime, raw: sys::JsContextRef) -> Self {
        Self { raw, runtime }
    }
}
