use std::fmt;
use std::num::NonZero;

/// Abstract, tree-based system for media transformations. Everything in this module is monads.
///
/// It makes caching easier.
pub mod tree;

/// Types and functions for actually processing the abstract tree
pub mod runtime;

///// monads for opening media files
//pub mod container;
///// monads for dealing with media streams and rendering
//pub mod media;
///// a time management system with good full justification
//pub mod sequence;
///// layout system for creating graphical systems that fill screen space
//pub mod layout;

/// A point in time, including a fractional subsecond component.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TimeRational {
    pub second: i32,
    /// fraction component
    pub frac: (i32, NonZero<u32>),
}
impl fmt::Display for TimeRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.second % 60;
        let mins = self.second / 60 % 60;
        let hours = self.second / 60 / 60 % 60;

        let (frac_num, frac_denom) = self.frac;
        write!(f, "{hours:02}:{mins:02}:{secs:02}+({frac_num}/{frac_denom})")
    }
}
