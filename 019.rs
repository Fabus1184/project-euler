// cargo-deps: chrono="0.4.24"

use chrono::Datelike;

let mut count = 0;
let mut date = chrono::NaiveDate::from_ymd_opt(1901, 1, 1).unwrap();

while date.year() < 2001 {
    if date.day() == 1 && date.weekday() == chrono::Weekday::Sun {
        count += 1;
    }
    date = date + chrono::Duration::days(1);
}

println!("{}", count);

