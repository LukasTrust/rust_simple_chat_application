use iced::{
    alignment::Horizontal,
    widget::{button, column, horizontal_space, row, text},
    Element,
};
use iced_aw::{widgets::Tabs, TabLabel};

use crate::backend::{database::models::User, entities::user_ops::find_all_user};

use super::tabs_home::{
    group_tab::{GroupTab, GroupTabMessage},
    setting_tab::{SettingTab, SettingsTabMessage},
    user_tab::{UserTab, UserTabMessage},
};

/// Represents the state of the home page
#[derive(Debug, Clone)]
pub struct Home {
    current_user: Option<User>,
    // Sub views
    active_tab: TabId,
    user_tab: UserTab,
    group_tab: GroupTab,
    settings_tab: SettingTab,
}

/// Represents the messages that can be sent to the home page
#[derive(Debug, Clone)]
pub enum HomeMessage {
    // Changeing page
    NavigateToLogin,
    // Tabs
    TabSelected(TabId),
    UserTab(UserTabMessage),
    GroupTab(GroupTabMessage),
    SettingsTab(SettingsTabMessage),
    // Load data
    Tick,
}

/// Represents the tabs of the home page
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TabId {
    User,
    Group,
    Settings,
}

/// Default implementation for the home page
impl Default for Home {
    fn default() -> Self {
        Home {
            current_user: None,
            active_tab: TabId::User,
            user_tab: UserTab::default(),
            group_tab: GroupTab::default(),
            settings_tab: SettingTab::default(),
        }
    }
}

/// Implementation of the home page
impl Home {
    /// Getter method for the current user
    pub fn get_current_user(&self) -> Option<User> {
        self.current_user.clone()
    }

    /// Setter method for the current user
    pub fn set_current_user(&mut self, user: User) {
        self.current_user = Some(user);
    }

    /// Getter method for the active tab
    pub fn get_active_tab(&self) -> TabId {
        self.active_tab.clone()
    }

    /// Getter method for the user tab
    pub fn get_user_tab(&mut self) -> &mut UserTab {
        &mut self.user_tab
    }

    /// Getter method for the group tab
    pub fn get_group_tab(&mut self) -> &mut GroupTab {
        &mut self.group_tab
    }

    /// Getter method for the settings tab
    pub fn get_settings_tab(&mut self) -> &mut SettingTab {
        &mut self.settings_tab
    }

    /// Clears the settings tab
    fn clear_settings_tab(&mut self) {
        self.settings_tab = SettingTab::default();
    }

    /// Loads the data of the home page. It filters the users that are not allowed to be shown, such as the current user and test users
    fn load_data(&mut self) -> Vec<User> {
        let users = find_all_user();

        match users {
            Ok(mut users) => {
                users.retain(|user| user.id != self.current_user.as_ref().unwrap().id);
                users
            }
            Err(_) => Vec::new(),
        }
    }

    /// Sets the properties of the home page
    pub fn update(&mut self, message: HomeMessage) {
        match message {
            // Changeing page
            HomeMessage::NavigateToLogin => {}
            HomeMessage::TabSelected(tab_id) => match tab_id {
                TabId::User => {
                    self.clear_settings_tab();
                    self.user_tab
                        .set_current_user(self.current_user.as_ref().unwrap().clone());
                    self.active_tab = tab_id;
                    self.update(HomeMessage::Tick);
                }

                TabId::Group => {
                    self.clear_settings_tab();
                    self.group_tab
                        .set_current_user(self.current_user.as_ref().unwrap().clone());
                    self.active_tab = tab_id;
                    self.update(HomeMessage::Tick);
                }
                TabId::Settings => {
                    self.settings_tab
                        .set_current_user(self.current_user.as_ref().unwrap().clone());
                    self.active_tab = tab_id
                }
            },
            HomeMessage::UserTab(message) => {
                self.user_tab.update(message);
            }
            HomeMessage::GroupTab(message) => {
                self.group_tab.update(message);
            }
            HomeMessage::SettingsTab(message) => {
                self.settings_tab.update(message);
            }
            // Load data
            HomeMessage::Tick => {
                let users = self.load_data();

                match self.active_tab {
                    TabId::User => {
                        self.user_tab
                            .set_current_user(self.current_user.as_ref().unwrap().clone());
                        self.user_tab.update(UserTabMessage::Tick(users.clone()));
                    }
                    TabId::Group => {
                        self.group_tab
                            .set_current_user(self.current_user.as_ref().unwrap().clone());
                        self.group_tab.update(GroupTabMessage::Tick(users.clone()));
                    }
                    _ => {}
                }
            }
        }
    }

    /// Returns the view of the home page
    pub fn view(&self) -> Element<HomeMessage> {
        let button_width = 100;
        let padding = 10;
        let spacing = 20;

        // Top row
        let welcome_message = text(format!(
            "Welcome {} {}!",
            self.current_user.as_ref().unwrap().first_name,
            self.current_user.as_ref().unwrap().last_name
        ))
        .size(30);

        let logout_button = button(text("Logout").horizontal_alignment(Horizontal::Center))
            .width(button_width)
            .padding(padding)
            .on_press(HomeMessage::NavigateToLogin);

        let top_row = row!(welcome_message, horizontal_space(), logout_button)
            .spacing(spacing)
            .padding(padding);

        let tabs = Tabs::new(HomeMessage::TabSelected)
            .push(
                TabId::User,
                TabLabel::IconText('👤', "User".to_string()),
                self.user_tab.view().map(HomeMessage::UserTab),
            )
            .push(
                TabId::Group,
                TabLabel::IconText('👥', "Group".to_string()),
                self.group_tab.view().map(HomeMessage::GroupTab),
            )
            .push(
                TabId::Settings,
                TabLabel::IconText('⛭', "Settings".to_string()),
                self.settings_tab.view().map(HomeMessage::SettingsTab),
            )
            .set_active_tab(&self.active_tab);

        let content = column![top_row, tabs].spacing(spacing);

        content.into()
    }
}

/// Getter for the current user
impl Home {
    /// Returns the current user
    pub fn active_tab(&self) -> TabId {
        self.active_tab.clone()
    }
}
