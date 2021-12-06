use crate::parser::evaluations::calculate_with_vars;
use crate::parser::symbolic::{num_to_letter_vec, shunting_yard};
use crate::{ComplexApp, Message};
use iced::{
    button, scrollable, text_input, Button, Checkbox, Column, Container,
    Element, Length, Row, Scrollable, Text, TextInput,
};
use num_complex::Complex;
use std::collections::HashMap;

#[derive(Default)]
pub struct CalculatorState {
    // Variables
    is_polar: bool,
    real_input: String,
    real_input_state: text_input::State,
    i_input: String,
    i_input_state: text_input::State,
    var_button: button::State,
    var_counter: i32,
    var_scroll: scrollable::State,
    variables: HashMap<String, (i32, Complex<f32>)>,
    // Calculations
    calc_input: String,
    calc_input_state: text_input::State,
    calc_button: button::State,
    calculation_scroll: scrollable::State,
    calculations: Vec<(String, Complex<f32>)>,
    // Delete variable
    delete_input: String,
    delete_input_state: text_input::State,
    delete_button: button::State,
    // Checkbox
    save_calcs: bool,
    // Clearing
    clear_vars_button: button::State,
    clear_calcs_button: button::State,
}

#[derive(Debug, Clone)]
pub enum CalcMessage {
    TogglePolar(bool),
    ChangeCalcInput(String),
    ChangeRealInput(String),
    ChangeImaginaryInput(String),
    ChangeDeleteInput(String),
    SaveCalculations(bool),
    DeleteVariable(String),
    Calculate,
    Save,
    ClearVars,
    ClearCalcs,
}

pub fn render_calculator(app: &mut ComplexApp) -> Element<Message> {
    let mut hash_vec: Vec<(&String, &(i32, Complex<f32>))> =
        app.calculator.variables.iter().collect();
    hash_vec.sort_by(|(_, (a, _)), (_, (b, _))| b.cmp(a));

    let mut variables: Column<Message> = Column::new();
    for (k, (_, v)) in hash_vec.iter() {
        variables = variables.push(Text::new(format!(
            "{}:  {} - polar {:?}",
            k,
            v,
            v.to_polar()
        )));
    }

    let mut calculations: Column<Message> = Column::new();

    for (left, right) in app.calculator.calculations.iter() {
        calculations =
            calculations.push(Text::new(format!("{} = {}", left, right)));
    }

    let real = if app.calculator.is_polar { "r" } else { "x" };
    let imaginary = if app.calculator.is_polar {
        "theta"
    } else {
        "y"
    };
    let addition = if app.calculator.is_polar {
        "* e ^ i "
    } else {
        "+ i "
    };

    let row: Element<_> = Row::new()
        .push(
            Column::new()
                .padding(20)
                .width(Length::Fill)
                .push(
                    Container::new(
                        Column::new()
                            .push(Text::new("Calculations"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.calculator.calc_input_state,
                                        "Enter vars",
                                        &app.calculator.calc_input,
                                        |v| {
                                            Message::Calculator(
                                                CalcMessage::ChangeCalcInput(
                                                    String::from(v),
                                                ),
                                            )
                                        },
                                    ))
                                    .push(
                                        Button::new(
                                            &mut app.calculator.calc_button,
                                            Text::new("Enter"),
                                        )
                                        .on_press(Message::Calculator(
                                            CalcMessage::Calculate,
                                        )),
                                    ),
                            )
                            .push(
                                Scrollable::new(
                                    &mut app.calculator.calculation_scroll,
                                )
                                .push(calculations),
                            ),
                    )
                    .height(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Checkbox::new(
                        app.calculator.save_calcs,
                        "Save calculations to variables",
                        |checked| {
                            Message::Calculator(CalcMessage::SaveCalculations(
                                checked,
                            ))
                        },
                    ))
                    .height(Length::from(32)),
                )
                .push(
                    Container::new(
                        Row::new()
                            .push(
                                Button::new(
                                    &mut app.calculator.clear_vars_button,
                                    Text::new("Clear variables"),
                                )
                                .on_press(
                                    Message::Calculator(CalcMessage::ClearVars),
                                ),
                            )
                            .push(
                                Button::new(
                                    &mut app.calculator.clear_calcs_button,
                                    Text::new("Clear calculations"),
                                )
                                .on_press(
                                    Message::Calculator(
                                        CalcMessage::ClearCalcs,
                                    ),
                                ),
                            ),
                    )
                    .height(Length::from(32)),
                ),
        )
        .push(
            Column::new()
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .push(Text::new("Variables"))
                .push(Container::new(Text::new("")).height(Length::from(16)))
                .push(Checkbox::new(
                    app.calculator.is_polar,
                    "Is polar",
                    |checked| {
                        Message::Calculator(CalcMessage::TogglePolar(checked))
                    },
                ))
                .push(Container::new(Text::new("")).height(Length::from(16)))
                .push(
                    Row::new()
                        .push(TextInput::new(
                            &mut app.calculator.real_input_state,
                            real,
                            &app.calculator.real_input,
                            |v| {
                                Message::Calculator(
                                    CalcMessage::ChangeRealInput(v),
                                )
                            },
                        ))
                        .push(Text::new(addition))
                        .push(TextInput::new(
                            &mut app.calculator.i_input_state,
                            imaginary,
                            &app.calculator.i_input,
                            |v| {
                                Message::Calculator(
                                    CalcMessage::ChangeImaginaryInput(v),
                                )
                            },
                        ))
                        .push(
                            Button::new(
                                &mut app.calculator.var_button,
                                Text::new("Save"),
                            )
                            .on_press(Message::Calculator(CalcMessage::Save)),
                        ),
                )
                .push(
                    Scrollable::new(&mut app.calculator.var_scroll)
                        .push(variables)
                        .height(Length::FillPortion(1)),
                )
                .push(
                    Row::new()
                        .push(TextInput::new(
                            &mut app.calculator.delete_input_state,
                            "Delete variables - (a, b, ...)",
                            &app.calculator.delete_input,
                            |v| {
                                Message::Calculator(
                                    CalcMessage::ChangeDeleteInput(v),
                                )
                            },
                        ))
                        .push(
                            Button::new(
                                &mut app.calculator.delete_button,
                                Text::new("Delete"),
                            )
                            .on_press(
                                Message::Calculator(
                                    CalcMessage::DeleteVariable(
                                        app.calculator.delete_input.clone(),
                                    ),
                                ),
                            ),
                        ),
                ),
        )
        .into();
    Container::new(row).height(Length::Fill).into()
}

