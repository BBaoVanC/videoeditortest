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
    /// See https://ffmpeg.org/doxygen/7.0/group__lavu__error.html#ga5792b4a2d18d7d9cb0efbcfc335dce24
    pub fn new(id: c_int) -> Self {
        // add 1 for null
        const MESSAGE_LEN: usize = (ff::AV_ERROR_MAX_STRING_SIZE + 1) as usize;
        let mut message = [0; MESSAGE_LEN];
        let message_ptr = message.as_mut_ptr();
        //let message = unsafe { CString::from_raw(message_ptr) };
        let message = if unsafe { ff::av_strerror(id, message_ptr, MESSAGE_LEN) } < 0 {
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
        if self < 0 {
            Err(AVError::new(self))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("error allocating some context in ffmpeg"))]
pub struct AllocContextError;

//#[derive(Debug)]
//pub struct FormatContext {
//    inner: *mut ff::AVFormatContext,
//}
//impl FormatContext {
//    pub fn alloc() -> Result<Self, AllocContextError> {
//        let ptr = unsafe { ff::avformat_alloc_context() };
//        if ptr.is_null() { return Err(AllocContextError); };
//        Ok(Self { inner: ptr } )
//    }
//}
//impl Drop for FormatContext {
//    fn drop(&mut self) {
//        // TODO: close correctly
//        //unsafe { ff::avformat_close_input(&mut self.format_ctx) };
//    }
//}

#[derive(Debug)]
pub struct InputContext {
    inner: *mut ff::AVFormatContext,
}
impl InputContext {
    pub fn open(url: &CStr) -> Result<Self, AVError> {
        let mut ctx = ptr::null_mut();
        unsafe {
            ff::avformat_open_input(
                &raw mut ctx, // tell ffmpeg to create an AVFormatContext itself
                url.as_ptr(),
                ptr::null(),     // TODO: allow changing input format
                ptr::null_mut(), // AVDictionary of options
            )
        }
        .map_averror()?;
        Ok(Self { inner: ctx })
    }
}
impl Drop for InputContext {
    fn drop(&mut self) {
        unsafe { ff::avformat_close_input(&raw mut self.inner) };
    }
}

//#[derive(Debug)]
//pub struct OutputContext {
//    inner: FormatContext,
//}
//impl OutputContext {
//    /// takes in a freshly allocated [`FormatContext`] from [`FormatContext::new`]
//    pub fn new(mut ctx: FormatContext, url: &CStr) -> Result<Self, AVError> {
//        ctx = todo!();
//        Ok(Self { inner: ctx })
//    }
//}
