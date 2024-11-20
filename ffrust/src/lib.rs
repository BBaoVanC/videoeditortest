use ffi::AllocFormatContextError;
use ffi::AVError;

use snafu::Snafu;
use snafu::ResultExt;

use std::ffi::CString;

mod ffi;

#[derive(Debug)]
pub struct InputHandle {
    inner: ffi::InputContext,
}
#[derive(Debug, Snafu)]
pub enum OpenContainerError {
    #[snafu(display("error allocating context: {source}"))]
    AllocContext { source: AllocFormatContextError },

    #[snafu(display("error opening input file/url: {source}"))]
    OpenInput { source: AVError },
    #[snafu(display("error opening output file/url: {source}"))]
    OpenOutput { source: AVError },
}
impl InputHandle {
    // TODO: cstr version for optimization if static string
    pub fn open_input_url(url: &str) -> Result<Self, OpenContainerError> {
        let fmt_ctx = ffi::FormatContext::new().context(AllocContextSnafu)?;
        let url_cstr = CString::new(url).unwrap(); // TODO: don't unwrap
        let inner = ffi::InputContext::new(fmt_ctx, &url_cstr).context(OpenInputSnafu)?;
        Ok(Self { inner })
    }
}

#[derive(Debug)]
pub struct OutputHandle {

}
impl OutputHandle {
}