pub fn process_calculator_message(app: &mut ComplexApp, message: CalcMessage) {
    match message {
        CalcMessage::SaveCalculations(b) => app.calculator.save_calcs = b,
        CalcMessage::TogglePolar(b) => app.calculator.is_polar = b,
        CalcMessage::ChangeRealInput(v) => {
            app.calculator.real_input = v;
        }
        CalcMessage::ChangeImaginaryInput(v) => {
            app.calculator.i_input = v;
        }
        CalcMessage::ChangeCalcInput(v) => {
            app.calculator.calc_input = v;
        }

        CalcMessage::ChangeDeleteInput(v) => {
            app.calculator.delete_input = v;
        }
        CalcMessage::DeleteVariable(var) => {
            let var = var.replace(" ", "");
            let var: Vec<&str> = var.split(",").collect();
            for v in var.iter() {
                app.calculator.variables.remove(&v.to_string());
            }
            app.calculator.delete_input = "".to_string();
        }
        CalcMessage::Save => {
            let r = app.calculator.real_input.clone().parse::<f32>();
            let i = app.calculator.i_input.clone().parse::<f32>();
            let real;
            match r {
                Ok(v) => real = v,
                Err(_) => real = 0.0,
            }
            let im;
            match i {
                Ok(v) => im = v,
                Err(_) => im = 0.0,
            }
            let res = if app.calculator.is_polar {
                Complex::from_polar(real, im)
            } else {
                Complex::new(real, im)
            };
            let n = app.calculator.var_counter;
            let id = num_to_letter_vec(n as usize).join("");
            app.calculator.variables.insert(id, (n, res));
            app.calculator.var_counter += 1;
            app.calculator.real_input = "".to_string();
            app.calculator.i_input = "".to_string();
        }
        CalcMessage::Calculate => {
            let input = app.calculator.calc_input.replace(" ", "");
            let algorithm = shunting_yard(input.clone());
            let z = calculate_with_vars(algorithm, &app.calculator.variables);

            if let Some(z) = z {
                app.calculator
                    .calculations
                    .insert(0, (app.calculator.calc_input.clone(), z));
                if app.calculator.save_calcs {
                    let n = app.calculator.var_counter;
                    let id = num_to_letter_vec(n as usize).join("");
                    app.calculator.variables.insert(id, (n, z));
                    app.calculator.var_counter += 1;
                }
            }
            app.calculator.calc_input = "".to_string();
        }
        CalcMessage::ClearCalcs => app.calculator.calculations = Vec::new(),
        CalcMessage::ClearVars => {
            app.calculator.variables = HashMap::new();
            app.calculator.var_counter = 0;
        }
    };
}
