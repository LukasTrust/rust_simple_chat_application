#[cfg(test)]
mod tests {

    use secse24_group08::{
        backend::{
            database::models::User,
            entities::{
                user_friend_ops::create_user_friend,
                user_ops::{create_user, delete_user, find_all_user},
            },
        },
        frontend::tabs_home::user_tab::{UserTab, UserTabMessage},
    };

    fn delete_friends(users: Vec<User>) {
        for user in users {
            let result = delete_user(user.id);
            assert!(result.is_ok());
        }
    }

    fn create_test_user(first_name: &str, last_name: &str, email: &str, password: &str) -> User {
        let user = create_user(first_name, last_name, email, password);
        assert!(user.is_ok());
        user.unwrap()
    }

    #[test]
    fn test_set_current_user() {
        let mut user_tab = UserTab::default();
        let user = User {
            id: 1,
            first_name: "Alice".to_string(),
            last_name: "Doe".to_string(),
        };

        user_tab.set_current_user(user.clone());
        let result = user_tab.get_current_user();

        assert!(result.is_some());
    }

    #[test]
    fn test_send_friend_request() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend1@mail.de", "wta3xr{F)o{uDh$w");
        let selected_user = create_test_user("Bob", "Doe", "friend2@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());
        user_tab.update(UserTabMessage::UserSelected(selected_user.clone()));

        user_tab.update(UserTabMessage::Tick(vec![
            selected_user.clone(),
            current_user.clone(),
        ]));
        user_tab.update(UserTabMessage::SendFriendRequestToSelectedUser);

        assert!(!user_tab
            .get_all_users_not_friends()
            .contains(&selected_user));
        assert!(user_tab.get_send_friend_requests().contains(&selected_user));
        assert!(user_tab.get_selected_user().is_none());

        delete_friends(vec![current_user, selected_user]);
    }

    #[test]
    fn test_handle_tick() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend3@mail.de", "wta3xr{F)o{uDh$w");
        let other_user = create_test_user("Bob", "Doe", "friend4@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());
        user_tab.update(UserTabMessage::Tick(vec![
            other_user.clone(),
            current_user.clone(),
        ]));

        assert!(user_tab.get_all_users_not_friends().contains(&other_user));
        assert!(!user_tab.get_all_users_not_friends().contains(&current_user));

        delete_friends(vec![current_user, other_user]);
    }

    #[test]
    fn test_remove_friend() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend5@mail.de", "wta3xr{F)o{uDh$w");
        let other_user = create_test_user("Bob", "Doe", "friend6@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());

        let friends = create_user_friend(current_user.id, other_user.id, true, true);
        assert!(friends.is_ok());

        let users = find_all_user();
        assert!(users.is_ok());

        let users = users.unwrap();
        user_tab.update(UserTabMessage::Tick(users));

        user_tab.update(UserTabMessage::RemoveFriend(other_user.clone()));

        assert!(user_tab.get_all_users_not_friends().contains(&other_user));
        assert!(!user_tab.get_friends().contains(&other_user));

        delete_friends(vec![current_user, other_user]);
    }

    #[test]
    fn test_accept_friend_request() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend7@mail.de", "wta3xr{F)o{uDh$w");
        let other_user = create_test_user("Bob", "Doe", "friend8@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());
        user_tab.update(UserTabMessage::Tick(vec![
            other_user.clone(),
            current_user.clone(),
        ]));
        user_tab.update(UserTabMessage::UserSelected(other_user.clone()));

        user_tab.update(UserTabMessage::SendFriendRequestToSelectedUser);

        user_tab.set_current_user(other_user.clone());
        user_tab.update(UserTabMessage::UserSelected(current_user.clone()));

        user_tab.update(UserTabMessage::AcceptFriendRequest(current_user.clone()));

        assert!(!user_tab.get_send_friend_requests().contains(&current_user));
        assert!(user_tab.get_friends().contains(&current_user));

        delete_friends(vec![current_user, other_user]);
    }

    #[test]
    fn test_remove_friend_request() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend9@mail.de", "wta3xr{F)o{uDh$w");
        let other_user = create_test_user("Bob", "Doe", "friend10@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());
        user_tab.update(UserTabMessage::UserSelected(other_user.clone()));
        user_tab.update(UserTabMessage::Tick(vec![
            other_user.clone(),
            current_user.clone(),
        ]));
        user_tab.update(UserTabMessage::SendFriendRequestToSelectedUser);

        assert!(user_tab.get_send_friend_requests().contains(&other_user));

        user_tab.update(UserTabMessage::RemoveFriendRequest(other_user.clone()));

        assert!(!user_tab.get_send_friend_requests().contains(&other_user));

        delete_friends(vec![current_user, other_user]);
    }

    #[test]
    fn test_decline_friend_request() {
        let mut user_tab = UserTab::default();

        let current_user = create_test_user("Alice", "Doe", "friend11@mail.de", "wta3xr{F)o{uDh$w");
        let other_user = create_test_user("Bob", "Doe", "friend12@mail.de", "wta3xr{F)o{uDh$w");

        user_tab.set_current_user(current_user.clone());
        user_tab.update(UserTabMessage::UserSelected(other_user.clone()));
        user_tab.update(UserTabMessage::Tick(vec![
            other_user.clone(),
            current_user.clone(),
        ]));
        user_tab.update(UserTabMessage::SendFriendRequestToSelectedUser);

        user_tab.set_current_user(other_user.clone());
        user_tab.update(UserTabMessage::UserSelected(current_user.clone()));

        user_tab.update(UserTabMessage::DeclineFriendRequest(current_user.clone()));

        assert!(!user_tab.get_send_friend_requests().contains(&current_user));
        assert!(!user_tab.get_friends().contains(&current_user));

        delete_friends(vec![current_user, other_user]);
    }

    #[test]
    fn test_update_user_selected() {
        let mut user_tab = UserTab::default();
        let user = User {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Doe".to_string(),
        };

        user_tab.update(UserTabMessage::UserSelected(user.clone()));

        assert_eq!(user_tab.get_selected_user(), Some(user).as_ref());
    }

    #[test]
    fn test_update_chat_with_user() {
        let mut user_tab = UserTab::default();
        let user = User {
            id: 2,
            first_name: "Bob".to_string(),
            last_name: "Doe".to_string(),
        };

        user_tab.update(UserTabMessage::ChatWithUser(user.clone()));

        assert_eq!(user_tab.get_user_to_chat_with(), Some(user).as_ref());
    }

    #[test]
    fn test_view() {
        let mut user_tab = UserTab::default();
        let user = User {
            id: 1,
            first_name: "Bob".to_string(),
            last_name: "Doe".to_string(),
        };

        user_tab.push_friend(user.clone());
        user_tab.push_send_friend_request(user.clone());
        user_tab.push_friend_request(user.clone());

        let _ = user_tab.view();
    }
}
