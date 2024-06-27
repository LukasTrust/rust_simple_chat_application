#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::{
            database::models::{User, UserMessage},
            entities::user_message_ops::{create_user_message, delete_user_message},
        },
        frontend::user_chat::{UserChat, UserChatMessage},
    };

    fn setup_chat() -> UserChat {
        let current_user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let other_user = User {
            id: 2,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };
        let mut chat = UserChat::default();
        chat.set_properties(current_user.clone(), other_user.clone());
        chat
    }

    #[test]
    fn test_default_user_chat() {
        let chat = UserChat::default();
        assert_eq!(chat.get_current_user().id, -1);
        assert_eq!(chat.get_other_user().id, -1);
        assert!(chat.get_messages().is_empty());
        assert!(chat.get_input_value().is_empty());
    }

    #[test]
    fn test_update_input_changed() {
        let mut chat = UserChat::default();
        chat.update(UserChatMessage::InputChanged("New input".to_string()));
        assert_eq!(chat.get_input_value(), "New input");
    }

    #[test]
    fn test_set_properties() {
        let chat = setup_chat();
        let current_user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let other_user = User {
            id: 2,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };

        assert_eq!(chat.get_current_user(), &current_user);
        assert_eq!(chat.get_other_user(), &other_user);
    }

    #[test]
    fn test_send_user_message() {
        let mut chat = setup_chat();
        assert!(chat.get_messages().is_empty());

        chat.update(UserChatMessage::InputChanged("Hello".to_string()));
        chat.update(UserChatMessage::SendMessage);

        assert_eq!(chat.get_messages().is_empty(), false);
        assert_eq!(chat.get_input_value(), "");

        // Clean up
        let result = delete_user_message(1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_user_message_sql_injection() {
        let mut chat = setup_chat();
        assert!(chat.get_messages().is_empty());

        // Attempt SQL injection in the message
        let sql_injection_payload = "'; DROP TABLE messages; --";
        chat.update(UserChatMessage::InputChanged(
            sql_injection_payload.to_string(),
        ));
        chat.update(UserChatMessage::SendMessage);

        // Ensure that the message was not added due to SQL injection protection
        assert_eq!(chat.get_messages().is_empty(), false,);

        assert_eq!(chat.get_messages()[0].message, sql_injection_payload);

        // Clean up
        let result = delete_user_message(1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_tick() {
        let mut chat = setup_chat();
        assert!(chat.get_messages().is_empty());

        let user_message = create_user_message(1, 2, "Hello, User 2!");
        assert!(user_message.is_ok());

        chat.update(UserChatMessage::Tick);

        assert_eq!(chat.get_messages().is_empty(), false);

        // Clean up
        let result = delete_user_message(1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_view() {
        let mut chat = setup_chat();

        let current_message = UserMessage {
            sender_id: 1,
            receiver_id: 2,
            message: "Hello, User 2!".to_string(),
            send_date: chrono::Utc::now().naive_utc(),
        };

        chat.push_message(current_message);

        let other_message = UserMessage {
            sender_id: 2,
            receiver_id: 1,
            message: "Hello, User 1!".to_string(),
            send_date: chrono::Utc::now().naive_utc(),
        };

        chat.push_message(other_message);

        let _ = chat.view();
    }
}
