use iced::widget::{button, text, container};
use iced::Length;

#[allow(dead_code)]
pub struct Button {
    text: String,
    width: u32,
    height: u32,
    action: String
}

impl Button {
    pub fn new(text: &str, width: u32, height: u32, action: &str) -> Self {
        Self {
            text: text.to_string(),
            width, height,
            action: action.to_string()
        }
    }

    pub fn view(&self) -> iced::Element<'static, String>
    {
        button(
            container(text(self.text).size(30))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        )
        .width(Length::Fixed(self.width as f32))
        .height(Length::Fixed(self.height as f32))
        .on_press(self.action)
        .into()
    }
}