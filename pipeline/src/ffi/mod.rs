use std::{
    ffi::{c_int, CStr, CString},
    ptr,
};

use rusty_ffmpeg::ffi as ff;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
#[snafu(display("raw AVError from ffmpeg: {id}: {message:?}"))]
pub struct AVError {
    id: c_int,
    message: Option<CString>,
}
impl AVError {
    // See https://ffmpeg.org/doxygen/7.0/group__lavu__error.html#ga5792b4a2d18d7d9cb0efbcfc335dce24
    pub fn new(id: c_int) -> Self {
        // add 1 for null
        const MESSAGE_LEN: usize = (ff::AV_ERROR_MAX_STRING_SIZE + 1) as usize;
        let mut message = [0; MESSAGE_LEN];
        let message_ptr = message.as_mut_ptr();
        let strerror_ret = unsafe { ff::av_strerror(id, message_ptr, MESSAGE_LEN) };
        //let message = unsafe { CString::from_raw(message_ptr) };
        let message = if strerror_ret != 0 {
            None
        } else {
            let s = unsafe { CStr::from_ptr(message.as_ptr()) }.to_owned();
            Some(s)
        };
        Self { id, message }
    }
}
trait MapIntToResultAVError {
    fn map_averror(self) -> Result<(), AVError>;
}
impl MapIntToResultAVError for c_int {
    fn map_averror(self) -> Result<(), AVError> {
        if self != 0 {
            Err(AVError::new(self))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct FormatContext {
    format_ctx: *mut ff::AVFormatContext,
}
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum AllocFormatContextError {
    #[snafu(display("error allocating FormatContext"))]
    AllocAVFormat,
}
impl FormatContext {
    pub fn new() -> Result<Self, AllocFormatContextError> {
        let format_ctx_ptr = unsafe { ff::avformat_alloc_context() };
        if format_ctx_ptr.is_null() {
            return Err(AllocFormatContextError::AllocAVFormat);
        }

        Ok(Self { format_ctx: format_ctx_ptr } )
    }
}
impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe { ff::avformat_close_input(&mut self.format_ctx) };
    }
}

#[derive(Debug)]
pub struct InputContext {
    inner: FormatContext,
}
impl InputContext {
    /// takes in a freshly allocated [`FormatContext`] from [`FormatContext::new`]
    pub fn new(mut ctx: FormatContext, url: &CStr) -> Result<Self, AVError> {
        unsafe {
            ff::avformat_open_input(
                &mut ctx.format_ctx,
                url.as_ptr(),
                ptr::null(),     // TODO: allow changing input format
                ptr::null_mut(), // AVDictionary of options
            )
        }
        .map_averror()?;
        Ok(Self { inner: ctx })
    }
}

#[derive(Debug)]
pub struct OutputContext {
    inner: FormatContext,
}
impl OutputContext {
    /// takes in a freshly allocated [`FormatContext`] from [`FormatContext::new`]
    pub fn new(mut ctx: FormatContext, url: &CStr) -> Result<Self, AVError> {
        ctx = todo!();
        Ok(Self { inner: ctx })
    }
}
