// Duration that is rational.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)
// TODO: impl Display
pub struct TimeRational {
    second: i32,
    // fraction
    subsec: (i32, u32),
}

// this would mainly be used for i.e. annotating source files
mod mark {
    pub struct MarkScope; // TODO:

    /// An identifier attached to an event
    pub struct MarkIdent {
        scope: MarkScope,
        name: String, // TODO: ref
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

mod containers {
    /// fixed-length stream of media
    pub trait MediaStreamFixed {
        type Output;
        fn length() -> TimeRational;
    }
}

mod video_comp {
    pub struct Image;
    pub struct RepeatStill {
        image: Image,
        duration: TimeRational,
    }

    // animations
}
