#[cfg(test)]
mod tests {
    use secse24_group08::frontend::login::{Login, LoginMessage};

    fn default_login_assertions(login: &Login) {
        assert_eq!(login.get_email(), "");
        assert_eq!(login.get_password(), "");
        assert_eq!(login.get_error(), "");
        assert_eq!(login.get_found_user().id, -1);
        assert_eq!(login.get_found_user().first_name, "");
        assert_eq!(login.get_found_user().last_name, "");
    }

    fn assert_error_message(login: &Login, expected_message: &str) {
        assert_eq!(login.get_error(), expected_message);
    }

    fn setup_login_with_email_and_password(email: &str, password: &str) -> Login {
        let mut login = Login::default();
        login.update(LoginMessage::EmailChanged(email.to_string()));
        login.update(LoginMessage::PasswordChanged(password.to_string()));
        login
    }

    #[test]
    fn test_default_login() {
        let login = Login::default();
        default_login_assertions(&login);
    }

    #[test]
    fn test_email_changed() {
        let mut login = Login::default();
        login.update(LoginMessage::EmailChanged("test@example.com".to_string()));
        assert_eq!(login.get_email(), "test@example.com");
    }

    #[test]
    fn test_password_changed() {
        let mut login = Login::default();
        login.update(LoginMessage::PasswordChanged("password".to_string()));
        assert_eq!(login.get_password(), "password");
    }

    #[test]
    fn test_handle_login_empty_fields() {
        let mut login = Login::default();
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(&login, "Please fill in both email and password fields.");
    }

    #[test]
    fn test_handle_login_invalid_email() {
        let mut login = setup_login_with_email_and_password("test", "password");
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(
            &login,
            "Login failed. Either the email or password was incorrect.",
        );
    }

    #[test]
    fn test_handle_login_invalid_password() {
        let mut login = setup_login_with_email_and_password("test1@email.de", "password");
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(
            &login,
            "Login failed. Either the email or password was incorrect.",
        );
    }

    #[test]
    fn test_handle_login_success() {
        let mut login = setup_login_with_email_and_password("test1@email.de", "n)+L8ZVWw$qKXDQo");
        login.update(LoginMessage::SubmitLogin);
        assert_eq!(login.get_found_user().id, 1);
        assert_eq!(login.get_error(), "");
    }

    #[test]
    fn test_handle_login_sql_injection() {
        // Attempt to inject SQL
        let mut login = setup_login_with_email_and_password(
            "test1@email.de' OR '1'='1", // Injected payload
            "anything' OR '1'='1",       // Injected payload
        );
        login.update(LoginMessage::SubmitLogin);

        assert_eq!(login.get_found_user().id, -1);
        assert_error_message(
            &login,
            "Login failed. Either the email or password was incorrect.",
        );
    }

    #[test]
    fn test_handle_login_sql_injection_2() {
        // Another SQL injection attempt
        let mut login = setup_login_with_email_and_password(
            "test1@email.de'; DROP TABLE users; --", // Injected payload
            "password",
        );
        login.update(LoginMessage::SubmitLogin);

        // Check that login was not successful and no table was dropped
        assert_eq!(login.get_found_user().id, -1);
        assert_error_message(
            &login,
            "Login failed. Either the email or password was incorrect.",
        );
    }

    #[test]
    fn test_handle_login_empty_inputs() {
        let mut login = Login::default();
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(&login, "Please fill in both email and password fields.");

        login.update(LoginMessage::EmailChanged("test".to_string()));
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(&login, "Please fill in both email and password fields.");

        login.update(LoginMessage::EmailChanged("".to_string()));
        login.update(LoginMessage::PasswordChanged("password".to_string()));
        login.update(LoginMessage::SubmitLogin);
        assert_error_message(&login, "Please fill in both email and password fields.");
    }

    #[test]
    fn test_view() {
        let login = Login::default();
        let _ = login.view();
    }
}
