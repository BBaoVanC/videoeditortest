use crate::TimeRational;

use std::error::Error as StdError;

/// sorta a wrapper around a buffer or something
#[non_exhaustive]
pub enum Frame {

}

pub trait VideoRender {
    type Output;
    type Error: StdError;
    fn render_frame(&mut self) -> Result<Self::Output, Self::Error>;
}

// TODO: should there be another type that's allowed to seek to a slightly different time than
// requested, or should that be abstracted away always?
pub trait SeekPrecise {
    type Error: StdError;
    /// Returns [`TimeRational`] of the actual time that is seeked to.
    ///
    fn seek(&mut self, dst: TimeRational) -> Result<(), Self::Error>;
}

/// A fixed-length video clip
pub struct VideoClipFixed {
}
