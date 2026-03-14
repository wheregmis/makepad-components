#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShadDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl ShadDate {
    pub fn parse_iso(value: &str) -> Option<Self> {
        let mut parts = value.split('-');
        let year = parts.next()?.parse::<i32>().ok()?;
        let month = parts.next()?.parse::<u8>().ok()?;
        let day = parts.next()?.parse::<u8>().ok()?;
        if parts.next().is_some() {
            return None;
        }
        Self::new(year, month, day)
    }

    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        if !(1..=12).contains(&month) {
            return None;
        }
        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return None;
        }
        Some(Self { year, month, day })
    }

    pub fn format_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn try_today_utc() -> Option<Self> {
        #[cfg(target_arch = "wasm32")]
        {
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let days_since_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| (duration.as_secs() / 86_400) as i64)
                .ok()?;
            let (year, month, day) = civil_from_days(days_since_epoch);
            Some(Self { year, month, day })
        }
    }

    pub fn today_utc() -> Self {
        Self::try_today_utc().unwrap_or_else(Self::fallback_visible_date)
    }

    pub fn fallback_visible_date() -> Self {
        Self {
            year: 2000,
            month: 1,
            day: 1,
        }
    }

    pub fn fallback_visible_month() -> (i32, u8) {
        let date = Self::fallback_visible_date();
        (date.year, date.month)
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

pub fn shift_month(year: i32, month: u8, delta: i32) -> (i32, u8) {
    let zero_based = month as i32 - 1 + delta;
    let year_offset = zero_based.div_euclid(12);
    let month = zero_based.rem_euclid(12) + 1;
    (year + year_offset, month as u8)
}

pub fn weekday_from_civil(year: i32, month: u8, day: u8) -> u8 {
    let mut y = year;
    let mut m = month as i32;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (day as i32 + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
    ((h + 6) % 7) as u8
}

#[cfg(not(target_arch = "wasm32"))]
pub fn civil_from_days(days: i64) -> (i32, u8, u8) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = (y + if m <= 2 { 1 } else { 0 }) as i32;
    (year, m as u8, d as u8)
}

#[cfg(test)]
mod tests {
    use super::{days_in_month, shift_month, weekday_from_civil, ShadDate};

    #[test]
    fn parses_and_formats_iso_dates() {
        let date = ShadDate::parse_iso("2026-03-13").unwrap();
        assert_eq!(date.format_iso(), "2026-03-13");
    }

    #[test]
    fn rejects_invalid_dates() {
        assert!(ShadDate::parse_iso("2026-02-30").is_none());
        assert!(ShadDate::parse_iso("bad-value").is_none());
    }

    #[test]
    fn shifts_months_across_year_boundaries() {
        assert_eq!(shift_month(2026, 1, -1), (2025, 12));
        assert_eq!(shift_month(2026, 12, 1), (2027, 1));
    }

    #[test]
    fn handles_leap_year_february() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
    }

    #[test]
    fn computes_weekday_with_sunday_zero() {
        assert_eq!(weekday_from_civil(2026, 3, 13), 5);
    }

    #[test]
    fn fallback_visible_date_is_stable() {
        assert_eq!(ShadDate::fallback_visible_date().format_iso(), "2000-01-01");
    }
}
