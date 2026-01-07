use chakracore_sys::JsErrorCode;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
#[error("{message} (code={code:?})")]
pub struct Error {
    pub code: JsErrorCode,
    pub message: Cow<'static, str>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub(crate) fn ok(code: JsErrorCode) -> Result<()> {
    ok_msg(code, "ChakraCore JsRT call failed")
}

#[inline]
pub(crate) fn ok_msg(code: JsErrorCode, msg: &'static str) -> Result<()> {
    if code == JsErrorCode::JsNoError {
        Ok(())
    } else {
        Err(Error {
            code,
            message: Cow::Borrowed(msg),
        })
    }
}
