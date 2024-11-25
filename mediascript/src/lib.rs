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
///// monads for dealing with media streams and rendering
//pub mod media;
/// a time management system with good full justification
pub mod sequence;
///// layout system for creating graphical systems that fill screen space
//pub mod layout;

pub struct Runtime {
    //pub filesystem: Filesystem,
}
impl Runtime {
    ///// Create runtime using a specific file backend
    // pub fn new
}

//pub struct Filesystem {
//    /// opened for read only
//    open_inputs: HashMap<String, File>,
//    /// opened for writing
//    open_outputs: HashMap<String, File>,
//}
//
//pub struct MediaFile {
//
//}
//impl Filesystem {
//    /// Get a handle to a file on disk, opened for **reads only**.
//    pub fn get_input_file(&mut self, path: &str) -> Result<MediaFile, std::io::Error> {
//        //let file = File::open(&path)?;
//        todo!()
//    }
//    ///// Get a handle to a file on disk, opened for writing.
//    //pub fn get_output_file(&mut self, path
//}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Fraction(pub i32, pub NonZero<u32>);

/// A point in time, including a fractional subsecond component.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TimeRational {
    pub second: i32,
    /// fraction component
    pub frac: Fraction,
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
impl std::ops::Mul for TimeRational {
    type Output = Self;
    /// # Proof
    ///
    /// First, multiply into improper fraction.
    ///
    /// ```text
    /// (a + (j/k))(b + (m/n))
    /// = ab + (am)/n + (bj)/k + (jm)/(kn)
    /// = (abkn)/(kn) + (amk)/(kn) + (bjn)/(kn) + (jm)/(kn)
    /// = (abkn + amk + bjn + jm)/(kn)
    /// ```
    ///
    /// # Panics
    ///
    /// TODO
    // TODO: tests
    fn mul(self, rhs: Rhs) -> Self {
        // FIXME: doesn't compile because of number types


        // see doc comment for var names
        let a = self.second as i64;
        let j = self.frac.0;
        let k = self.frac.1.get();

        let b = rhs.second as i64;
        let m = rhs.frac.0;
        let n = rhs.frac.1.get();

        let improper_num = (a * b * k * n) + (a * m * k) + (b * j * n) + (j * m);
        let improper_denom = k * n;

        // TODO: could replace this with num_traits to get both div and mod in one function
        TimeRational {
            second: (improper_num / improper_denom),
            frac: Fraction(
                (improper_num % improper_denom).try_into().unwrap(),
                NonZero::new(improper_denom).unwrap()
            ),
        }
    }
}
