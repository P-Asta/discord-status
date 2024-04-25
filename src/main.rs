use iced::{
    self,
    widget::{button, column, text, text_input},
    Application, Color, Sandbox, Settings, Theme,
};
use iced_aw::{badge, color_picker};
fn main() {
    <App as Sandbox>::run(Settings::default()).unwrap();
}

struct App;

#[derive(Debug, Clone)]
enum AppEvent {
    Text(String),
    BtnClick,
}

impl Sandbox for App {
    type Message = AppEvent;

    fn new() -> Self {
        Self
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            AppEvent::Text(t) => {
                println!("Text: {}", t)
            }
            AppEvent::BtnClick => {
                println!("Button clicked!");
            }
        }
    }
    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            text_input("client_id", "").on_input(AppEvent::Text),
            text_input("title", "").on_input(AppEvent::Text),
            text_input("description", "").on_input(AppEvent::Text),
            text_input("large_image", "").on_input(AppEvent::Text),
            text_input("large_text", "").on_input(AppEvent::Text),
            text_input("small_image", "").on_input(AppEvent::Text),
            text_input("small_text", "").on_input(AppEvent::Text),
            button("sus").on_press(AppEvent::BtnClick)
        ]
        .into()
    }
}
