use iced::{
    alignment::Horizontal,
    widget::{button, column, horizontal_space, pick_list, row, text, text_input},
    Alignment, Color, Element, Length,
};
use log::error;

use crate::backend::{
    database::models::{Group, User, UserToGroup},
    entities::{
        group_ops::{create_group, delete_group, find_groups_by_ids},
        user_friend_ops::find_all_user_to_user_friend_entries,
        user_group_ops::{
            create_user_group, delete_user_group, find_all_user_groups_of_group,
            find_all_user_groups_of_user, update_user_group,
        },
    },
};

/// Represents the group tab in the home screen
#[derive(Default, Debug, Clone)]
pub struct GroupTab {
    current_user: Option<User>,
    group_to_chat_with: Option<Group>,
    // Create Group
    new_group_name: String,
    // Selection
    friends_of_user: Vec<User>,
    selected_user: Option<User>,
    // Groups of User
    groups_of_user: Vec<Group>,
    // Invited Groups
    invited_groups: Vec<Group>,
    error: String,
    info: String,
}

/// Represents the messages that can be sent to the group tab
#[derive(Debug, Clone)]
pub enum GroupTabMessage {
    // Create Group
    GroupNameChanged(String),
    CreateGroup,
    // Selection
    UserSelected(User),
    InviteUserToGroup(Group),
    // Groups
    ChatWithGroup(Group),
    LeaveGroup(Group),
    // Invited Groups
    AcceptGroup(Group),
    // Load data
    Tick(Vec<User>),
}

/// Implementation of the group tab
impl GroupTab {
    /// Clears the group tab
    fn clear(&mut self) {
        self.friends_of_user.clear();
        self.groups_of_user.clear();
        self.invited_groups.clear();
    }

