use chrono::{DateTime, Datelike, Weekday};
use chrono::prelude::*;


pub enum Window {
    Upload,
    Vote,
    Winner,
}

pub fn get_window(date: Option<DateTime<Local>>) -> Window {
    let today: Weekday = date
        .map(|d| d.weekday())
        .unwrap_or_else(|| Local::now().weekday());

    match today {
        Weekday::Mon | Weekday::Tue => Window::Upload,
        Weekday::Wed | Weekday::Thu | Weekday::Fri => Window::Vote,
        Weekday::Sat | Weekday::Sun => Window::Winner,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_winner() {
        let dt  = Some(Local.with_ymd_and_hms(2025, 8, 24, 0, 0, 0).unwrap());
        let result = get_window(dt);
        assert!(matches!(result, Window::Winner));
    }

    #[test]
    fn test_window_upload() {
        let dt  = Some(Local.with_ymd_and_hms(2025, 8, 25, 0, 0, 0).unwrap());

        let result = get_window(dt);
        assert!(matches!(result, Window::Upload));
    }
}
