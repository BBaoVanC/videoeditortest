use crate::TimeRational;

use regex::Regex;
use snafu::Snafu;
use snafu::OptionExt;
use snafu::ResultExt;

use std::num::ParseIntError;
use std::sync::LazyLock;

/// Errors from parsing a string to create [`TimeRational`].
// TODO: redo this struct
#[derive(Debug, Snafu)]
pub enum CreateTimeError {
    /// the format is invalid and the regex did not match
    TimeFormat,

    /// hours could not be parsed
    ParseHours { source: ParseIntError },
    /// minutes could not be parsed
    ParseMinutes { source: ParseIntError },
    /// seconds could not be parsed
    ParseSeconds { source: ParseIntError },
    /// subsecond component could not be parsed
    //ParseSubsec { source: ParseSubsecError },
    // TODO: this is now missing context
    ParseSubsec { source: ParseIntError },
}
//#[derive(Debug, Snafu)]
//// TODO: split this into frac and dec versions
//pub enum ParseSubsecError {
//    ParseNumeratorError { source: ParseIntError },
//    ParseDenominatorError { source: ParseIntError },
//    ParseDecimalError { source: ParseIntError },
//}

/// this function will panic if format is malformed! or if there are overflows
///
/// `01:02:03;(4/60)` = 1 hour, 2 minutes, 3 seconds, and 4 frames @ 60 fps
///
/// ```
/// # use mediascript::funcs::time;
/// # use mediascript::TimeRational;
/// let t0 = time("01:02:03;(4/60)").unwrap();
/// assert_eq!(t0, TimeRational::new(1, 2, 3, (4, 60)));
/// ```
// ///
// /// Can omit stuff
// ///
// /// ```
// /// # use mediascript::funcs::time;
// /// # use mediascript::TimeRational;
// /// let t0 = time("::2;(12/30)").unwrap();
// /// assert_eq!(t0, TimeRational::new(0, 0, 2, (12, 30)));
pub fn time(s: &str) -> Result<TimeRational, CreateTimeError> {
    static TIME_REGEX: LazyLock<Regex> = LazyLock::new(
        || Regex::new(r"^([0-9]+):([0-9]+):([0-9]+);\(([0-9]+)\/([0-9]+)\)$").unwrap()
    );

    let caps = TIME_REGEX.captures(s).context(TimeFormatSnafu)?;

    let hours = caps[1].parse().context(ParseHoursSnafu)?;
    let mins = caps[2].parse().context(ParseMinutesSnafu)?;
    let secs = caps[3].parse().context(ParseSecondsSnafu)?;
    let subsec_num = caps[4].parse().context(ParseSubsecSnafu)?;
    let subsec_denom = caps[5].parse().context(ParseSubsecSnafu)?;

    Ok(TimeRational::new(hours, mins, secs, (subsec_num, subsec_denom)))
}
// TODO: tests

/// ```
/// # use mediascript::funcs::time_dec;
/// # use mediascript::TimeRational;
/// let t0 = time_dec("01:02:03.343").unwrap();
/// assert_eq!(t0, TimeRational::new(1, 2, 3, (343, 1000)));
/// ```
///
/// ```
/// # use mediascript::funcs::time_dec;
/// # use mediascript::TimeRational;
/// let t0 = time_dec("01:02:03").unwrap();
/// assert_eq!(t0, TimeRational::new(1, 2, 3, (0, 1)));
/// ```
pub fn time_dec(s: &str) -> Result<TimeRational, CreateTimeError> {
    static TIME_DEC_REGEX: LazyLock<Regex> = LazyLock::new(
        || Regex::new(r"^([0-9]+):([0-9]+):([0-9]+)(\.[0-9]+)?$").unwrap()
    );

    let caps = TIME_DEC_REGEX.captures(s).context(TimeFormatSnafu)?;

    let hours = caps[1].parse().context(ParseHoursSnafu)?;
    let mins = caps[2].parse().context(ParseMinutesSnafu)?;
    let secs = caps[3].parse().context(ParseSecondsSnafu)?;

    let (subsec_num, subsec_denom) = caps
        .get(4)
        .map(|ss| {
            let denom_mag: u32 = ss.len().try_into().unwrap();
            let denom_mag = denom_mag - 1; // sub 1 because of leading .
            Ok((
                ss.as_str()[1..].parse()?,
                10_u32.pow(denom_mag), // sub 1 because of the leading .
            ))
        }).transpose().context(ParseSubsecSnafu)?.unwrap_or((0, 1)); // TODO: should this count chars instead?

    Ok(TimeRational::new(hours, mins, secs, (subsec_num, subsec_denom)))
}

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
