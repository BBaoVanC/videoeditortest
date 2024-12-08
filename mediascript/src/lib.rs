#![expect(unused_imports)]

use std::fmt;
use std::num::NonZero;
use std::collections::HashMap;
use std::fs::{OpenOptions, File};

use ffrust as ffr;

///// Abstract, tree-based system for media transformations. Everything in this module is monads.
/////
///// It makes caching easier.
//pub mod tree;

///// Types and functions for actually processing the abstract tree
//pub mod runtime;

/// monads for opening media files
pub mod container;
/// a time management system with good full justification
pub mod sequence;
///// layout system for creating graphical systems that fill screen space
//pub mod layout;

#[derive(Debug, PartialEq, /*Eq,*/ Hash, Clone, Copy)]
pub struct Fraction {
    /// numerator
    pub num: i64,
    /// denominator
    pub denom: NonZero<u32>,
}
impl std::ops::Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // TODO: reduce/simplify using euclidian algorithm
        Self {
            num: self.num * rhs.num,
            denom: NonZero::new(self.denom.get() * rhs.denom.get()).unwrap(),
        }
    }
}
impl std::ops::Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let rhs_num = NonZero::new(rhs.num).expect("rhs numerator is zero");
        Self {
            num: self.num.checked_mul(rhs.denom.get() as i64).expect("numerator overflowed"),
            denom: NonZero::<i64>::from(self.denom)
                .checked_mul(rhs_num)
                .map(TryInto::try_into)
                .map(Result::ok)
                .flatten()
                .expect("denominator overflowed"),
        }
    }
}
// FIXME: implement PartialEq manually to compare if they are equivalent, even if different
// denominators

/// A point in time, including a fractional subsecond component.
#[derive(Debug, PartialEq, /*Eq,*/ Hash, Clone, Copy)]
pub struct TimeRational(pub Fraction);
impl fmt::Display for TimeRational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let second = self.0.num / (self.0.denom.get() as i64);

        let secs = second % 60;
        let mins = second / 60 % 60;
        let hours = second / 60 / 60 % 60;

        let frac_num = self.0.num % (self.0.denom.get() as i64);
        let frac_denom = self.0.denom;

        write!(f, "{hours:02}:{mins:02}:{secs:02}+({frac_num}/{frac_denom})")
    }
}

#[cfg(test)]
mod test {
    use crate::TimeRational;
    use crate::Fraction;
    use std::num::NonZero;

    #[test]
    fn fraction_div_no_common_factor() {
        let length = Fraction { num: 10, denom: NonZero::new(1).unwrap() }; // 10 seconds
        let velocity = Fraction { num: 1, denom: NonZero::new(8).unwrap() }; // 1/8 speed
        assert_eq!(length / velocity, Fraction { num: 80, denom: NonZero::new(1).unwrap() });
    }

    #[ignore = "common factor equality checks are not yet implemented"]
    #[test]
    fn fraction_div_common_factor() {
        let length = Fraction { num: 100, denom: NonZero::new(60).unwrap() }; // 10.5 seconds
        let velocity = Fraction { num: 1, denom: NonZero::new(4).unwrap() }; // 1/8 speed
        assert_eq!(length / velocity, Fraction { num: 20, denom: NonZero::new(3).unwrap() });
    }

    #[test]
    fn time_rational_formatting() {
        // 1 hour + 23 minutes + 17 seconds = 4997 seconds
        // 4997 * 60 = 299820 frames @ 60 fps
        let time = TimeRational(Fraction { num: 299839, denom: NonZero::new(60).unwrap() });
        let formatted = format!("{}", time);
        assert_eq!(formatted, "01:23:17+(19/60)");
    }
}
