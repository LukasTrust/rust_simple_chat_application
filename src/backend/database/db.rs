use chrono::{Datelike, Local, NaiveDateTime};
use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::sync::Mutex;

use diesel::pg::PgConnection;
use diesel::Connection;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONNECTION_MUTEX: Mutex<()> = Mutex::new(());
}

/// Establish a connection to the database. Returns the connection or an error message
pub fn establish_connection() -> Result<PgConnection, diesel::ConnectionError> {
    let _guard = CONNECTION_MUTEX
        .lock()
        .expect("Failed to acquire mutex lock");
    dotenv().expect("Failed to read .env file");

    if let Ok(database_url) = env::var("DATABASE_URL") {
        if let Ok(connection) = PgConnection::establish(&database_url) {
            return Ok(connection);
        }
    }

    Err(diesel::ConnectionError::BadConnection(
        "Failed to establish connection to databases".to_string(),
    ))
}

/// Check if an email is valid.
/// A valid email must:
/// - Contain only alphanumeric characters, dots, hyphens, and underscores
/// - Have a domain with at least one dot
/// - Have a top-level domain with at least two characters
pub fn is_valid_email(email: &str) -> bool {
    // Regular expression to match a basic email format
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

/// Check if a password is strong.
/// A strong password must:
/// - Be at least 8 characters long
/// - Contain at least one lowercase letter
/// - Contain at least one uppercase letter
/// - Contain at least one digit
/// - Contain at least one special character
pub fn is_strong_password(password: &str) -> bool {
    // Define password strength criteria
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    // Password is considered strong if it meets all criteria
    password.len() >= 8 && has_lowercase && has_uppercase && has_digit && has_special
}

/// Format the send date of a message.
/// The date is formatted conditionally based on the current date and time.
/// - If the message was sent today, only the time is shown
/// - If the message was sent this year, the month and day are shown without the year
/// - If the message was sent in a different year, the full date and time are shown
pub fn format_send_date(date: NaiveDateTime) -> String {
    // Get the current date and time in the local timezone
    let now = Local::now().naive_local();

    // Format the date conditionally
    let formatted_date = if date.date() == now.date() {
        // Same day: show only time
        date.format("%I:%M %p").to_string()
    } else if date.year() == now.year() {
        // Same year but different day: show month and day, but omit year
        date.format("%A %b %e, %I:%M %p").to_string()
    } else {
        // Different year: show full date and time
        date.format("%A %b %e, %Y %I:%M %p").to_string()
    };

    formatted_date
}
