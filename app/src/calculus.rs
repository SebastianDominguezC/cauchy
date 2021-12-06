use crate::{ComplexApp, Message};
use iced::{Container, Element, Length, Text};

#[derive(Default)]
pub struct CalculusState {}

#[derive(Debug, Clone)]
pub enum CalculusMessage {
    M,
}

pub fn render_calculus(app: &mut ComplexApp) -> Element<Message> {
    Container::new(Text::new("Calculus"))
        .height(Length::Fill)
        .into()
}

pub fn process_calculus_message(
    app: &mut ComplexApp,
    message: CalculusMessage,
) {
    match message {
        CalculusMessage::M => {}
    };
}
