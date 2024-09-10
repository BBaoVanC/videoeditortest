use rusty_ffmpeg::ffi as ff;
use snafu::Snafu;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_int;
use std::ptr;

trait MapIntToResultAVError {
    fn map_averror(self) -> Result<(), AVError>;
}
impl MapIntToResultAVError for c_int {
    fn map_averror(self) -> Result<(), AVError> {
        let i = self;
        if i != 0 { Err(AVError::new(i)) } else { Ok(()) }
    }
}

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
        const message_buf_len: usize = (ff::AV_ERROR_MAX_STRING_SIZE + 1) as usize;
        let message = CString::new([0; message_buf_len]).unwrap();
        let message_ptr = message.into_raw();
        let strerror_ret = unsafe { ff::av_strerror(id, message_ptr, message_buf_len) };
        let message = unsafe { CString::from_raw(message_ptr) };
        let message = if strerror_ret != 0 { None } else { Some(message) };
        Self { id, message }
    }
}

pub struct MuxContext {
    format_ctx: ff::AVFormatContext,
}
#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum CreateMuxContextError {
    #[snafu(display("error allocating MuxContext: {source}"))]
    AllocAVFormat { source: AVError },
    #[snafu(display("error opening input: {source}"))]
    OpenInput { source: AVError },
}
impl MuxContext {
    pub fn from_url(url: &CStr) -> Result<Self, CreateMuxContextError> {
        let mut format_ctx_ptr = unsafe { ff::avformat_alloc_context() };
        if format_ctx_ptr.is_null() {
            return Err(CreateMuxContextError::AllocAVFormat { source });
        }

        unsafe {
            ff::avformat_open_input(
                &mut format_ctx_ptr,
                url.as_ptr(),
                ptr::null(), // TODO: allow changing input format
                ptr::null_mut(), // AVDictionary of options
            )
        }.map_averror().context(OpenInputSnafu)?;

        todo!()
    }
}
