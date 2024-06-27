#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDateTime};
    use secse24_group08::backend::database::db::*;

    #[test]
    fn test_establish_connection_local() {
        let result = establish_connection();

        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_emails() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag+sorting@example.com"));
        assert!(is_valid_email("x@example.com")); // One-letter local-part
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!is_valid_email("plainaddress"));
        assert!(!is_valid_email("@missing-local-part.com"));
        assert!(!is_valid_email("missing-at-sign.com"));
        assert!(!is_valid_email("missing.domain@.com"));
    }
    #[test]
    fn test_is_strong_password() {
        // Strong passwords
        assert!(is_strong_password("Aa1!Aa1!"));
        assert!(is_strong_password("P@ssw0rd123"));
    }

    #[test]
    fn test_is_weak_password() {
        // Weak passwords
        assert!(!is_strong_password("password")); // No uppercase, no digit, no special character
        assert!(!is_strong_password("Password")); // No digit, no special character
        assert!(!is_strong_password("Passw0rd")); // No special character
        assert!(!is_strong_password("Pass!ord")); // No digit
        assert!(!is_strong_password("Pa1!")); // Too short
    }

    #[test]
    fn test_format_send_date() {
        let now = Local::now().naive_local();

        // Test same day
        let same_day = now;
        assert_eq!(
            format_send_date(same_day),
            same_day.format("%I:%M %p").to_string()
        );

        // Test same year but different day
        let same_year =
            NaiveDateTime::parse_from_str("2024-05-24 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(
            format_send_date(same_year),
            same_year.format("%A %b %e, %I:%M %p").to_string()
        );

        // Test different year
        let different_year =
            NaiveDateTime::parse_from_str("2023-05-24 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(
            format_send_date(different_year),
            different_year.format("%A %b %e, %Y %I:%M %p").to_string()
        );
    }
}
