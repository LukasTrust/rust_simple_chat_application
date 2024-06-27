use std::time::Duration;

use iced::{executor, time, Application, Command, Element, Subscription};

use crate::backend::database::models::User;

use super::{
    group_chat::{self, GroupChat},
    home::{self, Home},
    login::{self, Login},
    register::{self, Register},
    tabs_home::{
        group_tab::GroupTabMessage,
        setting_tab::{self, SettingsTabMessage},
        user_tab::UserTabMessage,
    },
    user_chat::{self, UserChat},
};

/// Define the pages that the application can have
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Home,
    Login,
    Register,
    UserChat,
    GroupChat,
}

/// Define the application struct
#[derive(Clone, Debug)]
pub struct App {
    current_user: Option<User>,
    current_page: Page,
    app_theme: setting_tab::AppTheme,
    login: Login,
    home: Home,
    register: Register,
    user_chat: UserChat,
    group_chat: GroupChat,
}

/// Define the messages that the application can have
#[derive(Debug, Clone)]
pub enum Message {
    SwitchPage(Page),
    LoginMessage(login::LoginMessage),
    HomeMessage(home::HomeMessage),
    RegisterMessage(register::RegisterMessage),
    UserChatMessage(user_chat::UserChatMessage),
    GroupChatMessage(group_chat::GroupChatMessage),
}

/// Implementation of the application
impl App {
    /// Clear the login page
    fn clear_login(&mut self) {
        self.login = Login::default();
    }

    /// Clear the register page
    fn clear_register(&mut self) {
        self.register = Register::default();
    }

    /// Clear the home page
    fn clear_home(&mut self) {
        self.home = Home::default();
    }

    /// Clear the user chat page
    fn clear_user_chat(&mut self) {
        self.user_chat = UserChat::default();
    }

    /// Clear the group chat page
    fn clear_group_chat(&mut self) {
        self.group_chat = GroupChat::default();
    }

    /// Handle the switch page message
    fn handle_switch_page(&mut self, page: Page) {
        self.current_page = page;
    }

    /// Handle the login message
    fn handle_login_message(&mut self, login_message: login::LoginMessage) {
        match login_message {
            login::LoginMessage::NavigateToRegister => {
                self.clear_register();
                self.handle_switch_page(Page::Register)
            }
            login::LoginMessage::SubmitLogin => {
                self.login.update(login_message);
                if self.login.get_found_user().id != -1 {
                    self.clear_home();
                    self.current_user = Some(self.login.get_found_user().clone());
                    self.home
                        .set_current_user(self.current_user.as_ref().unwrap().clone());
                    self.home.update(home::HomeMessage::Tick);
                    self.handle_switch_page(Page::Home)
                }
            }
            _ => {
                self.login.update(login_message);
            }
        }
    }

    /// Handle the home message. Home message can be from the home page, user tab, group tab, or settings tab
    fn handle_home_message(&mut self, home_message: home::HomeMessage) {
        match home_message {
            home::HomeMessage::NavigateToLogin => {
                self.clear_login();
                self.handle_switch_page(Page::Login)
            }
            home::HomeMessage::TabSelected(tab_id) => match tab_id {
                home::TabId::Settings => {
                    self.home.get_settings_tab().set_app_theme(self.app_theme);
                    self.home.update(home::HomeMessage::TabSelected(tab_id));
                }
                _ => {
                    self.home.update(home::HomeMessage::TabSelected(tab_id));
                }
            },
            home::HomeMessage::UserTab(user_tag_message) => match user_tag_message {
                UserTabMessage::ChatWithUser(user) => {
                    self.user_chat
                        .set_properties(self.current_user.as_ref().unwrap().clone(), user);
                    self.user_chat.update(user_chat::UserChatMessage::Tick);
                    self.handle_switch_page(Page::UserChat)
                }
                _ => {
                    self.home.get_user_tab().update(user_tag_message);
                }
            },

            home::HomeMessage::GroupTab(group_tab_message) => match group_tab_message {
                GroupTabMessage::ChatWithGroup(group) => {
                    self.group_chat
                        .set_properties(self.current_user.as_ref().unwrap().clone(), group);
                    self.group_chat.update(group_chat::GroupChatMessage::Tick);
                    self.handle_switch_page(Page::GroupChat);
                }
                _ => {
                    self.home.get_group_tab().update(group_tab_message);
                }
            },

            home::HomeMessage::SettingsTab(settings_tab_message) => match settings_tab_message {
                SettingsTabMessage::DeleteAccount => {
                    self.home.get_settings_tab().update(settings_tab_message);
                    if self.home.get_settings_tab().get_account_deleted() {
                        self.current_user = None;
                        self.clear_login();
                        self.clear_home();
                        self.handle_switch_page(Page::Login)
                    }
                }
                SettingsTabMessage::ChangeTheme(_) => {
                    self.home.get_settings_tab().update(settings_tab_message);
                    self.app_theme = self.home.get_settings_tab().get_app_theme();
                }
                _ => self.home.get_settings_tab().update(settings_tab_message),
            },
            _ => {
                self.home.update(home_message);
            }
        }
    }

