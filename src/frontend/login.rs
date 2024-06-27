use crate::backend::{
    database::models::User,
    entities::user_ops::{find_user_by_email, find_user_with_password_by_id},
};
use bcrypt::verify;
use iced::{
    alignment::Horizontal,
    widget::{button, column, row, text, text_input, Container},
    Alignment, Color, Element, Length,
};
use log::error;

/// Represents the state of the login page
#[derive(Debug, Clone)]
pub struct Login {
    email: String,
    password: String,
    error: String,
    found_user: User,
}

/// Represents the messages that can be sent to the login page
#[derive(Debug, Clone)]
pub enum LoginMessage {
    EmailChanged(String),
    PasswordChanged(String),
    SubmitLogin,
    LoginSuccess,
    NavigateToRegister,
}

/// Default implementation for the Login struct
impl Default for Login {
    fn default() -> Self {
        Login {
            error: String::new(),
            email: String::new(),
            password: String::new(),
            found_user: User {
                id: -1,
                first_name: String::new(),
                last_name: String::new(),
            },
        }
    }
}

/// Implementation of the login page
impl Login {
    /// Getter method for the found user
    pub fn get_found_user(&self) -> &User {
        &self.found_user
    }

    /// Setter method for the found user
    pub fn set_found_user(&mut self, user: User) {
        self.found_user = user;
    }

    /// Handles the login process. It checks if the email and password fields are empty,
    /// finds the user by email, finds the user's password, verifies the password, and sets the found user if the login is successful.
    /// If the login fails, it displays an error message.
    fn handle_login(&mut self) {
        // Check if the email and password fields are empty
        if self.email.is_empty() || self.password.is_empty() {
            self.error = "Please fill in both email and password fields.".to_string();
            return;
        }

        // Find the user by email
        let found_user = find_user_by_email(&self.email.to_lowercase());
        match found_user {
            Ok(user) => {
                // Find the user's password
                let user_id = user.id;
                let found_password = find_user_with_password_by_id(user_id);
                match found_password {
                    Ok(password_data) => {
                        let stored_password = password_data.password;

                        // Verify the password
                        match verify(&self.password, &stored_password) {
                            Ok(verified) => {
                                if verified {
                                    // Set the found user
                                    // Need to return the user to the main app
                                    self.found_user = user;
                                    return;
                                } else {
                                    error!("Password verification failed");
                                }
                            }
                            Err(err) => {
                                error!("Error verifying password: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("Error finding password: {:?}", err);
                    }
                }
            }
            Err(err) => {
                error!("Error finding user: {:?}", err);
            }
        }
        // If the login failed, display an error message
        self.error = "Login failed. Either the email or password was incorrect.".to_string();
    }

    /// Updates the login page based on the message received
    pub fn update(&mut self, message: LoginMessage) {
        match message {
            // Update the email field
            LoginMessage::EmailChanged(email) => {
                self.email = email;
            }
            // Update the password field
            LoginMessage::PasswordChanged(password) => {
                self.password = password;
            }
            // Handle the login submission
            LoginMessage::SubmitLogin => {
                self.handle_login();
            }
            // Is handled in the main app
            LoginMessage::LoginSuccess => {}
            // Is handled in the main app
            LoginMessage::NavigateToRegister => {}
        }
    }

    /// Returns the view of the login page
    pub fn view(&self) -> Element<LoginMessage> {
        let input_width = 300;
        let button_width = 120;
        let padding = 10;
        let spacing: u16 = 20;

        let login = text("Login:").size(30);

        let email_input = text_input("Email:", self.email.as_str())
            .width(input_width)
            .padding(padding)
            .on_input(LoginMessage::EmailChanged);

        let password_input = text_input("Password:", self.password.as_str())
            .width(input_width)
            .padding(padding)
            .secure(true)
            .on_input(LoginMessage::PasswordChanged)
            .on_submit(LoginMessage::SubmitLogin);

        let login_button = button(text("Login").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(LoginMessage::SubmitLogin);

        let register_button = button(text("Register").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(LoginMessage::NavigateToRegister);

        let error_message: Element<LoginMessage> = if !self.error.is_empty() {
            text(&self.error)
                .size(15)
                .style(Color::from_rgb(1.0, 0.0, 0.0)) // Red color
                .into()
        } else {
            text("").size(15).into()
        };

        let login_content = column![login, email_input, password_input]
            .padding(padding)
            .spacing(spacing);

        let button_row = row![login_button, register_button]
            .padding(padding)
            .spacing(spacing);

        let content = column![login_content, button_row, error_message]
            .spacing(20)
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
impl Login {
    /// Getter method for the email field
    pub fn get_email(&self) -> &String {
        &self.email
    }

    /// Getter method for the password field
    pub fn get_password(&self) -> &String {
        &self.password
    }

    /// Getter method for the error message
    pub fn get_error(&self) -> &String {
        &self.error
    }
}
