#[cfg(test)]
mod tests {
    use secse24_group08::{
        backend::{
            database::models::{Group, User},
            entities::{
                group_ops::{create_group, delete_group},
                user_ops::find_all_user,
            },
        },
        frontend::tabs_home::group_tab::{GroupTab, GroupTabMessage},
    };

    // Helper function to create a user
    fn create_test_user(id: i32, first_name: &str, last_name: &str) -> User {
        User {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    // Helper function to create and set current user in group_tab
    fn setup_group_tab_with_user(id: i32, first_name: &str, last_name: &str) -> GroupTab {
        let mut group_tab = GroupTab::default();
        let user = create_test_user(id, first_name, last_name);
        group_tab.set_current_user(user);
        group_tab
    }

    // Helper function to create a test group
    fn create_test_group(name: &str) -> Group {
        let group = create_group(name).expect("Failed to create group");
        group
    }

    // Helper function to clean up group
    fn clean_up_group(group_id: i32) {
        let result = delete_group(group_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_current_user() {
        let mut group_tab = GroupTab::default();
        let user = create_test_user(1, "Alice", "Doe");

        group_tab.set_current_user(user.clone());
        let result = group_tab.get_current_user();

        assert!(result.clone().is_some());
    }

    #[test]
    fn test_group_name_changed() {
        let mut group_tab = GroupTab::default();
        let message = GroupTabMessage::GroupNameChanged("New Group".to_string());

        group_tab.update(message);
        assert_eq!(group_tab.get_new_group_name(), "New Group");
    }

    #[test]
    fn test_create_group() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        group_tab.update(GroupTabMessage::GroupNameChanged("Test Group".to_string()));

        group_tab.update(GroupTabMessage::CreateGroup);

        assert_eq!(group_tab.get_error(), "");
        assert!(!group_tab.get_groups_of_user().is_empty());
        assert_eq!(group_tab.get_groups_of_user()[0].name, "Test Group");

        clean_up_group(group_tab.get_groups_of_user()[0].id);
    }

    #[test]
    fn test_user_selected() {
        let mut group_tab = GroupTab::default();
        let user = create_test_user(2, "John", "Doe");

        group_tab.update(GroupTabMessage::UserSelected(user.clone()));
        let result = group_tab.get_selected_user();

        assert!(result.is_some());
    }

    #[test]
    fn test_chat_with_group() {
        let mut group_tab = GroupTab::default();
        let group = create_test_group("Test Group");

        group_tab.update(GroupTabMessage::ChatWithGroup(group.clone()));
        let result = group_tab.get_group_to_chat_with();

        assert!(result.is_some());

        clean_up_group(group.id);
    }

    #[test]
    fn test_accept_group() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        let group = create_test_group("Test Group");

        group_tab.update(GroupTabMessage::AcceptGroup(group.clone()));
        assert!(!group_tab.get_groups_of_user().is_empty());
        assert_eq!(group_tab.get_groups_of_user()[0], group);

        clean_up_group(group.id);
    }

    #[test]
    fn test_leave_group() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        let group = create_test_group("Test Group");

        group_tab.push_groups_of_user(group.clone());
        group_tab.update(GroupTabMessage::LeaveGroup(group.clone()));

        assert!(group_tab.get_groups_of_user().is_empty());
    }

    #[test]
    fn test_invite_user_to_group() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        let invited_user = create_test_user(2, "Jane", "Doe");

        group_tab.update(GroupTabMessage::UserSelected(invited_user.clone()));
        let group = create_test_group("Test Group");

        group_tab.update(GroupTabMessage::InviteUserToGroup(group.clone()));
        assert_eq!(
            group_tab.get_info(),
            "User Jane Doe invited to group Test Group"
        );

        clean_up_group(group.id);
    }

    #[test]
    fn test_handle_tick() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        let group = create_test_group("Test Group");

        group_tab.push_groups_of_user(group.clone());
        let users = find_all_user().expect("Failed to find users");

        group_tab.update(GroupTabMessage::Tick(users));
        assert_eq!(group_tab.get_groups_of_user().len(), 1);

        clean_up_group(group.id);
    }

    #[test]
    fn test_view() {
        let mut group_tab = setup_group_tab_with_user(1, "John", "Doe");
        let users = find_all_user().expect("Failed to find users");

        group_tab.update(GroupTabMessage::Tick(users));
        group_tab.set_error("Test Error".to_string());
        group_tab.set_info("Test Info".to_string());

        group_tab.push_invited_groups(Group {
            id: 1,
            name: "Test Group".to_string(),
        });

        let _ = group_tab.view();
    }
}
