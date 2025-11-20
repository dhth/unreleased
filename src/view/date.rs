use chrono::{DateTime, Utc};

pub(super) fn get_humanized_date(dt: &DateTime<Utc>, reference: &DateTime<Utc>) -> String {
    let duration = reference.signed_duration_since(dt);

    let seconds = duration.num_seconds();

    if seconds < 0 {
        return "in the future".to_string();
    }

    if seconds < 60 {
        return "just now".to_string();
    }

    let minutes = duration.num_minutes();
    if minutes < 60 {
        return format!("{}m ago", minutes);
    }

    let hours = duration.num_hours();
    if hours < 24 {
        return format!("{}h ago", hours);
    }

    format!("{}d ago", duration.num_days())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanizing_recent_date_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-15T11:59:30Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "just now");
    }

    #[test]
    fn humanizing_minutes_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-15T11:45:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "15m ago");
    }

    #[test]
    fn humanizing_hours_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-15T09:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "3h ago");
    }

    #[test]
    fn humanizing_days_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-13T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "2d ago");
    }

    #[test]
    fn humanizing_weeks_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "14d ago");
    }

    #[test]
    fn humanizing_months_works() {
        // GIVEN
        let reference = "2025-03-16T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "60d ago");
    }

    #[test]
    fn humanizing_years_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2023-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "731d ago");
    }

    #[test]
    fn humanizing_future_date_works() {
        // GIVEN
        let reference = "2025-01-15T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let dt = "2025-01-16T12:00:00Z".parse::<DateTime<Utc>>().unwrap();

        // WHEN
        let result = get_humanized_date(&dt, &reference);

        // THEN
        assert_eq!(&result, "in the future");
    }
}
