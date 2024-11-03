use crate::TimeRational;

/// this function will panic if format is malformed! or if there are overflows
///
/// 01:02:03;(4/60) = 1 hour, 2 minutes, 3 seconds, and 4 frames @ 60 fps
pub fn time(s: &str) -> TimeRational {
    // could be written better with itertools collect_tuple
    let s_split = s.split(':');
    let hours = s_split[0];
    let mins = s_split[1];
    let secs_frac = s_split[2];

    let secs_frac_split = secs_frac.split(';').collect();
    let secs = secs_frac_split[0];
    let frac = secs_frac_split[1];
    assert!(frac[0] == '(');
    assert!(frac.last() == ')');
    let frac = frac[1..frac.len()];
    let frac_split = frac.split();
    let subsec_num = frac_split[0];
    let subsec_denom = frac_split[1];

    let hours: i32 = hours.parse().unwrap();
    let mins: i32 = mins.parse().unwrap();
    let secs: i32 = secs.parse().unwrap();
    let subsec_num: i32 = subsec_num.parse().unwrap();
    let subsec_denom: i32 = subsec_denom.parse().unwrap();

    TimeRational::new(hours, mins, secs, (subsec_num, subsec_denom))
}
// TODO: tests

//fold_seq
