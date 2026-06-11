use iced::alignment::Horizontal;
use iced::widget::{column, text};
use iced::{Element, Font};
use iced::Length::Fill;
use iced_test::ui;

fn update(_state: &mut (), _message: String) {
    println!("{}", _message)
}

fn view(_state: &()) -> Element<'_, String> {
    column![
        text("Hello World!")
            .font(Font::MONOSPACE)
            .size(30)
            .line_height(1.5)
            .width(Fill)
            .center(),
        ui::Button::new("Play", 150, 50, "Hi", 10.0).view()
    ]
    .width(Fill)
    .height(Fill)
    .spacing(15)
    .align_x(Horizontal::Center)
    .into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}