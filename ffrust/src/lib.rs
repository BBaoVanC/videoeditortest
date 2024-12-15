#![expect(unused_imports)]

use std::{
    ffi::{c_int, CStr, CString},
    ptr,
};

use rusty_ffmpeg::ffi as ff;
use snafu::{ResultExt, Snafu};

pub mod mux;
