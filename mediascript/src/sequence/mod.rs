use crate::TimeRational;
use crate::Fraction;

/// A fixed-length segment of multiplexed media
pub trait ClipFixedLength {
    fn length(&self) -> TimeRational;
}

pub struct ClipConstantVelocity<T: ClipFixedLength> {
    clip: T,
    velocity: Fraction,
}
impl<T: ClipFixedLength> ClipFixedLength for ClipConstantVelocity<T> {
    fn length(&self) -> TimeRational {
        TimeRational(self.clip.length().0 * self.velocity)
    }
}
