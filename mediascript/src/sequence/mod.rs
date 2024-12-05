use crate::TimeRational;
use crate::Fraction;

/// A fixed-length clip of multiplexed media that can be worked with in the justification system.
pub trait Segment {
    fn length(&self) -> TimeRational;
}


//pub struct ClipConstantVelocity<T: Segment> {
//    clip: T,
//    velocity: Fraction,
//}
//impl<T: Segment> Segment for ClipConstantVelocity<T> {
//    fn length(&self) -> TimeRational {
//        TimeRational(self.clip.length().0 / self.velocity)
//    }
//}
//
//pub struct ClipLinearVelocity<T: Segment> {
//    clip: T,
//    /// slope of velocity line
//    dvelocity,
//    /// the initial velocity at the clip's origin
//    velocity0,
//}
//impl<T: Segment> Segment for ClipLinearVelocity<T> {
//    fn length(&self) -> TimeRational {
//
//    }
//}
