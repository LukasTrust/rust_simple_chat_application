#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::entities::user_ops::{delete_user, find_user_by_email},
        frontend::register::{Register, RegisterMessage},
    };

    fn setup_register() -> Register {
        Register::default()
    }

    fn submit_register(email: &str, password: &str, first_name: &str, last_name: &str) -> Register {
        let mut register = setup_register();
        register.update(RegisterMessage::EmailChanged(email.to_string()));
        register.update(RegisterMessage::PasswordChanged(password.to_string()));
        register.update(RegisterMessage::FirstNameChanged(first_name.to_string()));
        register.update(RegisterMessage::LastNameChanged(last_name.to_string()));
        register.update(RegisterMessage::SubmitRegister);
        register
    }

    #[test]
    fn test_initial_state() {
        let register = setup_register();
        assert_eq!(register.get_email(), "");
        assert_eq!(register.get_password(), "");
        assert_eq!(register.get_first_name(), "");
        assert_eq!(register.get_last_name(), "");
        assert_eq!(register.get_error(), "");
        assert_eq!(register.get_info(), "");
    }

    #[test]
    fn test_email_change() {
        let email = "test@example.com";
        let mut register = setup_register();
        register.update(RegisterMessage::EmailChanged(email.to_string()));
        assert_eq!(register.get_email(), email);
    }

    #[test]
    fn test_password_change() {
        let password = "StrongP@ssw0rd";
        let mut register = setup_register();
        register.update(RegisterMessage::PasswordChanged(password.to_string()));
        assert_eq!(register.get_password(), password);
    }

    #[test]
    fn test_first_name_change() {
        let name = "John";
        let mut register = setup_register();
        register.update(RegisterMessage::FirstNameChanged(name.to_string()));
        assert_eq!(register.get_first_name(), name);
    }

    #[test]
    fn test_last_name_change() {
        let name = "Doe";
        let mut register = setup_register();
        register.update(RegisterMessage::LastNameChanged(name.to_string()));
        assert_eq!(register.get_last_name(), name);
    }

    #[test]
    fn test_submit_register_empty_fields() {
        let mut register = setup_register();
        register.update(RegisterMessage::SubmitRegister);
        assert_eq!(register.get_error(), "Please fill in all fields");
    }

    #[test]
    fn test_submit_register_invalid_email() {
        let register = submit_register("invalid_email", "StrongP@ssw0rd", "John", "Doe");
        assert_eq!(register.get_error(), "Invalid email format");
    }

    #[test]
    fn test_submit_register_weak_password() {
        let register = submit_register("register_test@example.com", "weak", "John", "Doe");
        assert_eq!(register.get_error(), "Password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one digit, and one special character");
    }

    #[test]
    fn test_submit_register_success() {
        let email = "register1_test@example.com";
        let register = submit_register(&email, "StrongP@ssw0rd", "John", "Doe");

        assert_eq!(register.get_error(), "");
        assert_eq!(register.get_info(), "Account has been registered");

        // Clean up
        let user = find_user_by_email(&email);
        assert!(user.is_ok());
        let user = user.unwrap();
        let result = delete_user(user.id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_register_failure() {
        let email = "register2_test@example.com";
        let password = "StrongP@ssw0rd";

        let register = submit_register(email, password, "John", "Doe");
        assert_eq!(register.get_error(), "");
        assert_eq!(register.get_info(), "Account has been registered");

        let register = submit_register(email, password, "John", "Doe");
        assert_eq!(register.get_error(), "Email address already in use");
        assert_eq!(register.get_info(), "");

        // Clean up
        let user = find_user_by_email(&email);
        assert!(user.is_ok());
        let user = user.unwrap();
        let result = delete_user(user.id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_view() {
        let mut register = setup_register();
        register.set_error("Error message".to_string());
        register.set_info("Info message".to_string());
        let _ = register.view();
    }
}
