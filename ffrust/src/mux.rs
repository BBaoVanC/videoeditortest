use std::ffi::CStr;
use std::ptr;

use crate::ff;
use crate::AVError;
use crate::AllocContextError;
use crate::MapIntToResultAVError;

#[derive(Debug)]
pub struct InputContext {
    inner: *mut ff::AVFormatContext,
}
impl InputContext {
    /// Open a file with ffmpeg, and get format information using
    /// [`avformat_find_stream_info`][ff::avformat_find_stream_info].
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
        debug_assert!(!ctx.is_null());

        unsafe { ff::avformat_find_stream_info(ctx, ptr::null_mut()).map_averror()? };
        Ok(Self { inner: ctx })
    }
    // pub fn iformat
    // nb_streams
    // streams
}
impl Drop for InputContext {
    fn drop(&mut self) {
        let ptr = &raw mut self.inner;
        unsafe { ff::avformat_close_input(ptr) };
        debug_assert!(ptr.is_null());
    }
}


#[derive(Debug)]
pub struct IoOutputContext {
}
impl IoOutputContext {
    pub fn open(url: &CStr) -> Result<Self, AVError> {
        let mut ctx = ptr::null_mut();
        unsafe {
            ff::avio_open2(
                ctx,
                url.as_ptr(),
                todo!(),
                todo!(),
                todo!(),
            )
        }
        .map_averror()?;
        todo!()
    }
}

#[derive(Debug)]
pub struct OutputContext {
}
impl OutputContext {
    pub fn open(url: &CStr, format: ff::AVOutputFormat) -> Result<Self, AllocContextError> {
        let mut ctx = unsafe { ff::avformat_alloc_context() };
        if ctx.is_null() {
            return Err(AllocContextError);
        }

        todo!()
    }
}
impl Drop for OutputContext {
    fn drop(&mut self) {
        todo!()
    }
}
