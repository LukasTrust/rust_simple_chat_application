use iced::{window, Application, Settings, Size};
use secse24_group08::frontend::app::App;

/// Runs the application
fn main() -> iced::Result {
    // Initializes the logger
    env_logger::init();

    let data = include_bytes!("icon.png");

    let icon = window::icon::from_file_data(data, None);

    assert!(icon.is_ok());

    let icon = icon.unwrap();

    let settings = Settings::<()> {
        window: window::Settings {
            size: Size::new(1600.0, 900.0),
            position: window::Position::Centered,
            icon: Some(icon),
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    // Runs the application
    App::run(settings)
}
