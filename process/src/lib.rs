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

pub struct MuxContext {
    format_ctx: *mut ff::AVFormatContext,
}
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum CreateMuxContextError {
    #[snafu(display("error allocating MuxContext"))]
    AllocAVFormat,
    #[snafu(display("error opening input: {source}"))]
    OpenInput { source: AVError },
}
impl MuxContext {
    pub fn from_url_cstr(url: &CStr) -> Result<Self, CreateMuxContextError> {
        let mut format_ctx_ptr = unsafe { ff::avformat_alloc_context() };
        if format_ctx_ptr.is_null() {
            return Err(CreateMuxContextError::AllocAVFormat);
        }

        unsafe {
            ff::avformat_open_input(
                &mut format_ctx_ptr,
                url.as_ptr(),
                ptr::null(),     // TODO: allow changing input format
                ptr::null_mut(), // AVDictionary of options
            )
        }
        .map_averror()
        .context(OpenInputSnafu)?;

        Ok(Self { format_ctx: format_ctx_ptr } )
    }
}
impl Drop for MuxContext {
    fn drop(&mut self) {
        unsafe { ff::avformat_close_input(&mut self.format_ctx) };
    }
}
