#![expect(unused_imports)]

use std::fmt;
use std::num::NonZero;
use std::collections::HashMap;
use std::fs::{OpenOptions, File};

use ffrust as ffr;


/// monads for opening media files
pub mod container;
///// a time management system with good full justification
//pub mod sequence;


/// A number representing a point in time.
#[derive(Debug, PartialEq, /*Eq,*/ Hash, Clone, Copy)]
pub struct TimeRational(pub Ratio);
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
