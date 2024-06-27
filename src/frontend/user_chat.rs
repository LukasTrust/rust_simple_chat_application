use crate::backend::{
    database::{
        db::format_send_date,
        models::{User, UserMessage},
    },
    entities::user_message_ops::{create_user_message, find_all_messages_between_users},
};

use iced::{
    alignment::Horizontal,
    widget::{button, column, horizontal_space, row, text, text_input, Scrollable},
    Element, Length,
};
use log::error;

/// Represents the state of the user chat
#[derive(Debug, Clone)]
pub struct UserChat {
    current_user: User,
    other_user: User,
    messages: Vec<UserMessage>,
    input_value: String,
}

/// Represents the messages that can be sent to the user chat
#[derive(Debug, Clone)]
pub enum UserChatMessage {
    SendMessage,
    InputChanged(String),
    Back,
    Tick,
}

/// Default implementation for the user chat
impl Default for UserChat {
    fn default() -> Self {
        UserChat {
            current_user: User {
                id: -1,
                first_name: String::new(),
                last_name: String::new(),
            },
            other_user: User {
                id: -1,
                first_name: String::new(),
                last_name: String::new(),
            },
            messages: vec![],
            input_value: String::new(),
        }
    }
}

/// Implementation of the user chat
impl UserChat {
    /// Sets the properties of the user chat
    pub fn set_properties(&mut self, current_user: User, other_user: User) {
        self.current_user = current_user;
        self.other_user = other_user;
    }

    /// Sends a user message. If the input value is empty, it returns. Otherwise, it creates a user message and adds it to the messages vector.
    fn send_user_message(&mut self) {
        if self.input_value.is_empty() {
            return;
        }

        let result =
            create_user_message(self.current_user.id, self.other_user.id, &self.input_value);
        match result {
            Ok(user_message) => {
                self.messages.push(user_message);
            }
            Err(e) => {
                error!("Error sending message: {:?}", e);
            }
        }
        self.input_value.clear();
    }

    /// Handles the tick event. It finds all messages between the current user and the other user and updates the messages vector.
    fn handle_tick(&mut self) {
        let all_messages =
            find_all_messages_between_users(self.current_user.id, self.other_user.id);
        match all_messages {
            Ok(messages) => {
                self.messages = messages;
            }
            Err(e) => {
                error!("Error loading messages: {:?}", e);
            }
        }
    }

    /// Updates the user chat based on the message
    pub fn update(&mut self, message: UserChatMessage) {
        match message {
            UserChatMessage::SendMessage => {
                self.send_user_message();
            }
            UserChatMessage::InputChanged(value) => {
                self.input_value = value;
            }
            UserChatMessage::Back => {}
            UserChatMessage::Tick => {
                self.handle_tick();
            }
        }
    }

    /// Returns the view of the user chat
    pub fn view(&self) -> Element<UserChatMessage> {
        let button_width = 100;
        let padding = 10;
        let spacing = 20;
        let text_size = 20;

        let back_button = button(text("Back").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(UserChatMessage::Back);

        let top_row = row!(horizontal_space(), back_button)
            .spacing(spacing)
            .padding(padding);

        let other_user_name = text(format!("[{}]:", self.other_user.first_name)).size(text_size);

        let own_name = text("[You]:").size(text_size);

        let name_row = row!(other_user_name, horizontal_space(), own_name)
            .spacing(spacing)
            .padding(padding);

        let mut message_column = column![].spacing(spacing).padding(padding);

        for message in &self.messages {
            if message.sender_id == self.current_user.id {
                let message_date =
                    text(format!("{}:", format_send_date(message.send_date))).size(text_size);
                let date_row = row!(horizontal_space(), message_date).padding(padding);
                message_column = message_column.push(date_row);

                let message_text = text(message.message.as_str()).size(text_size);

                let message_row = row!(horizontal_space(), message_text).padding(padding);
                message_column = message_column.push(message_row);
            } else {
                let message_date =
                    text(format!("{}:", format_send_date(message.send_date))).size(text_size);
                message_column = message_column.push(message_date);

                let message_text = text(message.message.as_str())
                    .size(20)
                    .horizontal_alignment(Horizontal::Left);
                message_column = message_column.push(message_text);
            }
        }

        let message_scrollable = Scrollable::new(message_column)
            .width(Length::Fill)
            .height(Length::Fill);

        let input_field = text_input("Type your message...", &self.input_value)
            .width(Length::Fill)
            .padding(padding)
            .on_submit(UserChatMessage::SendMessage)
            .on_input(UserChatMessage::InputChanged);

        let send_button = button(text("Send").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(UserChatMessage::SendMessage);

        let bottom_row = row!(input_field, send_button)
            .spacing(spacing)
            .padding(padding);

        let content = column![top_row, name_row, message_scrollable, bottom_row];

        content.into()
    }
}

/// Getter methods for testing
impl UserChat {
    /// Gets the messages
    pub fn get_messages(&self) -> &Vec<UserMessage> {
        &self.messages
    }

    /// Gets the input value
    pub fn get_input_value(&self) -> &String {
        &self.input_value
    }

    /// Gets the current user
    pub fn get_current_user(&self) -> &User {
        &self.current_user
    }

    /// Gets the other user
    pub fn get_other_user(&self) -> &User {
        &self.other_user
    }
}

/// Setter methods for testing
impl UserChat {
    /// Pushes a message to the messages vector
    pub fn push_message(&mut self, message: UserMessage) {
        self.messages.push(message);
    }
}
