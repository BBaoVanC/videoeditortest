//! Monads for muxing and demuxing media containers

use std::{ffi::CStr, ptr};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::error::Error;
use std::sync::AtomicBool;

use rusty_ffmpeg::ffi as av;

use crate::av_util::{AVError, MapIntToResultAVError};


/// A provider of media packets. It acts as an input, providing frames to enter the
/// pipeline.
///
/// The streams are not split here, you will need to use ... to get at a
/// specific stream.
pub trait MediaStream {
    type Error: Error;
    fn read_packet(&self) -> Result<av::AVPacket, Self::Error>;
    fn metadata(&self) -> ContainerMetadata;
}

/// Information about a container and its contents.
pub struct ContainerMetadata {
    pub format: av::AVInputFormat,
    pub video_stream_count: u32,
    pub audio_stream_count: u32,
    pub subtitle_stream_count: u32,
}
pub enum StreamType {
    Video,
    Audio,
    Subtitle,
}


/// Handle to an input media file. Basically like an [`Iterator`] but yields
/// frames of media instead.
// FIXME: buffering i/o on other thread
// FIXME: possibly async api instead
#[derive(Debug)]
pub struct InputFile {
    /// notify that the queue has had packets consumed
    queue_notify: mpsc::Sender<i32>,
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
        let (queue_notify, queue_rx) = mpsc::channel();

        // TODO: handle error instead of panic using Builder::spawn
        thread::spawn(move || {
            loop {
                match queue_rx.recv() {
                    Err(mpsc::RecvError) => {
                        av::avformat_close_input(&raw mut ctx);
                        debug_assert!(ctx.is_null());
                        break
                    },
                    Ok(count) => {
                        for i in count {

                            av::av_read_frame(ctx,
                        }
                    },
                }
            }
        });

        todo!()

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
impl MediaStream for InputFile {
    type Error = std::io::Error;
    fn read_packet(&self) -> Result<av::AVPacket, Self::Error> {
        todo!()
    }
}

/// Sorts the packets from a demuxer into separate media streams.
pub struct SortedMediaStream<T: MediaStream> {
    container: T,

    /// buffer for packets that haven't been read yet because another stream is
    /// being read instead
    buf

    pub video_streams: Vec<EncodedVideoStream>,
    //audio_streams: Vec<AudioSource>,
    //subtitle_streams: Vec<SubtitleSource>,
}

/// A single stream of encoded video. This is an iterator that yields packets
// nice to be able to read at the same time as decoding, so another thread
pub struct EncodedVideoStream {
}
impl EncodedVideoStream {
}

/// Objects that take media and output it to a file or somewhere.
pub trait MediaSink {
    // type Input
    // fn write
}

//pub struct VideoDecoder<F> {
//}

//pub struct VideoPacket<T: Encoder>;



