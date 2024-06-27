use iced::{
    alignment::Horizontal,
    widget::{button, column, horizontal_space, pick_list, row, text},
    Alignment, Element,
};
use log::error;

use crate::backend::{
    database::models::{User, UserToUserFriend},
    entities::user_friend_ops::{
        acccepte_friend_request, create_user_friend, delete_friend_to_friend_relation,
        find_all_user_to_user_friend_entries,
    },
};

/// Represents the user tab in the home tab
#[derive(Default, Debug, Clone)]
pub struct UserTab {
    current_user: Option<User>,
    user_to_chat_with: Option<User>,
    // Selection
    all_users_not_friends: Vec<User>,
    selected_user: Option<User>,
    // Friends
    friends: Vec<User>,
    // Requests
    friend_requests: Vec<User>,
    // Send friend request
    send_friend_request: Vec<User>,
}

/// Represents the messages that can be sent to the user tab
#[derive(Debug, Clone)]
pub enum UserTabMessage {
    // Selection
    UserSelected(User),
    SendFriendRequestToSelectedUser,
    // Friends
    ChatWithUser(User),
    RemoveFriend(User),
    // Accept or decline friend request
    AcceptFriendRequest(User),
    DeclineFriendRequest(User),
    // Requests
    RemoveFriendRequest(User),
    // Load data
    Tick(Vec<User>),
}

