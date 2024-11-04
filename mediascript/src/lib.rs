use std::fmt;
use std::collections::{BTreeMap, HashMap};

pub mod funcs;

/// A point in time, including a fractional subsecond component.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TimeRational {
    pub second: i32,
    /// fraction component
    pub subsec: (i32, u32),
}
impl fmt::Display for TimeRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.second % 60;
        let mins = self.second / 60 % 60;
        let hours = self.second / 60 / 60 % 60;

        let (frac_num, frac_denom) = self.subsec;
        write!(f, "{hours:02}:{mins:02}:{secs:02}+({frac_num}/{frac_denom})")
    }
}


/// Metadata that can be associated with a marker. See doc on [`Mark`] for why this is useful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkData {
    String(String),
    Int(i32),
    Bool(bool),
}

/// An object marking a point in time where something happens; represents a point of interest that
/// you might want to edit with respect to.
///
/// Marks don't make sense on their own. They have to be used in context of some time sequence,
/// such as a [`MediaStream`]. Therefore, the [`center`] field is a relative timecode in respect to
/// the timeline that holds the mark.
///
/// A mark can hold freeform data, TODO: example
#[derive(Debug, Clone)]
pub struct Mark {
    /// time relative to the semantic structure that holds this mark
    pub center: TimeRational,
    /// metadata
    pub data: MarkData,
}

///// A section of time where something of interest happens.
//#[derive(Debug, Clone)]
//pub struct MarkSpan {
//    ident: MarkIdent,
//    // the logical "zero" of the event
//    center: Mark,
//    before: TimeRational,
//    after: TimeRational,
//}


/// Represents a clip with a fixed length of time. It can render through the included callback
#[derive(Debug)]
pub struct StreamFixed


///// could be called stream or justify, it's for justifying[1] media clips in to a single stream
/////
///// [1]: justification as in similar to what it means in the latex world
//pub mod seq {
//    use crate::mark;
//}
//
///// this is NOT muxing into video containers/files. This is combining multiple media streams into
///// one unit
//pub mod multiplex {
//
//}
//
//pub mod format {
//    use crate::TimeRational;
//
//    /// open a container using ffmpeg url/filename
//    pub struct FFDemux {
//        url: String,
//    }
//
//
//    /// open a container for output, using ffmpeg url format
//    pub struct FFMux {
//        url: String,
//
//    }
//}
//
//pub mod stream {
//    /// A stream of media that is flexibily sized
//    pub trait StreamFlexible {}
//
//    /// A stream of media that is fixed-length
//    pub trait StreamFixed {}
//}
//
//pub mod video_comp {
//    use crate::TimeRational;
//
//    pub struct Image;
//    pub struct RepeatStill {
//        image: Image,
//        duration: TimeRational,
//    }
//
//    // animations
//}
