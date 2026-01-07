#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ffi::c_void;

pub type JsRuntimeHandle = *mut c_void;
pub type JsContextRef = *mut c_void;
pub type JsValueRef = *mut c_void;
pub type JsPropertyIdRef = *mut c_void;
pub type JsSourceContext = usize;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsErrorCode {
    JsNoError = 0,

    // Basic argument/context errors
    JsErrorInvalidArgument = 0x10001,
    JsErrorNullArgument = 0x10002,
    JsErrorNoCurrentContext = 0x10003,
    JsErrorInExceptionState = 0x10004,

    // Script errors
    JsErrorScriptException = 0x30001,
    JsErrorScriptCompile = 0x30002,
    JsErrorScriptTerminated = 0x30003,
    JsErrorScriptEvalDisabled = 0x30004,

    // Fatal
    JsErrorFatal = 0x40000,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum JsRuntimeAttributes {
    None = 0,
}

pub type JsNativeFunction = Option<
    unsafe extern "C" fn(
        callee: JsValueRef,
        is_construct_call: bool,
        arguments: *const JsValueRef,
        argument_count: u16,
        callback_state: *mut c_void,
    ) -> JsValueRef
>;

extern "C" {
    pub fn JsCreateRuntime(
        attributes: JsRuntimeAttributes,
        thread_service: *mut c_void,
        runtime: *mut JsRuntimeHandle,
    ) -> JsErrorCode;

    pub fn JsDisposeRuntime(runtime: JsRuntimeHandle) -> JsErrorCode;

    pub fn JsCreateContext(
        runtime: JsRuntimeHandle,
        new_context: *mut JsContextRef,
    ) -> JsErrorCode;

    pub fn JsSetCurrentContext(context: JsContextRef) -> JsErrorCode;
    pub fn JsGetCurrentContext(current_context: *mut JsContextRef) -> JsErrorCode;

    pub fn JsRunScript(
        script: *const u16,
        source_context: JsSourceContext,
        source_url: *const u16,
        result: *mut JsValueRef,
    ) -> JsErrorCode;

    pub fn JsIntToNumber(value: i32, result: *mut JsValueRef) -> JsErrorCode;
    pub fn JsNumberToInt(value: JsValueRef, result: *mut i32) -> JsErrorCode;

    pub fn JsCreateFunction(
        native_function: JsNativeFunction,
        callback_state: *mut c_void,
        function: *mut JsValueRef,
    ) -> JsErrorCode;

    pub fn JsCallFunction(
        function: JsValueRef,
        arguments: *const JsValueRef,
        argument_count: u16,
        result: *mut JsValueRef,
    ) -> JsErrorCode;
	
    pub fn JsCreateString(
        content: *const u8,
        length: usize,
        value: *mut JsValueRef,
    ) -> JsErrorCode;

    pub fn JsCreateError(message: JsValueRef, error: *mut JsValueRef) -> JsErrorCode;

    pub fn JsCreateTypeError(message: JsValueRef, error: *mut JsValueRef) -> JsErrorCode;

    pub fn JsAddRef(reference: JsValueRef, count: *mut u32) -> JsErrorCode;
    pub fn JsRelease(reference: JsValueRef, count: *mut u32) -> JsErrorCode;

    pub fn JsGetGlobalObject(global_object: *mut JsValueRef) -> JsErrorCode;

    pub fn JsGetPropertyIdFromName(name: *const u16, property_id: *mut JsPropertyIdRef)
        -> JsErrorCode;

    pub fn JsGetProperty(object: JsValueRef, property_id: JsPropertyIdRef, value: *mut JsValueRef)
        -> JsErrorCode;

    pub fn JsSetProperty(object: JsValueRef, property_id: JsPropertyIdRef, value: JsValueRef, use_strict: bool)
        -> JsErrorCode;

    pub fn JsCreateObject(object: *mut JsValueRef) -> JsErrorCode;

    pub fn JsDeleteProperty(object: JsValueRef, property_id: JsPropertyIdRef, use_strict: bool, result: *mut JsValueRef)
        -> JsErrorCode;

    // Recommended additions for better ergonomics / correctness
    pub fn JsGetUndefinedValue(undefined_value: *mut JsValueRef) -> JsErrorCode;
    pub fn JsGetNullValue(null_value: *mut JsValueRef) -> JsErrorCode;
    pub fn JsSetException(exception: JsValueRef) -> JsErrorCode;
}
