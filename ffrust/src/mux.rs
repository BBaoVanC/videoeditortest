use std::ffi::CStr;
use std::ptr;

use crate::ff;
use crate::AVError;
use crate::AllocContextError;
use crate::MapIntToResultAVError;


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
