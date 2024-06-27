#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::{database::models::User, entities::user_ops::create_user},
        frontend::tabs_home::setting_tab::{AppTheme, SettingTab, SettingsTabMessage},
    };

    fn create_default_setting_tab() -> SettingTab {
        SettingTab::default()
    }

    fn create_test_user() -> User {
        User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }

    fn set_up_user_in_tab(setting_tab: &mut SettingTab, user: User) {
        setting_tab.set_current_user(user);
    }

    #[test]
    fn test_default_setting_tab() {
        let setting_tab = create_default_setting_tab();
        assert_eq!(setting_tab.get_current_user(), None);
        assert_eq!(setting_tab.get_error(), "");
        assert_eq!(setting_tab.get_info(), "");
        assert_eq!(setting_tab.get_app_theme(), AppTheme::Moonfly);
        assert_eq!(setting_tab.get_new_email_value(), "");
        assert_eq!(setting_tab.get_new_password_value(), "");
        assert_eq!(setting_tab.get_current_password_value(), "");
        assert!(!setting_tab.get_delete_button_pressed());
        assert!(!setting_tab.get_account_deleted());
    }

    #[test]
    fn test_set_current_user() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        let result = setting_tab.get_current_user();
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, 1);
    }

    #[test]
    fn test_set_app_theme() {
        let mut setting_tab = create_default_setting_tab();
        setting_tab.set_app_theme(AppTheme::Dracula);
        assert_eq!(setting_tab.get_app_theme(), AppTheme::Dracula);
    }

    #[test]
    fn test_update_password_success() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::NewPasswordInputChanged(
            "wta3xr{F)o{uDh$w".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::CurrentPasswordInputChanged(
            "n)+L8ZVWw$qKXDQo".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdatePassword);

        assert_eq!(setting_tab.get_error(), "");
        assert_eq!(setting_tab.get_info(), "Password updated successfully");

        // Clean up
        setting_tab.update(SettingsTabMessage::NewPasswordInputChanged(
            "n)+L8ZVWw$qKXDQo".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::CurrentPasswordInputChanged(
            "wta3xr{F)o{uDh$w".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdatePassword);
    }

    #[test]
    fn test_update_password_failure_weak() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::NewPasswordInputChanged(
            "weak".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::CurrentPasswordInputChanged(
            "n)+L8ZVWw$qKXDQo".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdatePassword);

        assert_eq!(setting_tab.get_error(), "Password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one digit, and one special character");
    }

    #[test]
    fn test_update_password_failure_wrong() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::NewPasswordInputChanged(
            "wta3xr{F)o{uDh$w".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::CurrentPasswordInputChanged(
            "wrong password".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdatePassword);

        assert_eq!(setting_tab.get_error(), "Old password does not match.");
    }

    #[test]
    fn test_update_email_success() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::EmailInputChanged(
            "newuser@example.com".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdateEmail);

        assert_eq!(setting_tab.get_error(), "");
        assert_eq!(setting_tab.get_info(), "Email updated successfully");

        // Clean up
        setting_tab.update(SettingsTabMessage::EmailInputChanged(
            "test1@email.de".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdateEmail);
        assert_eq!(setting_tab.get_error(), "");
    }

    #[test]
    fn test_update_email_failure() {
        let mut setting_tab = create_default_setting_tab();
        let user = create_test_user();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::EmailInputChanged(
            "invalid-email".to_string(),
        ));
        setting_tab.update(SettingsTabMessage::UpdateEmail);
        assert_eq!(setting_tab.get_error(), "Invalid email format");
    }

    #[test]
    fn test_delete_account() {
        let user_result = create_user("John", "Doe", "test3@email.de", "password");
        assert!(user_result.is_ok());

        let user = user_result.unwrap();
        let mut setting_tab = create_default_setting_tab();
        set_up_user_in_tab(&mut setting_tab, user);

        setting_tab.update(SettingsTabMessage::DeleteAccount);
        assert!(setting_tab.get_delete_button_pressed());

        setting_tab.update(SettingsTabMessage::DeleteAccount);
        assert!(setting_tab.get_account_deleted());
    }

    #[test]
    fn test_view() {
        let mut setting_tab = create_default_setting_tab();
        setting_tab.set_error("Error".to_string());
        setting_tab.set_info("Info".to_string());
        setting_tab.set_delete_button_pressed(true);

        let _ = setting_tab.view();
    }
}
