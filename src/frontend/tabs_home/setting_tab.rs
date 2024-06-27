use iced::{
    alignment::{self, Horizontal},
    widget::{button, column, horizontal_space, radio, row, text, text_input, Container},
    Alignment, Color, Element, Length,
};

use crate::backend::{
    database::{db::is_valid_email, models::User},
    entities::user_ops::{delete_user, update_email, update_password},
};

/// Represents the setting tab in the home screen
#[derive(Debug, Clone)]
pub struct SettingTab {
    current_user: Option<User>,
    error: String,
    info: String,
    app_theme: AppTheme,
    // Update
    new_email_value: String,
    new_password_value: String,
    current_password_value: String,
    delete_button_pressed: bool,
    account_deleted: bool,
}

/// Represents the messages that can be sent to the settings tab
#[derive(Debug, Clone)]
pub enum SettingsTabMessage {
    UpdateEmail,
    UpdatePassword,
    EmailInputChanged(String),
    NewPasswordInputChanged(String),
    CurrentPasswordInputChanged(String),
    DeleteAccount,
    ChangeTheme(AppTheme),
}

/// The different themes that the application can have
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Moonfly,
    Oxocarbon,
    Dracula,
    KanagawaWave,
    Light,
}

/// The default implementation of the settings tab
impl Default for SettingTab {
    fn default() -> Self {
        SettingTab {
            current_user: None,
            error: String::new(),
            info: String::new(),
            app_theme: AppTheme::Moonfly,
            // Update
            new_password_value: String::new(),
            new_email_value: String::new(),
            current_password_value: String::new(),
            delete_button_pressed: false,
            account_deleted: false,
        }
    }
}