    /// Sets the current user
    pub fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user);
    }

    /// Checks if a group has users
    fn check_if_group_has_users(&self, group: &Group) -> bool {
        let group = find_groups_by_ids(vec![group.id]);

        match group {
            Ok(group) => {
                if group.is_empty() {
                    return false;
                }

                let group = group.first().unwrap();
                let users = find_all_user_groups_of_group(group.id);
                match users {
                    Ok(users) => !users.is_empty(),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Handles the result of creating a group. If successful, adds the user to the group
    fn handle_create_group_result(&mut self, result: Result<Group, String>) {
        match result {
            Ok(group) => {
                self.error.clear();
                self.add_user_to_group(group);
            }
            Err(e) => {
                error!("Error creating group: {}", e);
                self.error = format!("Error creating group: {}", self.new_group_name);
                self.info = String::new();
            }
        }
    }

    /// Adds a user to a group. If successful, adds the group to the list of groups of the user
    fn add_user_to_group(&mut self, group: Group) {
        let result = create_user_group(self.current_user.as_ref().unwrap().id, group.id, true);

        match result {
            Ok(_) => {
                self.info = format!("Group {} created", group.name);
                self.error = String::new();
                self.groups_of_user.push(group);
            }
            Err(e) => {
                let _ = delete_group(group.id);
                error!("Error creating group: {}", e);
                self.error = format!("Error creating group: {}", group.name);
                self.info = String::new();
            }
        }
    }

    /// Invites a user to a group. If successful, the ohter user receives an invite
    fn invite_user_to_group(&mut self, group: Group) {
        if let Some(user) = &self.selected_user {
            match create_user_group(user.id, group.id, false) {
                Ok(_) => {
                    self.info = format!(
                        "User {} {} invited to group {}",
                        user.first_name, user.last_name, group.name
                    );
                    self.error = String::new();
                }
                Err(e) => {
                    error!("Error inviting user to group: {}", e);
                    self.error = format!(
                        "User {} {} is already in group {}",
                        user.first_name, user.last_name, group.name
                    );
                    self.info = String::new();
                }
            }
            self.selected_user = None;
        } else {
            self.error = "Select a user to invite".to_string();
            self.info = String::new();
        }
    }

    /// Accepts an invite to a group. If successful, the user is added to the group
    fn accept_group_invite(&mut self, group: Group) {
        let user_id = self.current_user.as_ref().unwrap().id;

        match update_user_group(user_id, group.id, true) {
            Ok(_) => {
                self.info = format!("Accepted invite to group {}", group.name);
                self.invited_groups.retain(|g| g.id != group.id);
                self.groups_of_user.push(group);
            }
            Err(e) => {
                error!("Error accepting invite to group: {}", e);
                self.error = format!("Error accepting invite to group: {}", group.name);
                self.info = String::new();
            }
        }
    }

    /// Leaves a group. If successful, the user is removed from the group. If the group then has no users, it is deleted
    fn leave_group(&mut self, group: Group) {
        let user_id = self.current_user.as_ref().unwrap().id;

        match delete_user_group(user_id, group.id) {
            Ok(_) => {
                self.info = format!("Left group {}", group.name);
                self.error = String::new();
                self.groups_of_user.retain(|g| g.id != group.id);
                self.invited_groups.retain(|g| g.id != group.id);

                if !self.check_if_group_has_users(&group) {
                    let _ = delete_group(group.id);
                }
            }
            Err(e) => {
                error!("Error leaving group: {}", e);
                self.error = format!("Error leaving group: {}", group.name);
                self.info = String::new();
            }
        }
    }

    /// Loads the groups and invited groups of a user
    fn load_user_groups(&mut self, current_user_id: i32) {
        match find_all_user_groups_of_user(current_user_id) {
            Ok(user_groups) => {
                self.load_accepted_groups(&user_groups);
                self.load_invited_groups(&user_groups);
            }
            Err(e) => {
                error!("Error loading user_groups: {}", e);
            }
        }
    }

    /// Loads the groups that a user has accepted invites to
    fn load_accepted_groups(&mut self, user_groups: &[UserToGroup]) {
        let accepted_groups_ids: Vec<i32> = user_groups
            .iter()
            .filter(|group| group.accepted_invite)
            .map(|group| group.group_id)
            .collect();

        match find_groups_by_ids(accepted_groups_ids) {
            Ok(groups) => {
                self.groups_of_user = groups;
            }
            Err(e) => {
                error!("Error loading groups: {}", e);
            }
        }
    }

    /// Loads the groups that a user has been invited to
    fn load_invited_groups(&mut self, user_groups: &[UserToGroup]) {
        let invited_groups_ids: Vec<i32> = user_groups
            .iter()
            .filter(|group| !group.accepted_invite)
            .map(|group| group.group_id)
            .collect();

        match find_groups_by_ids(invited_groups_ids) {
            Ok(groups) => {
                self.invited_groups = groups;
            }
            Err(e) => {
                error!("Error loading invited groups: {}", e);
            }
        }
    }

    fn handle_tick(&mut self, users: Vec<User>) {
        self.clear();

        let friend_relations =
            find_all_user_to_user_friend_entries(self.current_user.as_ref().unwrap().id);

        match friend_relations {
            Ok(friends) => {
                let accepted_friends: Vec<_> = friends
                    .into_iter()
                    .filter(|friend_accepted| {
                        friend_accepted.accepted_user_one && friend_accepted.accepted_user_two
                    })
                    .collect();

                for friend in accepted_friends {
                    let user = users
                        .iter()
                        .find(|user| {
                            user.id != self.current_user.as_ref().unwrap().id
                                && user.id == friend.user_one_id
                                || user.id == friend.user_two_id
                        })
                        .unwrap();

                    self.friends_of_user.push(user.clone());
                }
            }
            Err(e) => {
                error!("Error loading friends: {}", e);
            }
        }

        // Filter out the current user from the list of all users
        self.friends_of_user
            .retain(|user| user != self.current_user.as_ref().unwrap());

        let current_user_id = self.current_user.as_ref().unwrap().id;
        self.load_user_groups(current_user_id);
    }

    /// Updates the group tab based on a message
    pub fn update(&mut self, message: GroupTabMessage) {
        match message {
            GroupTabMessage::UserSelected(user) => {
                self.selected_user = Some(user);
            }
            GroupTabMessage::GroupNameChanged(group) => {
                self.new_group_name = group;
            }
            GroupTabMessage::ChatWithGroup(group) => {
                self.group_to_chat_with = Some(group);
            }
            GroupTabMessage::CreateGroup => {
                if self.new_group_name.is_empty() {
                    self.error = "Group name cannot be empty".to_string();
                    self.info = String::new();
                    return;
                }

                let result = create_group(self.new_group_name.as_str());
                self.handle_create_group_result(result);
            }
            GroupTabMessage::InviteUserToGroup(group) => {
                self.invite_user_to_group(group);
            }
            GroupTabMessage::LeaveGroup(group) => {
                self.leave_group(group);
            }
            GroupTabMessage::AcceptGroup(group) => {
                self.accept_group_invite(group);
            }
            GroupTabMessage::Tick(users) => {
                self.handle_tick(users);
            }
        }
    }

    /// Returns the view of the group tab
    pub fn view(&self) -> Element<GroupTabMessage> {
        let button_width = 150;
        let interact_button_width = 150;
        let padding = 10;
        let spacing = 20;

        // Create Group
        let group_name_input = text_input("Group name:", self.new_group_name.as_str())
            .width(300)
            .padding(padding)
            .on_input(GroupTabMessage::GroupNameChanged);

        let create_group_button = button("Create group")
            .width(300)
            .padding(padding)
            .on_press(GroupTabMessage::CreateGroup);

        let create_new_group_column =
            column!(group_name_input, create_group_button,).spacing(spacing);

        // Selection
        let user_pick_list = pick_list(
            self.friends_of_user.clone(),
            self.selected_user.clone(),
            GroupTabMessage::UserSelected,
        )
        .width(300)
        .placeholder("Select a friend to add to a group");

        let error_message: Element<GroupTabMessage> = if !self.error.is_empty() {
            text(&self.error)
                .size(15)
                .style(Color::from_rgb(1.0, 0.0, 0.0)) // Red color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let info_message: Element<GroupTabMessage> = if !self.info.is_empty() {
            text(&self.info)
                .size(15)
                .style(Color::from_rgb(0.2, 0.8, 0.2)) //Green color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let user_pick = column![user_pick_list, error_message, info_message].spacing(spacing);

        let top_row = row!(create_new_group_column, horizontal_space(), user_pick,)
            .spacing(spacing)
            .width(Length::Fill)
            .padding(padding);

        // Groups of User

        let mut group_colum = column!(text("Groups:")).spacing(spacing);

        for group in &self.groups_of_user {
            let group_button = button(text(&group.name).horizontal_alignment(Horizontal::Center))
                .width(button_width)
                .on_press(GroupTabMessage::ChatWithGroup(group.clone()));

            let add_user_button =
                button(text("Invite user to group").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(GroupTabMessage::InviteUserToGroup(group.clone()));

            let leave_group_button =
                button(text("Leave group").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(GroupTabMessage::LeaveGroup(group.clone()));

            let group_row = row!(group_button, add_user_button, leave_group_button)
                .spacing(spacing)
                .align_items(Alignment::Center);

            group_colum = group_colum.push(group_row);
        }

        let mut invited_group_rows = column!(text("Invited groups:")).spacing(spacing);

        for group in &self.invited_groups {
            let group_text = text(&group.name).width(button_width);

            let accept_button =
                button(text("Accept group invite").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(GroupTabMessage::AcceptGroup(group.clone()));

            let decline_button =
                button(text("Reject group invite").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(GroupTabMessage::LeaveGroup(group.clone()));

            let invited_group_row = row!(group_text, accept_button, decline_button)
                .spacing(spacing)
                .align_items(Alignment::Center);

            invited_group_rows = invited_group_rows.push(invited_group_row);
        }

        let group_rows = row![group_colum, horizontal_space(), invited_group_rows,]
            .spacing(spacing)
            .padding(padding);

        let content = column![top_row, group_rows,].spacing(spacing);

        content.into()
    }
}

/// Geter methods for tests
impl GroupTab {
    /// Getter for the current user
    pub fn get_current_user(&self) -> &Option<User> {
        &self.current_user
    }

    /// Getter for the group to chat with
    pub fn get_group_to_chat_with(&self) -> &Option<Group> {
        &self.group_to_chat_with
    }

    /// Getter for the new group name
    pub fn get_new_group_name(&self) -> &str {
        &self.new_group_name
    }

    /// Getter for the selected user
    pub fn get_selected_user(&self) -> &Option<User> {
        &self.selected_user
    }

    /// Getter for the friends of the user
    pub fn get_groups_of_user(&self) -> &Vec<Group> {
        &self.groups_of_user
    }

    /// Getter for the error
    pub fn get_error(&self) -> &str {
        &self.error
    }

    /// Getter for the info
    pub fn get_info(&self) -> &str {
        &self.info
    }
}

/// Setter methods for tests
impl GroupTab {
    /// Setter for the error
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    /// Setter for the info
    pub fn set_info(&mut self, info: String) {
        self.info = info;
    }

    /// Pushes a group to the invited groups
    pub fn push_invited_groups(&mut self, group: Group) {
        self.invited_groups.push(group);
    }

    /// Pushes a group to the groups of the user
    pub fn push_groups_of_user(&mut self, group: Group) {
        self.groups_of_user.push(group);
    }
}