/// Implementation of the user tab
impl UserTab {
    /// Sets the current user
    pub fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user);
    }

    /// Clears all lists
    fn clear_lists(&mut self) {
        self.all_users_not_friends.clear();
        self.friends.clear();
        self.friend_requests.clear();
        self.send_friend_request.clear();
    }

    /// Loads all friends of the current user
    fn load_friends_of_user(current_user_id: i32) -> Vec<UserToUserFriend> {
        let all_user_friends = find_all_user_to_user_friend_entries(current_user_id);

        match all_user_friends {
            Ok(all_user_friends) => all_user_friends,
            Err(e) => {
                error!("Error loading user friends: {:?}", e);
                vec![]
            }
        }
    }

    /// Finds the lower and higher user id
    fn find_lower_and_higher_user_id(&mut self, other_user_id: i32) -> (i32, i32) {
        let current_user_id = self.current_user.as_ref().unwrap().id;

        if current_user_id < other_user_id {
            (current_user_id, other_user_id)
        } else {
            (other_user_id, current_user_id)
        }
    }

    /// Accepts a friend request. Sets the lower id to the first user and the higher id to the second user
    fn acccepte_friend_request(&mut self, other_user_id: i32) -> Result<(), String> {
        let (lower_user_id, higher_user_id) = self.find_lower_and_higher_user_id(other_user_id);

        acccepte_friend_request(lower_user_id, higher_user_id)
    }

    /// Removes a friend request. Sets the lower id to the first user and the higher id to the second user
    fn remove_friend_request(&mut self, other_user_id: i32) -> Result<(), String> {
        let (lower_user_id, higher_user_id) = self.find_lower_and_higher_user_id(other_user_id);

        delete_friend_to_friend_relation(lower_user_id, higher_user_id)
    }

    /// Sends a friend request. Selected user must be set. Sets the lower id to the first user and the higher id to the second user.
    /// Removes the user from the able to send requests list and adds the user to the sent friend requests list
    fn send_friend_request(&mut self) {
        if self.selected_user.is_none() {
            return;
        }

        // Send friend request
        let current_user_id = self.current_user.as_ref().unwrap().id;
        let other_user_id = self.selected_user.as_ref().unwrap().id;

        let (lower_user_id, higher_user_id) = self.find_lower_and_higher_user_id(other_user_id);

        let result = create_user_friend(
            lower_user_id,
            higher_user_id,
            current_user_id == lower_user_id,
            current_user_id == higher_user_id,
        );
        match result {
            Ok(_) => {
                // Remove user from able to send requests list
                let user_that_got_friend_requst = self
                    .all_users_not_friends
                    .iter()
                    .position(|u| u.id == other_user_id)
                    .unwrap();
                self.all_users_not_friends
                    .remove(user_that_got_friend_requst);
                // Add user to sent friend requests list
                self.send_friend_request
                    .push(self.selected_user.as_ref().unwrap().clone());

                self.selected_user = None;
            }
            Err(e) => {
                error!("Error sending friend request: {:?}", e);
            }
        }
    }

    /// Handles the tick event. Loads user data and filters out the current user. Fills the different lists with the user data
    fn handle_tick(&mut self, users: Vec<User>) {
        // Load user data asynchronously
        let friends = Self::load_friends_of_user(self.current_user.as_ref().unwrap().id);

        // Clears lists to avoid duplicates
        self.clear_lists();

        // Filter out the current user
        let current_user_id = self.current_user.as_ref().unwrap().id;
        let mut users = users
            .into_iter()
            .filter(|user| user.id != current_user_id)
            .collect::<Vec<_>>();

        // Iterate over friends to fill different lists
        for friend in &friends {
            // Check if the current user is involved in the friendship
            let (other_user_id, current_user_accepted, other_user_accepted) =
                if friend.user_one_id == current_user_id {
                    (
                        friend.user_two_id,
                        friend.accepted_user_one,
                        friend.accepted_user_two,
                    )
                } else if friend.user_two_id == current_user_id {
                    (
                        friend.user_one_id,
                        friend.accepted_user_two,
                        friend.accepted_user_one,
                    )
                } else {
                    continue; // Skip if the current user is not involved
                };

            let other_user = users
                .iter()
                .position(|u| u.id == other_user_id)
                .map(|i| users.remove(i));

            // Check if the friendship is accepted
            if current_user_accepted {
                if other_user_accepted {
                    // Add friend
                    self.friends.push(other_user.unwrap());
                } else {
                    // Add send friend request
                    self.send_friend_request.push(other_user.unwrap());
                }
            } else {
                // Add friend request
                self.friend_requests.push(other_user.unwrap());
            }
        }

        // Fill users not in any list
        for user in users {
            // Check if the user is not already in any other list
            if !self.friends.contains(&user)
                && !self.friend_requests.contains(&user)
                && !self.send_friend_request.contains(&user)
            {
                self.all_users_not_friends.push(user);
            }
        }
    }

    /// Updates the user tab based on the message
    pub fn update(&mut self, message: UserTabMessage) {
        match message {
            // Selection
            UserTabMessage::UserSelected(user) => {
                self.selected_user = Some(user);
            }
            // Friends
            UserTabMessage::ChatWithUser(user) => {
                self.user_to_chat_with = Some(user);
            }
            UserTabMessage::SendFriendRequestToSelectedUser => {
                self.send_friend_request();
            }
            UserTabMessage::RemoveFriend(user) => {
                let result = self.remove_friend_request(user.id);

                match result {
                    Ok(_) => {
                        // Remove user from friends list
                        self.friends.retain(|u| u != &user);
                        self.all_users_not_friends.push(user);
                    }
                    Err(e) => {
                        error!("Error removing friend: {:?}", e);
                    }
                }
            }
            // Accept or decline friend request
            UserTabMessage::AcceptFriendRequest(user) => {
                let result = self.acccepte_friend_request(user.id);

                match result {
                    Ok(_) => {
                        // Remove user from friend requests list
                        self.friend_requests.retain(|u| u != &user);
                        // Add user to friends list
                        self.friends.push(user);
                    }
                    Err(e) => {
                        error!("Error accepting friend request: {:?}", e);
                    }
                }
            }
            UserTabMessage::DeclineFriendRequest(user) => {
                let result = self.remove_friend_request(user.id);

                match result {
                    Ok(_) => {
                        // Remove user from friend requests list
                        self.friend_requests.retain(|u| u != &user);
                        self.all_users_not_friends.push(user);
                    }
                    Err(e) => {
                        error!("Error declining friend request: {:?}", e);
                    }
                }
            }
            // Requests
            UserTabMessage::RemoveFriendRequest(user) => {
                let result = self.remove_friend_request(user.id);

                match result {
                    Ok(_) => {
                        // Remove user from friend requests list
                        self.send_friend_request.retain(|u| u != &user);
                        self.all_users_not_friends.push(user);
                    }
                    Err(e) => {
                        error!("Error removing friend request: {:?}", e);
                    }
                }
            }
            // Load data
            UserTabMessage::Tick(users) => {
                self.handle_tick(users);
            }
        }
    }

    /// Returns the view of the user tab
    pub fn view(&self) -> Element<UserTabMessage> {
        let button_width = 150;
        let interact_button_width = 150;
        let padding = 10;
        let spacing = 20;

        // Pick user column

        let user_pick_list = pick_list(
            self.all_users_not_friends.clone(),
            self.selected_user.clone(),
            UserTabMessage::UserSelected,
        )
        .width(300)
        .placeholder("Select a user to add as a friend");

        let send_friend_request_button =
            button(text("Send friend request").horizontal_alignment(Horizontal::Center))
                .width(300)
                .padding(padding)
                .on_press(UserTabMessage::SendFriendRequestToSelectedUser);

        let pick_user_column = column![user_pick_list, send_friend_request_button]
            .align_items(Alignment::End)
            .spacing(spacing)
            .padding(padding);

        // All friends row
        let mut friends_column = column![text("Friends:")].spacing(spacing);

        for friend in &self.friends {
            let friend_button = button(
                text(format!("{} {}", friend.first_name, friend.last_name))
                    .horizontal_alignment(Horizontal::Center),
            )
            .width(button_width)
            .on_press(UserTabMessage::ChatWithUser(friend.clone()));

            let remove_friend_button =
                button(text("Unfriend user").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(UserTabMessage::RemoveFriend(friend.clone()));

            let friends_row = row![friend_button, remove_friend_button]
                .spacing(spacing)
                .align_items(Alignment::Center);

            friends_column = friends_column.push(friends_row);
        }

        // Friend requests
        let mut friend_requests_column = column![text("Friend requests:")].spacing(spacing);

        for friend_request in &self.friend_requests {
            let friend_requests = text(format!(
                "{} {}",
                friend_request.first_name, friend_request.last_name
            ));

            let accept_button =
                button(text("Accept friend invite").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(UserTabMessage::AcceptFriendRequest(friend_request.clone()));

            let decline_button =
                button(text("Reject friend invite").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(UserTabMessage::DeclineFriendRequest(friend_request.clone()));

            let friend_requests_row = row![friend_requests, accept_button, decline_button]
                .spacing(spacing)
                .align_items(Alignment::Center);

            friend_requests_column = friend_requests_column.push(friend_requests_row);
        }

        // Send friend requests
        let mut send_friend_request_column =
            column![text("Sent friend requests:")].spacing(spacing);

        for send_friend_request in &self.send_friend_request {
            let send_friend_request_text = text(format!(
                "{} {}",
                send_friend_request.first_name, send_friend_request.last_name
            ));

            let remove_friend_request_button =
                button(text("Retract invite").horizontal_alignment(Horizontal::Center))
                    .width(interact_button_width)
                    .on_press(UserTabMessage::RemoveFriendRequest(
                        send_friend_request.clone(),
                    ));

            let send_friend_request_row =
                row![send_friend_request_text, remove_friend_request_button]
                    .spacing(spacing)
                    .align_items(Alignment::Center);

            send_friend_request_column = send_friend_request_column.push(send_friend_request_row);
        }

        let all_friends_row = row![
            friends_column,
            horizontal_space(),
            friend_requests_column,
            horizontal_space(),
            send_friend_request_column
        ]
        .spacing(spacing)
        .padding(padding);

        let content = column![pick_user_column, all_friends_row].spacing(spacing);

        content.into()
    }
}

/// Getters mainly for testing
impl UserTab {
    /// Returns the current user
    pub fn get_current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Returns the user to chat with
    pub fn get_user_to_chat_with(&self) -> Option<&User> {
        self.user_to_chat_with.as_ref()
    }

    /// Returns all users not friends
    pub fn get_all_users_not_friends(&self) -> &Vec<User> {
        &self.all_users_not_friends
    }

    /// Returns the selected user
    pub fn get_selected_user(&self) -> Option<&User> {
        self.selected_user.as_ref()
    }

    /// Returns the friends
    pub fn get_friends(&self) -> &Vec<User> {
        &self.friends
    }

    /// Returns the friend requests
    pub fn get_send_friend_requests(&self) -> &Vec<User> {
        &self.send_friend_request
    }
}

/// Setters methods for testing
impl UserTab {
    /// Pushes a friend to the friends list
    pub fn push_friend(&mut self, user: User) {
        self.friends.push(user);
    }

    /// Pushes a friend request to the friend requests list
    pub fn push_friend_request(&mut self, user: User) {
        self.friend_requests.push(user);
    }

    /// Pushes a user to the all users not friends list
    pub fn push_send_friend_request(&mut self, user: User) {
        self.send_friend_request.push(user);
    }
}
