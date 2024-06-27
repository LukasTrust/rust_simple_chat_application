use crate::backend::{
    database::{
        db::format_send_date,
        models::{Group, GroupMessage, User},
    },
    entities::{
        group_message_ops::{create_group_message, find_all_messages_of_group},
        user_ops::find_users_by_ids,
    },
};

use iced::{
    alignment::Horizontal,
    widget::{button, column, horizontal_space, row, text, text_input, Scrollable},
    Element, Length,
};

use log::error;

/// Represents the state of the group chat
#[derive(Debug, Clone)]
pub struct GroupChat {
    current_user: User,
    current_group: Group,
    messages: Vec<GroupMessage>,
    users_of_group: Vec<User>,
    input_value: String,
}

/// Represents the messages that can be sent to the group chat
#[derive(Debug, Clone)]
pub enum GroupChatMessage {
    SendMessage,
    InputChanged(String),
    Back,
    Tick,
}

/// Default implementation for the group chat
impl Default for GroupChat {
    fn default() -> Self {
        GroupChat {
            current_user: User {
                id: -1,
                first_name: String::new(),
                last_name: String::new(),
            },
            current_group: Group {
                id: -1,
                name: String::new(),
            },
            users_of_group: vec![],
            messages: vec![],
            input_value: String::new(),
        }
    }
}

/// Implementation of the group chat
impl GroupChat {
    /// Sets the properties of the group chat
    pub fn set_properties(&mut self, current_user: User, current_group: Group) {
        self.current_user = current_user;
        self.current_group = current_group;
    }

    /// Clears the data of the group chat
    fn clear_data(&mut self) {
        self.users_of_group = vec![];
        self.messages = vec![];
    }

    /// Sends a message to the group. If the message is empty, it does nothing
    fn send_group_message(&mut self) {
        if self.input_value.is_empty() {
            return;
        }

        let result = create_group_message(
            self.current_user.id,
            self.current_group.id,
            self.input_value.as_str(),
        );

        match result {
            Ok(message) => {
                self.input_value = String::new();
                self.messages.push(message);
            }
            Err(e) => {
                error!("Error sending message: {}", e);
            }
        }
    }

    /// Handles the tick event. Fetches all messages of the group and the users of the group
    fn handle_tick(&mut self) {
        self.clear_data();

        let result = find_all_messages_of_group(self.current_group.id);

        match result {
            Ok(messages) => {
                self.messages = messages;
                let user_ids: Vec<i32> = self
                    .messages
                    .iter()
                    .map(|message| message.sender_id)
                    .collect();

                let result = find_users_by_ids(user_ids);

                match result {
                    Ok(users) => {
                        self.users_of_group = users;
                    }
                    Err(e) => {
                        error!("Error fetching users: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error fetching messages: {}", e);
            }
        }
    }

    /// Updates the group chat based on the message.
    pub fn update(&mut self, message: GroupChatMessage) {
        match message {
            GroupChatMessage::SendMessage => {
                self.send_group_message();
            }
            GroupChatMessage::InputChanged(value) => {
                self.input_value = value;
            }
            GroupChatMessage::Back => {}
            GroupChatMessage::Tick => {
                self.handle_tick();
            }
        }
    }

    /// Returns the view of the group chat
    pub fn view(&self) -> Element<GroupChatMessage> {
        let button_width = 100;
        let padding = 10;
        let spacing = 20;
        let text_size = 20;

        let group_name = text(format!("[{}]:", self.current_group.name)).size(text_size);

        let back_button = button(text("Back").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(GroupChatMessage::Back);

        let top_row = row!(group_name, horizontal_space(), back_button)
            .spacing(spacing)
            .padding(padding);

        let other_users = text("[Users]:").size(text_size);

        let own_name = text("[You]:").size(text_size);

        let name_row = row!(other_users, horizontal_space(), own_name)
            .spacing(spacing)
            .padding(padding);

        let mut message_column = column![].spacing(spacing).padding(padding);

        for message in &self.messages {
            if message.sender_id == self.current_user.id {
                let message_date =
                    text(format!("{}:", format_send_date(message.send_date))).size(text_size);

                let date_row = row!(horizontal_space(), message_date);
                message_column = message_column.push(date_row);

                let message_text = text(message.message.as_str()).size(text_size);

                let message_row = row!(horizontal_space(), message_text);
                message_column = message_column.push(message_row);
            } else {
                let user = self
                    .users_of_group
                    .iter()
                    .find(|user| user.id == message.sender_id)
                    .unwrap();

                let message_date = text(format!(
                    "{} {}, {}:",
                    user.first_name,
                    user.last_name,
                    format_send_date(message.send_date)
                ))
                .size(text_size);

                message_column = message_column.push(message_date);

                let message_text = text(message.message.as_str()).size(text_size);

                message_column = message_column.push(message_text);
            }
        }

        let message_scrollable = Scrollable::new(message_column)
            .width(Length::Fill)
            .height(Length::Fill);

        let input_field = text_input("Type your message...", &self.input_value)
            .width(Length::Fill)
            .padding(padding)
            .on_submit(GroupChatMessage::SendMessage)
            .on_input(GroupChatMessage::InputChanged);

        let send_button = button(text("Send").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(GroupChatMessage::SendMessage);

        let bottom_row = row!(input_field, send_button)
            .spacing(spacing)
            .padding(padding);

        let content = column![top_row, name_row, message_scrollable, bottom_row];

        content.into()
    }
}

/// Getter methods for testing
impl GroupChat {
    /// Getter for the current user
    pub fn get_current_user(&self) -> &User {
        &self.current_user
    }

    /// Getter for the current group
    pub fn get_current_group(&self) -> &Group {
        &self.current_group
    }

    /// Getter for the messages
    pub fn get_messages(&self) -> &Vec<GroupMessage> {
        &self.messages
    }

    /// Getter for the users of the group
    pub fn get_users_of_group(&self) -> &Vec<User> {
        &self.users_of_group
    }

    /// Getter for the input value
    pub fn get_input_value(&self) -> &String {
        &self.input_value
    }
}

/// Setters methods for testing
impl GroupChat {
    /// Setter for the current user
    pub fn push_message(&mut self, message: GroupMessage) {
        self.messages.push(message);
    }

    /// Setter for the users of the group
    pub fn push_users_of_group(&mut self, user: User) {
        self.users_of_group.push(user);
    }
}
