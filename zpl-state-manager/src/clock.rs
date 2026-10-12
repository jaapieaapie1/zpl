//! Real-time clock fields (^FC, ^ST, ^SO).

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use std::time::{SystemTime, UNIX_EPOCH};

/// A calendar date and time (proleptic Gregorian, no time zone).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub year: i64,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

/// Days since 1970-01-01 for a civil date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { yoe + era * 400 + 1 } else { yoe + era * 400 }, m, d)
}

impl DateTime {
    /// The system clock. `wasm32-unknown-unknown` has none (`SystemTime::now` panics
    /// there), so it reads as the epoch; callers in the browser set the clock instead.
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn now() -> Self {
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
        Self::from_unix(secs)
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn now() -> Self {
        Self::from_unix(0)
    }

    pub(crate) fn from_unix(secs: i64) -> Self {
        let (days, rem) = (secs.div_euclid(86400), secs.rem_euclid(86400));
        let (year, month, day) = civil_from_days(days);
        Self { year, month, day, hour: (rem / 3600) as u32, minute: (rem / 60 % 60) as u32, second: (rem % 60) as u32 }
    }

    fn unix(&self) -> i64 {
        days_from_civil(self.year, self.month, self.day) * 86400
            + self.hour as i64 * 3600
            + self.minute as i64 * 60
            + self.second as i64
    }

    /// Applies an ^SO offset. Months and years move the calendar date (clamping the day to the
    /// new month's length); the rest is added as elapsed time.
    pub fn offset(&self, o: &Offset) -> Self {
        let months = self.year * 12 + self.month as i64 - 1 + o.months as i64 + o.years as i64 * 12;
        let (year, month) = (months.div_euclid(12), months.rem_euclid(12) as u32 + 1);
        let next = if month == 12 { days_from_civil(year + 1, 1, 1) } else { days_from_civil(year, month + 1, 1) };
        let month_len = (next - days_from_civil(year, month, 1)) as u32;
        let shifted = Self { year, month, day: self.day.min(month_len), ..*self };
        let delta = o.days as i64 * 86400 + o.hours as i64 * 3600 + o.minutes as i64 * 60 + o.seconds as i64;
        Self::from_unix(shifted.unix() + delta)
    }

    fn weekday(&self) -> u32 {
        // 1970-01-01 was a Thursday; 0 = Sunday.
        (days_from_civil(self.year, self.month, self.day) + 4).rem_euclid(7) as u32
    }

    fn day_of_year(&self) -> u32 {
        (days_from_civil(self.year, self.month, self.day) - days_from_civil(self.year, 1, 1) + 1) as u32
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Offset {
    pub months: i32,
    pub days: i32,
    pub years: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}

const WEEKDAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November",
    "December",
];

/// Expands the ^FC format codes in `data`. `clocks` pairs each indicator character with the
/// time it reads (primary, secondary, tertiary).
pub fn expand(data: &str, clocks: &[(char, DateTime)]) -> String {
    let mut out = String::with_capacity(data.len() + 16);
    let mut chars = data.chars().peekable();
    while let Some(c) = chars.next() {
        let Some(&(_, t)) = clocks.iter().find(|(ind, _)| *ind == c) else {
            out.push(c);
            continue;
        };
        let Some(&code) = chars.peek() else {
            out.push(c);
            break;
        };
        let wd = t.weekday();
        let h12 = if t.hour % 12 == 0 { 12 } else { t.hour % 12 };
        let doy = t.day_of_year();
        let s = match code {
            'a' => WEEKDAYS[wd as usize][..3].to_string(),
            'A' => WEEKDAYS[wd as usize].to_string(),
            'b' => MONTHS[t.month as usize - 1][..3].to_string(),
            'B' => MONTHS[t.month as usize - 1].to_string(),
            'd' => format!("{:02}", t.day),
            'H' => format!("{:02}", t.hour),
            'I' => format!("{:02}", h12),
            'j' => format!("{doy:03}"),
            'm' => format!("{:02}", t.month),
            'M' => format!("{:02}", t.minute),
            'p' => (if t.hour < 12 { "AM" } else { "PM" }).to_string(),
            'S' => format!("{:02}", t.second),
            'y' => format!("{:02}", t.year.rem_euclid(100)),
            'Y' => format!("{:04}", t.year),
            'w' => wd.to_string(),
            // Week numbers: weeks start on Sunday (%U) or Monday (%W); days before the first
            // such day are week 0.
            'U' => format!("{:02}", (doy + 6 - wd) / 7),
            'W' => format!("{:02}", (doy + 6 - (wd + 6) % 7) / 7),
            _ => {
                out.push(c);
                continue;
            }
        };
        chars.next();
        out.push_str(&s);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: DateTime = DateTime { year: 2026, month: 10, day: 13, hour: 14, minute: 5, second: 9 };

    #[test]
    fn formats() {
        assert_eq!(expand("%Y-%m-%d %H:%M:%S", &[('%', T)]), "2026-10-13 14:05:09");
        assert_eq!(expand("%a %A %b %B %I%p %y %j %w", &[('%', T)]), "Tue Tuesday Oct October 02PM 26 286 2");
        assert_eq!(expand("100%", &[('%', T)]), "100%");
        assert_eq!(expand("%Q", &[('%', T)]), "%Q");
    }

    #[test]
    fn offsets() {
        let t = DateTime { year: 2026, month: 1, day: 31, hour: 23, minute: 0, second: 0 };
        let o = Offset { months: 1, hours: 2, ..Default::default() };
        assert_eq!(t.offset(&o), DateTime { year: 2026, month: 3, day: 1, hour: 1, minute: 0, second: 0 });
        assert_eq!(T.offset(&Offset { years: -1, ..Default::default() }).year, 2025);
    }

    #[test]
    fn round_trips_unix() {
        assert_eq!(DateTime::from_unix(0), DateTime { year: 1970, month: 1, day: 1, hour: 0, minute: 0, second: 0 });
        assert_eq!(DateTime::from_unix(T.unix()), T);
    }
}
