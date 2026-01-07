use crate::error::{ok, Result};
use crate::guard::Guard;
use crate::value::Value;
use crate::runtime::Runtime;
use chakracore_sys as sys;
use std::ffi::c_void;

pub struct CallInfo {
    pub arguments: Vec<Value>,
}

type Callback = dyn Fn(&Guard<'_>, CallInfo) -> Result<Value> + Send + Sync + 'static;

pub struct Function {
    v: Value,
    // Thin pointer to heap storage of a fat pointer (Box<dyn Trait>)
    state_ptr: *mut Box<Callback>,
}

impl Function {
    pub fn new(runtime: &Runtime, _guard: &Guard<'_>, cb: Box<Callback>) -> Self {
        // Box<Callback> is a fat pointer -> cannot cast to void* directly.
        // So allocate it again: Box<Box<Callback>> is a thin pointer.
        let outer: Box<Box<Callback>> = Box::new(cb);
        let state_ptr = Box::into_raw(outer) as *mut c_void;

        // runtime owns the callback_state lifetime
        runtime.register_callback_state(state_ptr);

        let mut func: sys::JsValueRef = std::ptr::null_mut();
        unsafe {
            let _ = ok(sys::JsCreateFunction(Some(native_trampoline), state_ptr, &mut func));
        }

        Self {
            v: Value { raw: func },
            state_ptr: state_ptr as *mut Box<Callback>,
        }
    }

    pub fn call(&self, _guard: &Guard<'_>, args: &[&Value]) -> Result<Value> {
        // ChakraCore requires argv[0] = thisArg. We'll use the function itself as thisArg.
        let mut argv: Vec<sys::JsValueRef> = Vec::with_capacity(args.len() + 1);
        argv.push(self.v.raw);
        for a in args {
            argv.push(a.raw);
        }

        let mut out: sys::JsValueRef = std::ptr::null_mut();
        unsafe {
            ok(sys::JsCallFunction(
                self.v.raw,
                argv.as_ptr(),
                argv.len() as u16,
                &mut out,
            ))?;
        }
        Ok(Value { raw: out })
    }

    pub fn into(self) -> Value {
        self.v
    }
}

impl Drop for Function {
    fn drop(&mut self) {
        // Do NOT free callback_state here.
        // Runtime will free all callback states after JsDisposeRuntime.
        self.state_ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" fn native_trampoline(
    _callee: sys::JsValueRef,
    _is_construct_call: bool,
    arguments: *const sys::JsValueRef,
    argument_count: u16,
    callback_state: *mut c_void,
) -> sys::JsValueRef {
    // Cast back to our thin pointer (Box<Box<Callback>>)
    let cb_ptr = callback_state as *mut Box<Callback>;
    let cb: &Callback = &**cb_ptr;

    // Assume the host already has a current context set via Guard.
    let mut current: sys::JsContextRef = std::ptr::null_mut();
    let _ = sys::JsGetCurrentContext(&mut current);

    let guard = Guard {
        prev: current,
        current,
        _marker: std::marker::PhantomData,
    };

    // Copy args
    let mut argv: Vec<Value> = Vec::with_capacity(argument_count as usize);
    if !arguments.is_null() {
        let slice = std::slice::from_raw_parts(arguments, argument_count as usize);
        for &a in slice {
            argv.push(Value { raw: a });
        }
    }

    // ChakraCore passes thisArg at argv[0]. The user's closure expects only user args.
    let user_args = if argv.len() >= 2 {
        argv[1..].to_vec()
    } else {
        Vec::new()
    };

    let info = CallInfo { arguments: user_args };

    match cb(&guard, info) {
        Ok(v) => v.raw,
        Err(e) => {
            // thiserror v2: Display is generated from #[error("...")]
            let msg = format!("{}", e);

            if let Ok(js_err) = Value::error_from_message(&guard, &msg) {
            let _ = sys::JsSetException(js_err.raw);
                return js_err.raw;
            }

            // Fallback to undefined if error creation fails
            let mut undef = std::ptr::null_mut();
            let _ = sys::JsGetUndefinedValue(&mut undef);
            let _ = sys::JsSetException(undef);
            undef
        }
    }
}

pub(crate) unsafe fn free_callback_state(p: *mut c_void) {
    // p was created from Box::into_raw(Box<Box<Callback>>) as *mut c_void
    let _outer: Box<Box<Callback>> = Box::from_raw(p as *mut Box<Callback>);
    // drop happens here
}
