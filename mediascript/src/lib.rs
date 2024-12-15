use std::fmt;

use num_rational::Ratio;


pub mod av_util;

///// Re-export of [`rusty_ffmpeg`] bindings
//pub use rusty_ffmpeg::ffi as libav;


// monads for opening media files
pub mod container;
//// a time management system with good full justification
//pub mod sequence;


/// A number representing a point in time.
#[derive(Debug, PartialEq, /*Eq,*/ Hash, Clone, Copy)]
pub struct TimeRational(pub Ratio<i64>);
impl fmt::Display for TimeRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let second = self.0.numer() / self.0.denom();

        let secs = second % 60;
        let mins = second / 60 % 60;
        let hours = second / 60 / 60 % 60;

        let frac_num = self.0.numer() % self.0.denom();
        let frac_denom = self.0.denom();

        write!(f, "{hours:02}:{mins:02}:{secs:02}+({frac_num}/{frac_denom})")
    }
}

#[cfg(test)]
mod tests {
    use crate::TimeRational;
    use num_rational::Ratio;

    #[test]
    fn time_rational_formatting() {
        // 1 hour + 23 minutes + 17 seconds = 4997 seconds
        // 4997 * 60 = 299820 frames @ 60 fps
        let time = TimeRational(Ratio::new(299839, 60));
        let formatted = format!("{}", time);
        assert_eq!(formatted, "01:23:17+(19/60)");
    }
}
