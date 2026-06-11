use iced::widget::{button, container, text};
use iced::{Background, Border, Color, Length, Theme};

pub struct Button {
    text: String,
    width: u32,
    height: u32,
    action: String,
    border_radius: f32,
}

impl Button {
    pub fn new(
        text: &str, width: u32, height: u32, action: &str, border_radius: f32,
    ) -> Self {
        Self {
            text: text.to_string(),
            width,
            height,
            action: action.to_string(),
            border_radius,
        }
    }

    pub fn view(&self) -> iced::Element<'static, String> {
        let border_radius = self.border_radius;

        button(
            container(text(self.text.clone()).size(30))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .padding(iced::Padding::new(0.0).bottom(50.0)),
        )
        .style(move |theme, status| eon(theme, status, border_radius))
        .width(self.width)
        .height(self.height)
        .on_press(self.action.clone())
        .into()
    }
}

fn eon(_theme: &Theme, status: button::Status, radius: f32) -> button::Style {
    let base_border = Border {
        color: Color::WHITE,
        width: 3.0,
        radius: radius.into(),
    };

    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb8(13, 100, 136))),
            text_color: Color::WHITE,
            border: base_border,
            ..Default::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb8(13, 50, 136))),
            text_color: Color::BLACK,
            border: base_border,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(Background::Color(Color::from_rgb8(44, 44, 44))),
            text_color: Color::WHITE,
            border: base_border,
            ..Default::default()
        },
    }
}