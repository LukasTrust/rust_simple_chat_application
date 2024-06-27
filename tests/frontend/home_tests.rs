#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::database::models::User,
        frontend::{
            home::{Home, HomeMessage, TabId},
            tabs_home::{
                group_tab::GroupTabMessage, setting_tab::SettingsTabMessage,
                user_tab::UserTabMessage,
            },
        },
    };

    fn setup_home_with_user() -> Home {
        let mut home = Home::default();
        let current_user = User {
            id: 1,
            first_name: "Alice".to_string(),
            last_name: "Doe".to_string(),
        };
        home.set_current_user(current_user);
        home
    }

    #[test]
    fn test_home_default() {
        let home = Home::default();
        assert!(home.get_current_user().is_none());
        assert_eq!(home.get_active_tab(), TabId::User);
    }

    #[test]
    fn test_load_data() {
        let mut home = setup_home_with_user();

        // Check if user tab works
        home.update(HomeMessage::Tick);

        home.update(HomeMessage::TabSelected(TabId::Group));

        // Check if group tab works
        home.update(HomeMessage::Tick);
    }

    #[test]
    fn test_update_tab_selected() {
        let mut home = setup_home_with_user();

        assert_eq!(home.get_active_tab(), TabId::User);

        home.update(HomeMessage::TabSelected(TabId::Group));
        assert_eq!(home.get_active_tab(), TabId::Group);

        home.update(HomeMessage::TabSelected(TabId::Settings));
        assert_eq!(home.get_active_tab(), TabId::Settings);

        home.update(HomeMessage::TabSelected(TabId::User));
        assert_eq!(home.get_active_tab(), TabId::User);
    }

    #[test]
    fn test_update_user_tab_message() {
        let mut home = Home::default();
        let user_tab_message = UserTabMessage::SendFriendRequestToSelectedUser;
        home.update(HomeMessage::UserTab(user_tab_message.clone()));
    }

    #[test]
    fn test_update_group_tab_message() {
        let mut home = Home::default();
        let group_tab_message = GroupTabMessage::CreateGroup;
        home.update(HomeMessage::GroupTab(group_tab_message.clone()));
    }

    #[test]
    fn test_update_settings_tab_message() {
        let mut home = Home::default();
        let settings_tab_message = SettingsTabMessage::UpdateEmail;
        home.update(HomeMessage::SettingsTab(settings_tab_message.clone()));
    }

    #[test]
    fn test_view() {
        let home = setup_home_with_user();
        let _ = home.view();
    }
}
