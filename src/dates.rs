use time::{Date, Duration};

/// Calculates the date of `duration` days from the given date.
/// - Use `ext::NumericalDuration` to create duration such `1.days()`
/// - Use `Date::from_calendar_date` to create a date.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// # use dryip::dates::add_days;
/// # use time::{ext::NumericalDuration, Date, Month, Duration};
/// assert_eq!(Ok(Date::from_calendar_date(2019, Month::January, 2).unwrap()), add_days(1.days(),Date::from_calendar_date(2019, Month::January, 1).unwrap()));
/// ```
pub fn add_days(duration: Duration, date: Date) -> Result<Date, time::error::ComponentRange> {
    Ok(date + duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{ext::NumericalDuration, Date, Month};

    #[test]
    fn test_add_days() {
        let current_date = Date::from_calendar_date(2019, Month::January, 31).unwrap();

        assert_eq!(
            Ok(Date::from_calendar_date(2019, Month::February, 1).unwrap()),
            add_days(1.days(), current_date)
        );
        assert_eq!(
            Ok(Date::from_calendar_date(2019, Month::January, 29).unwrap()),
            add_days(-2.days(), current_date)
        );
    }
}
