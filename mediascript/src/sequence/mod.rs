use crate::TimeRational;
use crate::Fraction;

/// A fixed-length segment of multiplexed media
pub trait ClipFixedLength {
    fn length(&self) -> Fraction;
}

pub struct ClipConstantVelocity<T: ClipFixedLength> {
    clip: T,
    velocity: Fraction,
}
impl<T: ClipFixedLength> ClipFixedLength for ClipConstantVelocity<T> {
    fn length(&self) { self.clip.length() * self.velocity }
}
