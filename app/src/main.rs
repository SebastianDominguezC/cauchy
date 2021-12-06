mod calculator;
mod calculus;
mod grapher;
mod parser;

use calculator::{
    process_calculator_message, render_calculator, CalcMessage, CalculatorState,
};
use calculus::{
    process_calculus_message, render_calculus, CalculusMessage, CalculusState,
};
use grapher::{
    process_grapher_message, render_grapher, GrapherMessage, GrapherState,
};
use iced::{
    executor, menu, Application, Clipboard, Command, Element, Menu, Settings,
};
use iced_native::keyboard::{Hotkey, KeyCode, Modifiers};
use parser::symbolic::test_ops;

pub fn main() -> iced::Result {
    test_ops();
    ComplexApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
pub struct ComplexApp {
    pub calculator: CalculatorState,
    pub grapher: GrapherState,
    pub calculus: CalculusState,
    window: Window,
}

#[derive(Debug, Clone)]
pub enum Message {
    Calculator(CalcMessage),
    Grapher(GrapherMessage),
    Calculus(CalculusMessage),
    Menu(Window),
}

#[derive(Debug, Clone)]
pub enum Window {
    Calculator,
    Grapher,
    Calculus,
}

impl Default for Window {
    fn default() -> Window {
        Window::Calculator
    }
}

impl Application for ComplexApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (ComplexApp, Command<Message>) {
        (ComplexApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Complex Number App")
    }

    fn menu(&self) -> Menu<Message> {
        let alt = Modifiers::ALT;
        // let ctrl_shift = Modifiers::CTRL | Modifiers::SHIFT;

        Menu::with_entries(vec![
            menu::Entry::dropdown(
                "View",
                Menu::with_entries(vec![
                    menu::Entry::item(
                        "Calculator",
                        Hotkey::new(alt, KeyCode::F1),
                        Message::Menu(Window::Calculator),
                    ),
                    menu::Entry::item(
                        "Grapher",
                        Hotkey::new(alt, KeyCode::F3),
                        Message::Menu(Window::Grapher),
                    ),
                    menu::Entry::item(
                        "Calculus",
                        Hotkey::new(alt, KeyCode::F4),
                        Message::Menu(Window::Calculus),
                    ),
                ]),
            ),
            menu::Entry::dropdown("Options", Menu::with_entries(vec![])),
        ])
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match message {
            Message::Calculator(m) => {
                process_calculator_message(self, m);
            }
            Message::Grapher(m) => {
                process_grapher_message(self, m);
            }
            Message::Calculus(m) => {
                process_calculus_message(self, m);
            }
            Message::Menu(v) => {
                self.window = v;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.window {
            Window::Calculator => render_calculator(self),
            Window::Grapher => render_grapher(self),
            Window::Calculus => render_calculus(self),
        }
    }
}

// TODO
// Error messages
// - Global widget for popup error messages
//
// CODE
// Refactor and restructure - MUST
//
// UI
// - clean up - MUST
// - re-design - MUST
//
// TESTS
// - automatic tests for parsers - MUST
//
// PARSER / INPUT / CALCULATIONS
// - Polar calculations
//
// NEW PAGES
// Grapher
// - Polar grid
// - Polar inputs
// - Moving grid (with input or with gestures)
// - Evaluation plane vs Function plane, by colors with movile points
//   Evaulation plane defined by regions or functions, which are graphed in the function plane
// - U(x,y) + i V(x,y) inputs - MUST
//
// Calculus
// - integral
// - differential
//
// Series
// - Auto sums
