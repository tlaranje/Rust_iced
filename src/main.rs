use iced::{Element, Fill, Font};
use iced::widget::{button, column, text};

#[derive(Default)]
struct Counter {
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
        button("+").on_press(Message::Increment),
        text(counter.value),
        button("-").on_press(Message::Decrement),
    ]
    .into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}