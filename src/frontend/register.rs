use iced::{
    alignment::Horizontal,
    widget::{button, column, row, text, text_input, Container},
    Alignment, Color, Element, Length,
};

use crate::backend::{
    database::db::{is_strong_password, is_valid_email},
    entities::user_ops::create_user,
};

/// Represents the state of the registration page
#[derive(Default, Debug, Clone)]
pub struct Register {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    error: String,
    info: String,
}

/// Represents the messages that can be sent to the registration page
#[derive(Debug, Clone)]
pub enum RegisterMessage {
    EmailChanged(String),
    PasswordChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    SubmitRegister,
    NavigateToLogin,
}

/// Implementation of the registration page
impl Register {
    /// Handles the registration process. It checks if the email, password, first name, and last name fields are empty,
    /// validates the email and password, and creates a new user. If the registration fails, it displays an error message.
    fn submit_register(&mut self) {
        if self.email.is_empty()
            || self.password.is_empty()
            || self.first_name.is_empty()
            || self.last_name.is_empty()
        {
            self.error = "Please fill in all fields".to_string();
            self.info = String::new();
        } else if !is_valid_email(&self.email) {
            self.error = "Invalid email format".to_string();
            self.info = String::new();
        } else if !is_strong_password(&self.password) {
            self.error = "Password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one digit, and one special character".to_string();
            self.info = String::new();
        } else {
            let result = create_user(
                &self.first_name,
                &self.last_name,
                &self.email.to_lowercase(),
                &self.password,
            );

            match result {
                Ok(_user) => {
                    self.error = String::new();
                    self.info = "Account has been registered".to_string();
                }
                Err(error) => {
                    self.error = error;
                    self.info = String::new()
                }
            }
        }
    }

    /// Updates the registration page based on the message received
    pub fn update(&mut self, message: RegisterMessage) {
        match message {
            // Handle email change event
            RegisterMessage::EmailChanged(email) => {
                self.email = email;
            }
            // Handle password change event
            RegisterMessage::PasswordChanged(password) => {
                self.password = password;
            }
            // Handle first name change event
            RegisterMessage::FirstNameChanged(first_name) => {
                self.first_name = first_name;
            }
            // Handle last name change event
            RegisterMessage::LastNameChanged(last_name) => {
                self.last_name = last_name;
            }
            // Handle register submission event
            RegisterMessage::SubmitRegister => {
                self.submit_register();
            }
            RegisterMessage::NavigateToLogin => {}
        }
    }

    /// Returns the view of the registration page
    pub fn view(&self) -> Element<RegisterMessage> {
        let input_width = 300;
        let button_width = 100;
        let padding = 10;
        let spacing = 20;

        let registration = text("Registration:").size(30);

        let first_name_input = text_input("First Name", self.first_name.as_str())
            .width(input_width)
            .padding(padding)
            .on_input(RegisterMessage::FirstNameChanged);

        let last_name_input = text_input("Last Name", self.last_name.as_str())
            .width(input_width)
            .padding(padding)
            .on_input(RegisterMessage::LastNameChanged);

        let email_input = text_input("Email", self.email.as_str())
            .width(input_width)
            .padding(padding)
            .on_input(RegisterMessage::EmailChanged);

        let password_input = text_input("Password", self.password.as_str())
            .width(input_width)
            .padding(padding)
            .secure(true)
            .on_input(RegisterMessage::PasswordChanged)
            .on_submit(RegisterMessage::SubmitRegister);

        let register_button = button(text("Register").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(RegisterMessage::SubmitRegister);

        let back_button = button(text("Back").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(RegisterMessage::NavigateToLogin);

        let error_message: Element<RegisterMessage> = if !self.error.is_empty() {
            text(&self.error)
                .size(15)
                .style(Color::from_rgb(1.0, 0.0, 0.0)) // Red color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let info_message: Element<RegisterMessage> = if !self.info.is_empty() {
            text(&self.info)
                .size(15)
                .style(Color::from_rgb(0.2, 0.8, 0.2)) //Green color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let registration_content = column![
            registration,
            first_name_input,
            last_name_input,
            email_input,
            password_input
        ]
        .spacing(spacing)
        .padding(padding);

        let buttons_row = row![register_button, back_button]
            .spacing(spacing)
            .padding(padding);

        let content = column![
            registration_content,
            buttons_row,
            error_message,
            info_message
        ]
        .spacing(spacing)
        .padding(padding)
        .align_items(Alignment::Center);

        let content = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        content.into()
    }
}

/// Getter methods for testing
impl Register {
    /// Gets the first name
    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }

    /// Gets the last name
    pub fn get_last_name(&self) -> &String {
        &self.last_name
    }

    /// Gets the email
    pub fn get_email(&self) -> &String {
        &self.email
    }

    /// Gets the password
    pub fn get_password(&self) -> &String {
        &self.password
    }

    /// Gets the error message
    pub fn get_error(&self) -> &String {
        &self.error
    }

    /// Gets the info message
    pub fn get_info(&self) -> &String {
        &self.info
    }
}

/// Setter methods for testing
impl Register {
    /// Sets the first name
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    /// Sets the last name
    pub fn set_info(&mut self, info: String) {
        self.info = info;
    }
}
