use iced::{Element, Font};
use iced::widget::{button, column, text, container};
use iced::Length::Fill;
use smart_default::SmartDefault;
use iced::alignment::{Horizontal};
use iced::Length;

#[derive(SmartDefault)]
struct Counter {
    #[default = 42]
    value: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<'_, Message> {
    column![
        text("- Hello there!\n- General Kenobi!")
            .font(Font::MONOSPACE)
            .size(30)
            .line_height(1.5)
            .width(Fill) 
            .center(),

        button(
            container(
                text("Play").size(30)
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        ).width(150).height(50).on_press(Message::Increment),
        text(counter.value).size(24),
        button("Exit").on_press(Message::Decrement).width(150).height(50),
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