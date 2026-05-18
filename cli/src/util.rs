use num_format::{Locale, ToFormattedString};

pub fn fmt_currency(value: f64) -> String {
    let s = format!("{:.2}", value);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0]
        .parse::<i64>()
        .unwrap()
        .to_formatted_string(&Locale::en);

    format!("${}.{}", int_part, parts[1])
}
