use std::fmt;

use num_rational::Ratio;


pub mod av_util;

// monads for opening media files
pub mod container;
//// a time management system with good full justification
//pub mod sequence;


/// A number representing a point in time.
#[derive(Debug, PartialEq, /* Eq, */ Hash, Clone, Copy)]
pub struct Timecode(pub Ratio<i64>);
impl fmt::Display for Timecode {
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

pub struct Interval {
}


pub trait Render {

}

pub struct VideoUnit {

}


#[cfg(test)]
mod tests {
    use num_rational::Ratio;

    use crate::Timecode;

    #[test]
    fn time_rational_formatting() {
        // 1 hour + 23 minutes + 17 seconds = 4997 seconds
        // 4997 * 60 = 299820 frames @ 60 fps
        let time = Timecode(Ratio::new(299839, 60));
        let formatted = format!("{}", time);
        assert_eq!(formatted, "01:23:17+(19/60)");
    }
}
