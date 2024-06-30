use chrono::Datelike;

pub fn solution() -> usize {
    itertools::iproduct!(1901..=2000, 1..=12)
        .filter_map(|(year, month)| chrono::NaiveDate::from_ymd_opt(year, month, 1))
        .filter(|date| date.weekday() == chrono::Weekday::Sun)
        .count()
}
