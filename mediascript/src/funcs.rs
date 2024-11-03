use crate::TimeRational;

use itertools::Itertools;

/// this function will panic if format is malformed! or if there are overflows
///
/// `01:02:03;(4/60)` = 1 hour, 2 minutes, 3 seconds, and 4 frames @ 60 fps
///
/// ```
/// # use mediascript::funcs::time;
/// # use mediascript::TimeRational;
/// let t0 = time("01:02:03;(4/60)");
/// assert_eq!(t0, TimeRational::new(1, 2, 3, (4, 60)));
/// ```
pub fn time(s: &str) -> TimeRational {
    let (hours, mins, secs_frac) = s.split(':').collect_tuple().unwrap();
    let (secs, frac) = secs_frac.split(';').collect_tuple().unwrap();

    let mut frac_chars = frac.chars();
    frac_chars.next();
    frac_chars.next_back();
    let (subsec_num, subsec_denom) = frac_chars.as_str().split('/').collect_tuple().unwrap();

    let hours: i32 = hours.parse().unwrap();
    let mins: i32 = mins.parse().unwrap();
    let secs: i32 = secs.parse().unwrap();
    let subsec_num: i32 = subsec_num.parse().unwrap();
    let subsec_denom: u32 = subsec_denom.parse().unwrap();

    TimeRational::new(hours, mins, secs, (subsec_num, subsec_denom))
}
// TODO: tests

//fold_seq

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn time_parse() {
//        assert_eq!(time("01"));
//    }
//}
