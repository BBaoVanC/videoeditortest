//! Utilities for dealing with ffmpeg/libav API

use std::ffi::CStr;
use std::ffi::c_int;
use std::ffi::CString;

use rusty_ffmpeg::ffi as av;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(display("raw AVError from ffmpeg: {id}: {message:?}"))]
pub struct AVError {
    id: c_int,
    message: Option<CString>,
}
impl AVError {
    /// See <https://ffmpeg.org/doxygen/7.0/group__lavu__error.html#ga5792b4a2d18d7d9cb0efbcfc335dce24>
    pub fn new(id: c_int) -> Self {
        // add 1 for null
        const MESSAGE_LEN: usize = (av::AV_ERROR_MAX_STRING_SIZE + 1) as usize;
        let mut message = [0; MESSAGE_LEN];
        let message_ptr = message.as_mut_ptr();
        //let message = unsafe { CString::from_raw(message_ptr) };
        let message = if unsafe { av::av_strerror(id, message_ptr, MESSAGE_LEN) } < 0 {
            None
        } else {
            let s = unsafe { CStr::from_ptr(message.as_ptr()) }.to_owned();
            Some(s)
        };
        Self { id, message }
    }
}
pub trait MapIntToResultAVError {
    fn map_averror(self) -> Result<c_int, AVError>;
}
impl MapIntToResultAVError for c_int {
    fn map_averror(self) -> Result<c_int, AVError> {
        if self < 0 {
            Err(AVError::new(self))
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, Snafu)]
/// error allocating a context in ffmpeg
pub struct AllocContextError;
