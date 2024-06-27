#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::{
            database::models::{Group, GroupMessage, User},
            entities::group_message_ops::{create_group_message, delete_group_messages},
        },
        frontend::group_chat::{GroupChat, GroupChatMessage},
    };

    fn default_user() -> User {
        User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }

    fn default_group() -> Group {
        Group {
            id: 1,
            name: "Test Group".to_string(),
        }
    }

    fn setup_group_chat() -> GroupChat {
        let mut group_chat = GroupChat::default();
        group_chat.set_properties(default_user(), default_group());
        group_chat
    }

    #[test]
    fn test_default_group_chat() {
        let group_chat = GroupChat::default();
        assert_eq!(group_chat.get_current_user().id, -1);
        assert_eq!(group_chat.get_current_group().id, -1);
        assert!(group_chat.get_users_of_group().is_empty());
        assert!(group_chat.get_messages().is_empty());
        assert!(group_chat.get_input_value().is_empty());
    }

    #[test]
    fn test_set_properties() {
        let mut group_chat = GroupChat::default();
        let user = default_user();
        let group = default_group();

        group_chat.set_properties(user.clone(), group.clone());
        assert_eq!(group_chat.get_current_user(), &user);
        assert_eq!(group_chat.get_current_group(), &group);
    }

    #[test]
    fn test_send_group_message_empty() {
        let mut group_chat = GroupChat::default();
        group_chat.update(GroupChatMessage::SendMessage);
        assert!(group_chat.get_messages().is_empty());
    }

    #[test]
    fn test_send_group_message_success() {
        let mut group_chat = setup_group_chat();
        assert!(group_chat.get_messages().is_empty());

        group_chat.update(GroupChatMessage::InputChanged("Hello, world!".to_string()));
        group_chat.update(GroupChatMessage::SendMessage);
        assert!(!group_chat.get_messages().is_empty());

        let result = delete_group_messages(1, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_input_changed() {
        let mut group_chat = GroupChat::default();
        group_chat.update(GroupChatMessage::InputChanged("Hello, world!".to_string()));
        assert_eq!(group_chat.get_input_value(), "Hello, world!");
    }

    #[test]
    fn test_handle_tick() {
        let mut group_chat = setup_group_chat();
        let old_message_count = group_chat.get_messages().len();

        let group_messages = create_group_message(1, 1, "Hello, world!");
        assert!(group_messages.is_ok());

        group_chat.update(GroupChatMessage::Tick);
        assert!(group_chat.get_messages().len() > old_message_count);

        let result = delete_group_messages(1, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_group_message_special_chars() {
        let mut group_chat = setup_group_chat();

        group_chat.update(GroupChatMessage::InputChanged("Hello, @world!".to_string()));
        group_chat.update(GroupChatMessage::SendMessage);
        assert!(!group_chat.get_messages().is_empty());

        let result = delete_group_messages(1, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_tick_failure() {
        let mut group_chat = setup_group_chat();

        // Simulate failure by providing an invalid group ID.
        group_chat.set_properties(
            default_user(),
            Group {
                id: -1,
                name: "Invalid Group".to_string(),
            },
        );
        group_chat.update(GroupChatMessage::Tick);
        assert!(group_chat.get_messages().is_empty());
    }

    #[test]
    fn test_view() {
        let mut group_chat = setup_group_chat();

        let other_user = User {
            id: 2,
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
        };

        group_chat.push_users_of_group(other_user.clone());

        let current_message = GroupMessage {
            sender_id: 1,
            receiver_id: 1,
            send_date: chrono::Utc::now().naive_utc(),
            message: "Hello, world!".to_string(),
        };

        group_chat.push_message(current_message);

        let other_message = GroupMessage {
            sender_id: 2,
            receiver_id: 1,
            send_date: chrono::Utc::now().naive_utc(),
            message: "Hello, world!".to_string(),
        };

        group_chat.push_message(other_message);

        let _ = group_chat.view();
    }
}
