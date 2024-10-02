use std::ffi::CString;

mod ffi;

#[derive(Debug)]
pub struct ContainerHandle {
    inner: ffi::MuxContext,
}
impl ContainerHandle {
    // TODO: cstr version for optimization if static string
    // TODO: dont unwrap CString construction
    pub fn new_from_ffmpeg_url(url: &str) -> Result<Self, ffi::CreateMuxContextError> {
        let inner = ffi::MuxContext::from_url_cstr(&CString::new(url).unwrap())?;
        Ok(Self { inner })
    }
}