    /// Handle the register message
    fn handle_register_message(&mut self, register_message: register::RegisterMessage) {
        match register_message {
            register::RegisterMessage::NavigateToLogin => {
                self.clear_login();
                self.handle_switch_page(Page::Login)
            }
            _ => {
                self.register.update(register_message);
            }
        }
    }

    /// Handle the user chat message
    fn handle_user_chat_message(&mut self, user_chat_message: user_chat::UserChatMessage) {
        match user_chat_message {
            user_chat::UserChatMessage::Back => {
                self.clear_user_chat();
                self.handle_switch_page(Page::Home)
            }
            _ => {
                self.user_chat.update(user_chat_message);
            }
        }
    }

    /// Handle the group chat message
    fn handle_group_chat_message(&mut self, group_chat_message: group_chat::GroupChatMessage) {
        match group_chat_message {
            group_chat::GroupChatMessage::Back => {
                self.clear_group_chat();
                self.handle_switch_page(Page::Home)
            }
            _ => {
                self.group_chat.update(group_chat_message);
            }
        }
    }
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    /// Initialize the application
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Self {
            current_user: None,
            current_page: Page::Login,
            app_theme: setting_tab::AppTheme::Moonfly,
            login: Login::default(),
            home: Home::default(),
            register: Register::default(),
            user_chat: UserChat::default(),
            group_chat: GroupChat::default(),
        };
        (app, Command::none())
    }

    // Set the title of the application
    fn title(&self) -> String {
        "SecSE24 Group08".to_string()
    }

    /// Update the application based on the message
    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::SwitchPage(page) => {
                self.handle_switch_page(page);
                Command::none()
            }
            Message::LoginMessage(login_message) => {
                self.handle_login_message(login_message);
                Command::none()
            }
            Message::HomeMessage(home_message) => {
                self.handle_home_message(home_message);
                Command::none()
            }
            Message::RegisterMessage(register_message) => {
                self.handle_register_message(register_message);
                Command::none()
            }
            Message::UserChatMessage(user_chat_message) => {
                self.handle_user_chat_message(user_chat_message);
                Command::none()
            }
            Message::GroupChatMessage(group_chat_message) => {
                self.handle_group_chat_message(group_chat_message);
                Command::none()
            }
        }
    }

    /// View the application based on the current page
    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Login => self.login.view().map(Message::LoginMessage),
            Page::Home => self.home.view().map(Message::HomeMessage),
            Page::Register => self.register.view().map(Message::RegisterMessage),
            Page::UserChat => self.user_chat.view().map(Message::UserChatMessage),
            Page::GroupChat => self.group_chat.view().map(Message::GroupChatMessage),
        }
    }

    /// Set the theme of the application
    fn theme(&self) -> iced::Theme {
        match self.app_theme {
            setting_tab::AppTheme::Moonfly => iced::Theme::Moonfly,
            setting_tab::AppTheme::Oxocarbon => iced::Theme::Oxocarbon,
            setting_tab::AppTheme::Dracula => iced::Theme::Dracula,
            setting_tab::AppTheme::KanagawaWave => iced::Theme::KanagawaWave,
            setting_tab::AppTheme::Light => iced::Theme::Light,
        }
    }

    /// Set the subscription of the application. The subscription is used to update the application every few seconds based on the current page
    fn subscription(&self) -> iced::Subscription<Message> {
        match self.current_page {
            Page::Home => time::every(Duration::from_secs(10))
                .map(|_| Message::HomeMessage(home::HomeMessage::Tick)),
            Page::UserChat => time::every(Duration::from_secs(5))
                .map(|_| Message::UserChatMessage(user_chat::UserChatMessage::Tick)),
            Page::GroupChat => time::every(Duration::from_secs(5))
                .map(|_| Message::GroupChatMessage(group_chat::GroupChatMessage::Tick)),
            _ => Subscription::none(),
        }
    }
}

/// Getter methods for testing
impl App {
    /// Get the current page
    pub fn get_current_page(&self) -> Page {
        self.current_page.clone()
    }

    /// Get the current user
    pub fn get_current_user(&self) -> Option<User> {
        self.current_user.clone()
    }

    /// Get the home page
    pub fn get_home(&self) -> Home {
        self.home.clone()
    }

    /// Get the app theme
    pub fn get_app_theme(&self) -> setting_tab::AppTheme {
        self.app_theme
    }
}

/// Setter methods for testing
impl App {
    /// Set the current user
    pub fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user.clone());
        self.home.set_current_user(user.clone());
        self.login.set_found_user(user);
    }
}
