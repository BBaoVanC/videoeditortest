//! Monads for muxing and demuxing media containers

use std::{ffi::CStr, ptr};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use rusty_ffmpeg::ffi as av;

use crate::av_util::{AVError, MapIntToResultAVError};


/// A container of media. It acts as an input, providing frames to enter the
/// pipeline.
///
/// The streams are not split here, you will need to use ... to get at a
/// specific stream.
pub trait MediaSource {
    type Output;
}
// type Output
// fn read


/// Handle to an input media file. Basically like an [`Iterator`] but yields
/// frames of media instead.
// FIXME: buffering i/o on other thread
// FIXME: possibly async api instead
#[derive(Debug)]
pub struct InputFile {
    /// signal to the i/o thread whether to stop
    stop_tx: mpsc::Sender<()>,
    /// buffers packets from the i/o thread which is blocking in ffmpeg
    queue: Arc<Mutex<VecDeque<av::AVPacket>>>,
}
impl InputFile {
    pub fn open_cstr(url: &CStr) -> Result<Self, AVError> {
        let mut ctx = ptr::null_mut();
        unsafe {
            av::avformat_open_input(
                &raw mut ctx, // tell ffmpeg to create an AVFormatContext itself
                url.as_ptr(),
                ptr::null(),     // TODO: allow changing input format
                ptr::null_mut(), // AVDictionary of options
            )
        }
        .map_averror()?;
        debug_assert!(!ctx.is_null());

        unsafe { av::avformat_find_stream_info(ctx, ptr::null_mut()).map_averror()? };

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        // TODO: should this be synchronous so Drop is synchronous?
        let (stop_tx, stop_rx) = mpsc::channel();

        // TODO: handle error instead of panic using Builder::spawn
        thread::spawn(move || {
            let queue = &queue;
            loop {
                match stop_rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => break,
                    Err(mpsc::TryRecvError::Empty) => {
                        unsafe { av::
                        queue.
                    }
                }
            }
        });

        Ok(Self { ctx })
    }
}
impl Drop for InputFile {
    fn drop(&mut self) {
        let ptr = &raw mut self.ctx;
        unsafe { av::avformat_close_input(ptr) };
        debug_assert!(ptr.is_null());
    }
}

/// Objects that take media and output it to a file or somewhere.
pub trait MediaSink {
    // type Input
    // fn write
}

/// A single stream of encoded video. This is an iterator that yields packets
// TODO: buffering via seperate thread to read because of blocking i/o
pub struct VideoSource {}

//pub struct VideoDecoder<F> {
//}

//pub struct VideoPacket<T: Encoder>;