/// Implementation of the setting tab
impl SettingTab {
    /// Set the current user
    pub fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user);
    }

    /// Set the app theme
    pub fn set_app_theme(&mut self, app_theme: AppTheme) {
        self.app_theme = app_theme;
    }

    pub fn get_app_theme(&self) -> AppTheme {
        self.app_theme
    }

    pub fn get_account_deleted(&self) -> bool {
        self.account_deleted
    }

    /// Update the password. The password can only be updated if the fields are not empty, the new password is strong, and the current password is correct
    fn update_password(&mut self) {
        if self.new_password_value.is_empty() || self.current_password_value.is_empty() {
            self.info = String::new();
            self.error = "Password fields cannot be empty".to_string();
            return;
        }

        let result = update_password(
            self.current_user.as_ref().unwrap().id,
            self.current_password_value.as_str(),
            self.new_password_value.as_str(),
        );

        match result {
            Ok(_) => {
                self.error = String::new();
                self.info = "Password updated successfully".to_string();
            }
            Err(e) => {
                self.error = e;
                self.info = String::new();
            }
        }
    }

    /// Update the email. The email can only be updated if the field is not empty and the email is valid
    fn update_email(&mut self) {
        if self.new_email_value.is_empty() {
            self.info = String::new();
            self.error = "Email and password fields cannot be empty".to_string();
            return;
        }

        if !is_valid_email(&self.new_email_value) {
            self.info = String::new();
            self.error = "Invalid email format".to_string();
            return;
        }

        let result = update_email(
            self.current_user.as_ref().unwrap().id,
            self.new_email_value.as_str(),
        );
        match result {
            Ok(_) => {
                self.error = String::new();
                self.info = "Email updated successfully".to_string();
            }
            Err(e) => {
                self.info = String::new();
                self.error = format!("Error updating email: {:?}", e);
            }
        }
    }

    /// Update the settings tab based on the message
    pub fn update(&mut self, message: SettingsTabMessage) {
        match message {
            SettingsTabMessage::ChangeTheme(app_theme) => {
                self.app_theme = app_theme;
            }
            // Upates the email value
            SettingsTabMessage::EmailInputChanged(email) => {
                self.new_email_value = email;
            }
            // Updates the new password value
            SettingsTabMessage::NewPasswordInputChanged(new_password) => {
                self.new_password_value = new_password;
            }
            // Updates the current password value
            SettingsTabMessage::CurrentPasswordInputChanged(current_password) => {
                self.current_password_value = current_password;
            }
            // Deletes the account, after pressing the button twice
            SettingsTabMessage::DeleteAccount => {
                if !self.delete_button_pressed {
                    self.delete_button_pressed = true;
                    return;
                }

                let result = delete_user(self.current_user.as_ref().unwrap().id);
                match result {
                    Ok(_) => {
                        self.account_deleted = true;
                    }
                    Err(e) => {
                        self.info = String::new();
                        self.error = format!("Error deleting account: {:?}", e);
                    }
                }
            }
            // Updates the email
            SettingsTabMessage::UpdateEmail => {
                self.update_email();
            }
            // Updates the password, if the old password is correct and the new password is strong
            SettingsTabMessage::UpdatePassword => {
                self.update_password();
            }
        }
    }

    /// Returns the view of the settings tab
    pub fn view(&self) -> Element<SettingsTabMessage> {
        let input_width = 300;
        let button_width = 150;
        let padding = 10;
        let spacing = 20;

        let theme_moonfly_radio = radio(
            "Moonfly",
            AppTheme::Moonfly,
            Some(self.app_theme),
            SettingsTabMessage::ChangeTheme,
        )
        .spacing(spacing);

        let theme_oxocarbon_radio = radio(
            "Oxocarbon",
            AppTheme::Oxocarbon,
            Some(self.app_theme),
            SettingsTabMessage::ChangeTheme,
        )
        .spacing(spacing);

        let theme_dracula_radio = radio(
            "Dracula",
            AppTheme::Dracula,
            Some(self.app_theme),
            SettingsTabMessage::ChangeTheme,
        )
        .spacing(spacing);

        let theme_kanagawa_wave_radio = radio(
            "KanagawaWave",
            AppTheme::KanagawaWave,
            Some(self.app_theme),
            SettingsTabMessage::ChangeTheme,
        )
        .spacing(spacing);

        let theme_light_radio = radio(
            "Light",
            AppTheme::Light,
            Some(self.app_theme),
            SettingsTabMessage::ChangeTheme,
        )
        .spacing(spacing);

        let top_row = row!(
            theme_moonfly_radio,
            theme_oxocarbon_radio,
            theme_dracula_radio,
            theme_kanagawa_wave_radio,
            theme_light_radio
        )
        .spacing(spacing)
        .padding(padding);

        let current_password_field = text_input("Old Password", &self.current_password_value)
            .width(input_width)
            .padding(padding)
            .secure(true)
            .on_input(SettingsTabMessage::CurrentPasswordInputChanged);

        let new_password_field = text_input("New Password", &self.new_password_value)
            .width(input_width)
            .padding(padding)
            .secure(true)
            .on_input(SettingsTabMessage::NewPasswordInputChanged);

        let update_password_button =
            button(text("Update Password").horizontal_alignment(alignment::Horizontal::Center))
                .width(button_width)
                .padding(padding)
                .on_press(SettingsTabMessage::UpdatePassword);

        let update_password_row = row!(
            horizontal_space(),
            current_password_field,
            new_password_field,
            update_password_button
        )
        .spacing(spacing)
        .padding(padding);

        let new_email_field = text_input("Email", &self.new_email_value)
            .width(input_width)
            .padding(padding)
            .on_input(SettingsTabMessage::EmailInputChanged);

        let update_email_button =
            button(text("Update Email").horizontal_alignment(alignment::Horizontal::Center))
                .width(button_width)
                .padding(padding)
                .on_press(SettingsTabMessage::UpdateEmail);

        let update_email_row = row!(horizontal_space(), new_email_field, update_email_button)
            .spacing(spacing)
            .padding(padding);

        let delete_button =
            button(text("Delete Account").horizontal_alignment(alignment::Horizontal::Center))
                .width(button_width)
                .padding(padding)
                .on_press(SettingsTabMessage::DeleteAccount);

        let error_message: Element<SettingsTabMessage> = if !self.error.is_empty() {
            text(&self.error)
                .size(15)
                .style(Color::from_rgb(1.0, 0.0, 0.0)) // Red color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let info_message: Element<SettingsTabMessage> = if !self.info.is_empty() {
            text(&self.info)
                .size(15)
                .style(Color::from_rgb(0.2, 0.8, 0.2)) //Green color
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let confirmation_dialog: Element<SettingsTabMessage> = if self.delete_button_pressed {
            text("Are you sure you want to delete your account?".to_string())
                .size(15)
                .style(Color::from_rgb(1.0, 0.0, 0.0))
                .horizontal_alignment(Horizontal::Center)
                .into()
        } else {
            text("").into()
        };

        let message_column = column![error_message, info_message, confirmation_dialog]
            .spacing(spacing)
            .padding(padding)
            .align_items(Alignment::Center);

        let delete_row = row!(horizontal_space(), delete_button)
            .spacing(spacing)
            .padding(padding);

        let content = column![
            top_row,
            update_password_row,
            update_email_row,
            delete_row,
            message_column
        ]
        .spacing(spacing)
        .align_items(Alignment::Center);

        let content = Container::new(content)
            .width(Length::Fill)
            .center_x()
            .center_y();

        content.into()
    }
}

/// Geter methods for tests
impl SettingTab {
    /// Getter method for the current user
    pub fn get_current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Getter method for the error message
    pub fn get_error(&self) -> &str {
        &self.error
    }

    /// Getter method for the info message
    pub fn get_info(&self) -> &str {
        &self.info
    }

    /// Getter method for the new password value
    pub fn get_new_password_value(&self) -> &str {
        &self.new_password_value
    }

    /// Getter method for the new email value
    pub fn get_new_email_value(&self) -> &str {
        &self.new_email_value
    }

    /// Getter method for the current password value
    pub fn get_current_password_value(&self) -> &str {
        &self.current_password_value
    }

    /// Getter method for the delete button pressed
    pub fn get_delete_button_pressed(&self) -> bool {
        self.delete_button_pressed
    }
}

/// Setter methods for tests
impl SettingTab {
    /// Setter method for the current password value
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    /// Setter method for the info message
    pub fn set_info(&mut self, info: String) {
        self.info = info;
    }

    /// Setter method for the new password value
    pub fn set_delete_button_pressed(&mut self, delete_button_pressed: bool) {
        self.delete_button_pressed = delete_button_pressed;
    }
}
