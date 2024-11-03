use std::fmt;

pub mod funcs;

/// A point in time, including a fractional subsecond component.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TimeRational {
    second: i32,
    // fraction
    subsec: (i32, u32),
}
impl TimeRational {
    /// Construct from basic time units. The tuple `subsec` is just `(numerator, denominator)`.
    pub fn new(hours: i32, minutes: i32, seconds: i32, subsec: (i32, u32)) -> Self {
        Self {
            second: (hours * 60 * 60) + (minutes * 60) + seconds,
            subsec,
        }
    }
    //pub fn from_str(s: &str) -> Self {
    //    s.split(":");
    //}
}
impl fmt::Display for TimeRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.second % 60;
        let minsec = self.second - (hours * 60);
        let mins = minsec % 60;
        let secs = minsec - (minsec * 60);
        let (frac_num, frac_denom) = self.subsec;
        write!(f, "{hours:02}:{mins:02}:{secs:02}+({frac_num}/{frac_denom})")
    }
}

//// this would mainly be used for i.e. annotating source files
//pub mod mark {
//    use crate::TimeRational;
//
//    use std::collections::{BTreeMap, HashMap};
//
//    #[derive(Debug, Clone, PartialEq, Eq)]
//    pub enum MarkData {
//        String(String),
//        Int(i32),
//        Bool(bool),
//    }
//
//    /// A singular point where something of interest happens; a point of interest.
//    ///
//    /// A mark can hold freeform data that can be used to template things.
//    #[derive(Debug, Clone)]
//    pub struct Mark {
//        center: TimeRational,
//        data: MarkData,
//    }
//    impl Mark {
//        // TODO: data not string
//        pub fn new(center: TimeRational, data: HashMap<String, MarkData>) -> Self {
//            Self { center, data }
//        }
//        pub fn new_empty_data(center: TimeRational) -> Self { Self { center, data: HashMap::new() } }
//        /// Returns the time relative to whatever context/scope the mark is in.
//        pub fn get_rel_time(&self) -> TimeRational { self.center }
//    }
//    ///
//    /// A scope where identifiers are connected together. WIthin a scope, there cannot be multiple
//    /// marks with the same name.
//    ///
//    /// TODO: could be named MarkContext?
//    #[derive(Debug, Clone)]
//    pub struct MarkScope {
//        marks: BTreeMap<String, Mark>,
//    }
//    impl MarkScope {
//        //pub fn from_map(marks: BTreeMap<String, Mark>) -> Self { Self { marks } }
//        pub fn from_static(marks: &[(&'static str, Mark)]) -> Self {
//            Self { marks: BTreeMap::from(marks) }
//        }
//        //pub fn get_mark
//    }
//
//    ///// A section of time where something of interest happens.
//    //#[derive(Debug, Clone)]
//    //pub struct MarkSpan {
//    //    ident: MarkIdent,
//    //    // the logical "zero" of the event
//    //    center: Mark,
//    //    before: TimeRational,
//    //    after: TimeRational,
//    //}
//}
//
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
