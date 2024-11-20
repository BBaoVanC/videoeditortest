#![expect(dead_code)]

use crate::runtime::FileHandle;

/// Open a file
pub struct OpenMediaFile<'p> {
    pub(crate) path: &'p str,
}

pub fn open_file(path: &str) -> OpenMediaFile {
    OpenMediaFile { path }
}

/// Demux a file to get at the container inside
pub struct Demux {
    pub(crate) file: OpenMediaFile
}

pub struct GetMediaStreams<'r, 'f> {
    pub(crate) container: &'r Demux<'f>,
}
