use std::fmt;

// Duration that is rational.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TimeRational {
    second: i32,
    // fraction
    subsec: (i32, u32),
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

// this would mainly be used for i.e. annotating source files
pub mod mark {
    use crate::TimeRational;

    /// An identifier attached to an event
    #[derive(Debug, Clone)]
    pub struct MarkIdent {
        name: String, // TODO: ref, and Copy
    }

    /// A singular point where something of interest happens; a point of interest.
    #[derive(Debug, Clone)]
    pub struct Mark {
        ident: MarkIdent,
        center: TimeRational,
    }

    /// A section of time where something of interest happens.
    #[derive(Debug, Clone)]
    pub struct MarkSpan {
        ident: MarkIdent,
        // the logical "zero" of the event
        center: Mark,
        before: TimeRational,
        after: TimeRational,
    }
}

pub mod justify {

}

pub mod format {
    use crate::TimeRational;

    /// open a container using ffmpeg url/filename
    pub struct FFDemux {
        url: String,
    }


    /// open a container for output, using ffmpeg url format
    pub struct FFMux {
        url: String,

    }
}

pub mod stream {
    /// A stream of media that is flexibily sized
    pub trait StreamFlexible {}

    /// A stream of media that is fixed-length
    pub trait StreamFixed {}
}

pub mod video_comp {
    use crate::TimeRational;

    pub struct Image;
    pub struct RepeatStill {
        image: Image,
        duration: TimeRational,
    }

    // animations
}
