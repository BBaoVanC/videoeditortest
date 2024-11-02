use std::fmt;

// Duration that is rational.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// TODO: impl Display
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
        write!(f, "{hours:02}:{mins:02}:{secs:02} + ({frac_num}/{frac_denom})")
    }
}

// this would mainly be used for i.e. annotating source files
pub mod mark {
    use crate::TimeRational;

    /// An identifier attached to an event
    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub struct MarkIdent {
        name: String, // TODO: ref, and Copy
    }

    /// A singular point where something of interest happens; a point of interest.
    pub struct Mark {
        ident: MarkIdent,
        center: TimeRational,
    }

    /// A section of time where something of interest happens.
    pub struct MarkSpan {
        ident: MarkIdent,
        // the logical "zero" of the event
        center: Mark,
        before: TimeRational,
        after: TimeRational,
    }
}

pub mod containers {
    use crate::TimeRational;

    /// fixed-length stream of media
    pub trait MediaStreamFixed {
        type Output;
        fn length() -> TimeRational;
    }
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
