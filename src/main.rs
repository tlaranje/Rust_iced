use iced::{Element, Font};
use iced::widget::{button, column, text};

// =========================================================================
// APPROACH 1: THE AUTOMATIC WAY (Active)
// =========================================================================
// The '#[derive(Default)]' attribute tells the Rust compiler to automatically
// generate the 'Default' trait for this struct behind the scenes.
// It will initialize 'value' with its type's standard default, which is 0 for i64.
#[derive(Default)]
struct Counter {
    value: i64,
}

/* // =========================================================================
// APPROACH 2: THE MANUAL WAY (Commented Out)
// =========================================================================
// This approach splits the logic into two steps to gain full control over 
// the initial state (setting it to 42 instead of the automatic 0).

impl Counter {
    // 1. Inherent Implementation: A custom constructor function named 'new'.
    //    It manually creates a Counter with a specific starting value of 42.
    fn new() -> Self {
        Counter { value: 42 }
    }
}

impl Default for Counter {
    // 2. Trait Implementation: Fulfills the 'Default' contract required by iced.
    //    Instead of using Rust's standard 0, it redirects 'Counter::default()' 
    //    to trigger our custom 'Counter::new()' function.
    fn default() -> Self {
        Counter::new()
    }
}
 */

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
            .center(),
        button("+").on_press(Message::Increment),
        text(counter.value),
        button("-").on_press(Message::Decrement),
    ]
    .into()
}

fn main() -> iced::Result {
    // 'iced::run' implicitly requires the state ('Counter') to implement 'Default'.
    // Currently, it uses the generated macro from Approach 1, starting the app at 0.
    // If you swap them, it would use Approach 2 and start the app at 42.
    iced::run(update, view)
}