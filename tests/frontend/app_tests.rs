#[cfg(test)]
mod tests {
    use std::time::Duration;

    use iced::{time, Application, Subscription};
    use secse24_group08::{
        backend::{
            database::models::{Group, User},
            entities::user_ops::create_user,
        },
        frontend::{
            app::{App, Message, Page},
            group_chat::GroupChatMessage,
            home::{HomeMessage, TabId},
            login::LoginMessage,
            register::RegisterMessage,
            tabs_home::{
                group_tab::GroupTabMessage,
                setting_tab::{AppTheme, SettingsTabMessage},
                user_tab::UserTabMessage,
            },
            user_chat::UserChatMessage,
        },
    };

    fn setup_app() -> (App, iced::Command<Message>) {
        App::new(())
    }

    fn test_create_user(id: i32, first_name: &str, last_name: &str) -> User {
        User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    #[test]
    fn test_initial_state() {
        let (app, _cmd) = setup_app();
        assert_eq!(app.get_current_page(), Page::Login);
        assert!(app.get_current_user().is_none());
    }

    #[test]
    fn test_switch_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::Register));
        assert_eq!(app.get_current_page(), Page::Register);

        let _ = app.update(Message::SwitchPage(Page::Home));
        assert_eq!(app.get_current_page(), Page::Home);
    }

    #[test]
    fn test_handle_login_message_navigate_to_register() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::LoginMessage(LoginMessage::NavigateToRegister));
        assert_eq!(app.get_current_page(), Page::Register);
    }

    #[test]
    fn test_handle_login_message_submit_login() {
        let (mut app, _cmd) = setup_app();

        app.set_current_user(test_create_user(1, "John", "Doe"));
        let _ = app.update(Message::LoginMessage(LoginMessage::SubmitLogin));

        assert_eq!(app.get_current_page(), Page::Home);
        assert!(app.get_current_user().is_some());
    }

    #[test]
    fn test_handle_register_message_navigate_to_login() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::RegisterMessage(RegisterMessage::NavigateToLogin));
        assert_eq!(app.get_current_page(), Page::Login);
    }

    #[test]
    fn test_handle_home_message_navigate_to_login() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::HomeMessage(HomeMessage::NavigateToLogin));
        assert_eq!(app.get_current_page(), Page::Login);
        assert!(app.get_current_user().is_none());
    }

    #[test]
    fn test_handle_home_message_tab_selected_settings() {
        let (mut app, _cmd) = setup_app();
        app.set_current_user(test_create_user(1, "John", "Doe"));
        let _ = app.update(Message::HomeMessage(HomeMessage::TabSelected(
            TabId::Settings,
        )));
        assert_eq!(app.get_home().active_tab(), TabId::Settings);
    }

    #[test]
    fn test_handle_home_message_tab_selected_user() {
        let (mut app, _cmd) = setup_app();
        let current_user = test_create_user(1, "John", "Doe");
        let other_user = test_create_user(2, "Jane", "Doe");

        app.set_current_user(current_user.clone());
        let _ = app.update(Message::HomeMessage(HomeMessage::UserTab(
            UserTabMessage::SendFriendRequestToSelectedUser,
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::UserTab(
            UserTabMessage::ChatWithUser(other_user),
        )));
        app.set_current_user(current_user);
    }

    #[test]
    fn test_handle_home_message_tab_selected_group() {
        let (mut app, _cmd) = setup_app();
        let current_user = test_create_user(1, "John", "Doe");
        let group = Group {
            id: 1,
            name: "Group 1".to_string(),
        };

        app.set_current_user(current_user);
        let _ = app.update(Message::HomeMessage(HomeMessage::GroupTab(
            GroupTabMessage::ChatWithGroup(group),
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::GroupTab(
            GroupTabMessage::CreateGroup,
        )));
    }

    #[test]
    fn test_handle_user_chat_message_back() {
        let (mut app, _cmd) = setup_app();
        app.set_current_user(test_create_user(1, "John", "Doe"));

        let _ = app.update(Message::SwitchPage(Page::UserChat));
        let _ = app.update(Message::UserChatMessage(UserChatMessage::Back));

        assert_eq!(app.get_current_page(), Page::Home);
    }

    #[test]
    fn test_handle_group_chat_message_back() {
        let (mut app, _cmd) = setup_app();
        app.set_current_user(test_create_user(1, "John", "Doe"));

        let _ = app.update(Message::SwitchPage(Page::GroupChat));
        let _ = app.update(Message::GroupChatMessage(GroupChatMessage::Back));

        assert_eq!(app.get_current_page(), Page::Home);
    }

    #[test]
    fn test_handle_settings_tab_message_delete_account_positive() {
        let (mut app, _cmd) = setup_app();

        let _ = app.update(Message::SwitchPage(Page::Home));
        let user = create_user("John", "Doe", "app@email.de", "password").unwrap();

        app.set_current_user(user);
        let _ = app.update(Message::HomeMessage(HomeMessage::TabSelected(
            TabId::Settings,
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::SettingsTab(
            SettingsTabMessage::DeleteAccount,
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::SettingsTab(
            SettingsTabMessage::DeleteAccount,
        )));

        assert_eq!(app.get_current_page(), Page::Login);
        assert!(app.get_current_user().is_none());
    }

    #[test]
    fn test_handle_settings_tab_message_delete_account_negative() {
        let (mut app, _cmd) = setup_app();

        let _ = app.update(Message::SwitchPage(Page::Home));
        app.set_current_user(test_create_user(1, "John", "Doe"));
        let _ = app.update(Message::HomeMessage(HomeMessage::TabSelected(
            TabId::Settings,
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::SettingsTab(
            SettingsTabMessage::DeleteAccount,
        )));

        assert_eq!(app.get_current_page(), Page::Home);
        assert!(app.get_current_user().is_some());
    }

    #[test]
    fn test_handle_settings_tab_message_change_theme() {
        let (mut app, _cmd) = setup_app();

        app.set_current_user(test_create_user(1, "John", "Doe"));
        let _ = app.update(Message::SwitchPage(Page::Home));
        let _ = app.update(Message::HomeMessage(HomeMessage::TabSelected(
            TabId::Settings,
        )));
        let _ = app.update(Message::HomeMessage(HomeMessage::SettingsTab(
            SettingsTabMessage::ChangeTheme(AppTheme::Moonfly),
        )));

        assert_eq!(
            app.get_app_theme(),
            app.get_home().get_settings_tab().get_app_theme()
        );
    }

    #[test]
    fn test_subscription_home_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::Home));
        let subscription = app.subscription();

        let expected_subscription =
            time::every(Duration::from_secs(10)).map(|_| Message::HomeMessage(HomeMessage::Tick));

        assert_eq!(
            format!("{:?}", subscription),
            format!("{:?}", expected_subscription)
        );
    }

    #[test]
    fn test_subscription_user_chat_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::UserChat));
        let subscription = app.subscription();

        let expected_subscription = time::every(Duration::from_secs(5))
            .map(|_| Message::UserChatMessage(UserChatMessage::Tick));

        assert_eq!(
            format!("{:?}", subscription),
            format!("{:?}", expected_subscription)
        );
    }

    #[test]
    fn test_subscription_group_chat_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::GroupChat));
        let subscription = app.subscription();

        let expected_subscription = time::every(Duration::from_secs(5))
            .map(|_| Message::GroupChatMessage(GroupChatMessage::Tick));

        assert_eq!(
            format!("{:?}", subscription),
            format!("{:?}", expected_subscription)
        );
    }

    #[test]
    fn test_subscription_login_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::Login));
        let subscription = app.subscription();

        let expected_subscription: Subscription<Message> = Subscription::none();

        assert_eq!(
            format!("{:?}", subscription),
            format!("{:?}", expected_subscription)
        );
    }

    #[test]
    fn test_subscription_register_page() {
        let (mut app, _cmd) = setup_app();
        let _ = app.update(Message::SwitchPage(Page::Register));
        let subscription = app.subscription();

        let expected_subscription: Subscription<Message> = Subscription::none();

        assert_eq!(
            format!("{:?}", subscription),
            format!("{:?}", expected_subscription)
        );
    }

    #[test]
    fn test_view() {
        let app = setup_app().0;
        app.view();
    }
}
