use crate::parser::evaluations::{evaulate_points, evaulate_points_on_contour};
use crate::parser::symbolic::shunting_yard;
use crate::{ComplexApp, Message};
use iced::{
    button,
    canvas::{self, Canvas, Cursor, Geometry, Path, Stroke, Text as CText},
    mouse, pick_list, text_input, Align, Button, Color, Column, Container,
    Element, HorizontalAlignment, Length, PickList, Point, Rectangle, Row,
    Text, TextInput, VerticalAlignment,
};
use num_complex::Complex;
use std::default::Default;

#[derive(Default)]
pub struct GrapherState {
    // Graph
    graph: GraphState,
    divisions_state: text_input::State,
    divisions_input: String,
    divisions_button: button::State,
    // X interval
    interval_state_left_x: text_input::State,
    interval_input_left_x: String,
    interval_state_right_x: text_input::State,
    interval_input_right_x: String,
    interval_equality_left_x: pick_list::State<LeftEqualities>,
    interval_equality_right_x: pick_list::State<LeftEqualities>,
    selected_equality_left_x: Option<LeftEqualities>,
    selected_equality_right_x: Option<LeftEqualities>,
    precision_state_x: text_input::State,
    precision_input_x: String,
    // Y interval
    interval_state_left_y: text_input::State,
    interval_input_left_y: String,
    interval_state_right_y: text_input::State,
    interval_input_right_y: String,
    interval_equality_left_y: pick_list::State<LeftEqualities>,
    interval_equality_right_y: pick_list::State<LeftEqualities>,
    selected_equality_left_y: Option<LeftEqualities>,
    selected_equality_right_y: Option<LeftEqualities>,
    precision_state_y: text_input::State,
    precision_input_y: String,
    // Evaluation input function
    contour_input_state: text_input::State,
    contour_input: String,
    clear_contour_button: button::State,
    // Function Input
    function_input_state: text_input::State,
    function_input: String,
    function_button: button::State,
    // Radius Input
    radius_input_state: text_input::State,
    radius_input: String,
    radius_button: button::State,
    // Clear
    clear_all_button: button::State,
    // Function options
    pick_list: pick_list::State<FOptions>,
}

#[derive(Debug, Clone)]
pub enum GrapherMessage {
    ChangeIntervalLeftX(String),
    ChangeIntervalRightX(String),
    ChangeIntervalLeftY(String),
    ChangeIntervalRightY(String),
    ChangeFunctionInput(String),
    ChangePrecisionX(String),
    ChangePrecisionY(String),
    ChangeDivisions(String),
    ChangeRadius(String),
    ChangeEvaluationFunctionInput(String),
    UpdateDivisions,
    UpdateRadius,
    GraphFunction,
    ClearAll,
    ClearEvaluationFunction,
    OptionSelected(FOptions),
    EqualitySelectedLeftX(LeftEqualities),
    EqualitySelectedRightX(LeftEqualities),
    EqualitySelectedLeftY(LeftEqualities),
    EqualitySelectedRightY(LeftEqualities),
}

pub fn render_grapher<'a>(app: &'a mut ComplexApp) -> Element<Message> {
    let dimension = Length::Fill;

    let mut functions: Column<Message> = Column::new();
    for z in app.grapher.graph.functions.iter() {
        functions = functions.push(Text::new(format!("F: {:?}", z.operation)));
    }

    let pick_list = PickList::new(
        &mut app.grapher.pick_list,
        &FOptions::ALL[..],
        app.grapher.graph.selected_option,
        |o| Message::Grapher(GrapherMessage::OptionSelected(o)),
    )
    .placeholder("Function colors");

    let x_left_interval_list = PickList::new(
        &mut app.grapher.interval_equality_left_x,
        &LeftEqualities::ALL[..],
        app.grapher.selected_equality_left_x,
        |o| Message::Grapher(GrapherMessage::EqualitySelectedLeftX(o)),
    )
    .placeholder("<");
    let x_right_interval_list = PickList::new(
        &mut app.grapher.interval_equality_right_x,
        &LeftEqualities::ALL[..],
        app.grapher.selected_equality_right_x,
        |o| Message::Grapher(GrapherMessage::EqualitySelectedRightX(o)),
    )
    .placeholder("<");
    let y_left_interval_list = PickList::new(
        &mut app.grapher.interval_equality_left_y,
        &LeftEqualities::ALL[..],
        app.grapher.selected_equality_left_y,
        |o| Message::Grapher(GrapherMessage::EqualitySelectedLeftY(o)),
    )
    .placeholder("<");
    let y_right_interval_list = PickList::new(
        &mut app.grapher.interval_equality_right_y,
        &LeftEqualities::ALL[..],
        app.grapher.selected_equality_right_y,
        |o| Message::Grapher(GrapherMessage::EqualitySelectedRightY(o)),
    )
    .placeholder("<");

    Container::new(
        Column::new()
            .push(Text::new("Grapher"))
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .push(Text::new("Interval Selection"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.interval_state_left_x,
                                        "0",
                                        &mut app.grapher.interval_input_left_x,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeIntervalLeftX(v),
                                    )
                                        },
                                    ))
                                    .push(x_left_interval_list)
                                    .push(Text::new("x"))
                                    .push(x_right_interval_list)
                                    .push(TextInput::new(
                                        &mut app.grapher.interval_state_right_x,
                                        "10",
                                        &mut app.grapher.interval_input_right_x,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeIntervalRightX(v),
                                    )
                                        },
                                    )),
                            )
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.interval_state_left_y,
                                        "0",
                                        &mut app.grapher.interval_input_left_y,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeIntervalLeftY(v),
                                    )
                                        },
                                    ))
                                    .push(y_left_interval_list)
                                    .push(Text::new("y"))
                                    .push(y_right_interval_list)
                                    .push(TextInput::new(
                                        &mut app.grapher.interval_state_right_y,
                                        "10",
                                        &mut app.grapher.interval_input_right_y,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeIntervalRightY(v),
                                    )
                                        },
                                    )),
                            )
                            .push(Text::new("Precision (x, y)"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.precision_state_x,
                                        "1",
                                        &mut app.grapher.precision_input_x,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangePrecisionX(v),
                                    )
                                        },
                                    ))
                                    .push(TextInput::new(
                                        &mut app.grapher.precision_state_y,
                                        "1",
                                        &mut app.grapher.precision_input_y,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangePrecisionY(v),
                                    )
                                        },
                                    )),
                            )
                            .push(Text::new("Zoom Level"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.divisions_state,
                                        "10",
                                        &mut app.grapher.divisions_input,
                                        |v| {
                                            Message::Grapher(
                                                GrapherMessage::ChangeDivisions(
                                                    v,
                                                ),
                                            )
                                        },
                                    ))
                                    .push(
                                        Button::new(
                                            &mut app.grapher.divisions_button,
                                            Text::new("Update zoom"),
                                        )
                                        .on_press(Message::Grapher(
                                            GrapherMessage::UpdateDivisions,
                                        )),
                                    ),
                            )
                            .push(Text::new("Point size"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.radius_input_state,
                                        "1.0",
                                        &mut app.grapher.radius_input,
                                        |v| {
                                            Message::Grapher(
                                                GrapherMessage::ChangeRadius(v),
                                            )
                                        },
                                    ))
                                    .push(
                                        Button::new(
                                            &mut app.grapher.radius_button,
                                            Text::new("Update radii"),
                                        )
                                        .on_press(Message::Grapher(
                                            GrapherMessage::UpdateRadius,
                                        )),
                                    ),
                            )
                            .push(Text::new("Insert function"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.function_input_state,
                                        "Insert f(z)",
                                        &mut app.grapher.function_input,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeFunctionInput(v),
                                    )
                                        },
                                    ))
                                    .push(
                                        Button::new(
                                            &mut app.grapher.function_button,
                                            Text::new("Insert"),
                                        )
                                        .on_press(Message::Grapher(
                                            GrapherMessage::GraphFunction,
                                        )),
                                    ),
                            )
                            .push(Text::new("Evaluation function (?)"))
                            .push(
                                Row::new()
                                    .push(TextInput::new(
                                        &mut app.grapher.contour_input_state,
                                        "Add a f(x)",
                                        &mut app.grapher.contour_input,
                                        |v| {
                                            Message::Grapher(
                                        GrapherMessage::ChangeEvaluationFunctionInput(v),
                                    )
                                        },
                                    ))
                                    .push(
                                        Button::new(
                                            &mut app.grapher.clear_contour_button,
                                            Text::new("Clear"),
                                        )
                                        .on_press(Message::Grapher(
                                            GrapherMessage::ClearEvaluationFunction,
                                        )),
                                    ),
                            )
                            .push(functions.height(Length::FillPortion(1)))
                            .push(
                                Row::new()
                                    .push(
                                        Button::new(
                                            &mut app.grapher.clear_all_button,
                                            Text::new("Clear all"),
                                        )
                                        .on_press(Message::Grapher(
                                            GrapherMessage::ClearAll,
                                        )),
                                    )
                                    .push(pick_list),
                            )
                            .width(Length::from(300)),
                    )
                    .push(
                        Canvas::new(&mut app.grapher.graph)
                            .width(dimension)
                            .height(dimension),
                    ),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(20)
            .align_items(Align::Center),
    )
    .into()
}

pub fn process_grapher_message(app: &mut ComplexApp, message: GrapherMessage) {
    match message {
        GrapherMessage::ClearEvaluationFunction => {
            app.grapher.contour_input = "".to_string()
        }
        GrapherMessage::ChangeEvaluationFunctionInput(v) => {
            app.grapher.contour_input = v
        }
        GrapherMessage::ChangeRadius(v) => app.grapher.radius_input = v,
        GrapherMessage::UpdateRadius => {
            let radii = app.grapher.radius_input.parse::<f32>();
            if let Ok(radius) = radii {
                app.grapher.graph.radius = radius;
                app.grapher.graph.update();
            }
        }
        GrapherMessage::ChangeDivisions(v) => app.grapher.divisions_input = v,
        GrapherMessage::UpdateDivisions => {
            let divs = app.grapher.divisions_input.parse::<u32>();
            if let Ok(div) = divs {
                app.grapher.graph.divisions = div;
                app.grapher.graph.update_grid();
            }
        }
        GrapherMessage::ChangePrecisionX(v) => {
            app.grapher.precision_input_x = v
        }
        GrapherMessage::ChangePrecisionY(v) => {
            app.grapher.precision_input_y = v
        }
        GrapherMessage::EqualitySelectedLeftX(o) => {
            app.grapher.selected_equality_left_x = Some(o);
        }
        GrapherMessage::EqualitySelectedRightX(o) => {
            app.grapher.selected_equality_right_x = Some(o);
        }
        GrapherMessage::EqualitySelectedLeftY(o) => {
            app.grapher.selected_equality_left_y = Some(o);
        }
        GrapherMessage::EqualitySelectedRightY(o) => {
            app.grapher.selected_equality_right_y = Some(o);
        }
        GrapherMessage::ChangeIntervalLeftX(v) => {
            app.grapher.interval_input_left_x = v
        }
        GrapherMessage::ChangeIntervalRightX(v) => {
            app.grapher.interval_input_right_x = v
        }
        GrapherMessage::ChangeIntervalLeftY(v) => {
            app.grapher.interval_input_left_y = v
        }
        GrapherMessage::ChangeIntervalRightY(v) => {
            app.grapher.interval_input_right_y = v
        }
        GrapherMessage::ChangeFunctionInput(v) => {
            app.grapher.function_input = v
        }
        GrapherMessage::OptionSelected(o) => {
            app.grapher.graph.selected_option = Some(o);
        }
        GrapherMessage::ClearAll => {
            app.grapher.function_input = "".to_string();
            app.grapher.graph.points = vec![];
            app.grapher.graph.functions = vec![];
            app.grapher.graph.update();
        }
        GrapherMessage::GraphFunction => {
            let left_x = app.grapher.interval_input_left_x.parse::<i32>();
            let right_x = app.grapher.interval_input_right_x.parse::<i32>();
            let left_y = app.grapher.interval_input_left_y.parse::<i32>();
            let right_y = app.grapher.interval_input_right_y.parse::<i32>();

            let l_x: i32;
            match left_x {
                Ok(v) => l_x = v,
                Err(_) => l_x = -10,
            }

            let r_x: i32;
            match right_x {
                Ok(v) => r_x = v,
                Err(_) => r_x = 10,
            }
            let l_y: i32;
            match left_y {
                Ok(v) => l_y = v,
                Err(_) => l_y = -10,
            }

            let r_y: i32;
            match right_y {
                Ok(v) => r_y = v,
                Err(_) => r_y = 10,
            }

            let mut x_interval = (-10, 10);
            let mut y_interval = (-10, 10);

            match app.grapher.selected_equality_left_x {
                Some(v) => match v {
                    LeftEqualities::Less => {
                        x_interval.0 = l_x + 1;
                    }
                    LeftEqualities::LessEqual => {
                        x_interval.0 = l_x;
                    }
                },
                None => {
                    x_interval.0 = l_x;
                }
            }
            match app.grapher.selected_equality_right_x {
                Some(v) => match v {
                    LeftEqualities::Less => {
                        x_interval.1 = r_x - 1;
                    }
                    LeftEqualities::LessEqual => {
                        x_interval.1 = r_x;
                    }
                },
                None => {
                    x_interval.1 = r_x;
                }
            }
            match app.grapher.selected_equality_left_y {
                Some(v) => match v {
                    LeftEqualities::Less => {
                        y_interval.0 = l_y + 1;
                    }
                    LeftEqualities::LessEqual => {
                        y_interval.0 = l_y;
                    }
                },
                None => {
                    y_interval.0 = l_y;
                }
            }
            match app.grapher.selected_equality_right_y {
                Some(v) => match v {
                    LeftEqualities::Less => {
                        y_interval.1 = r_y - 1;
                    }
                    LeftEqualities::LessEqual => {
                        y_interval.1 = r_y;
                    }
                },
                None => {
                    y_interval.1 = r_y;
                }
            }

            if x_interval.0 > x_interval.1 || y_interval.0 > y_interval.1 {
                return;
            }

            let x_precision = app.grapher.precision_input_x.parse::<i32>();
            let y_precision = app.grapher.precision_input_y.parse::<i32>();

            let xp: i32;
            let yp: i32;
            match x_precision {
                Ok(v) => xp = v,
                Err(_) => xp = 1,
            }
            match y_precision {
                Ok(v) => yp = v,
                Err(_) => yp = 1,
            }

            let input = app.grapher.function_input.replace(" ", "");
            let algorithm = shunting_yard(input.clone());

            let evaluation_function =
                app.grapher.contour_input.replace(" ", "");
            let ev_algorithm = shunting_yard(evaluation_function);

            if !(ev_algorithm.len() > 0) {
                let points =
                    evaulate_points(algorithm, x_interval, xp, y_interval, yp);
                let mut f_points = vec![];
                for (z, r, g) in points.iter() {
                    let t = (*z, *r, *g);
                    f_points.push(t);
                }
                app.grapher
                    .graph
                    .functions
                    .push(Function::new(input, f_points));
            } else {
                let points = evaulate_points_on_contour(
                    algorithm,
                    x_interval,
                    xp,
                    ev_algorithm,
                );
                let mut f_points = vec![];
                for (z, r, g) in points.iter() {
                    let t = (*z, *r, *g);
                    f_points.push(t);
                }
                app.grapher
                    .graph
                    .functions
                    .push(Function::new(input, f_points));
            }

            app.grapher.function_input = "".to_string();
            app.grapher.graph.update();
        }
    };
}

#[derive(Debug)]
struct GraphState {
    divisions: u32,
    radius: f32,
    grid_cache: canvas::Cache,
    input_cache: canvas::Cache,
    function_cache: canvas::Cache,
    cursor_position: Point,
    points: Vec<Complex<f32>>,
    functions: Vec<Function>,
    selected_option: Option<FOptions>,
}
impl Default for GraphState {
    fn default() -> GraphState {
        GraphState {
            divisions: 10,
            radius: 1.0,
            grid_cache: Default::default(),
            input_cache: Default::default(),
            function_cache: Default::default(),
            cursor_position: Point::ORIGIN,
            points: Vec::new(),
            functions: Vec::new(),
            selected_option: Default::default(),
        }
    }
}
impl GraphState {
    pub fn update(&mut self) {
        self.input_cache.clear();
        self.function_cache.clear();
    }
    pub fn update_grid(&mut self) {
        self.grid_cache.clear();
        self.input_cache.clear();
        self.function_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for GraphState {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let length = self.divisions * 2;
        let scale = bounds.height / length as f32;
        let horizontal_divisions = (bounds.height / scale).floor();
        let n = (horizontal_divisions / 2.0).ceil();
        let vertical_divisions = (bounds.width / scale).floor();
        let m = (vertical_divisions / 2.0).ceil();
        let radius = self.radius;

        // Function points
        let function_points =
            self.function_cache.draw(bounds.size(), |frame| {
                for function in self.functions.iter() {
                    for (z, r, g) in function.points.iter() {
                        let x = (z.re + m as f32) * scale;
                        if x > frame.size().width || x < -frame.size().width {
                            continue;
                        }
                        let y =
                            frame.size().height - ((z.im + n as f32) * scale);
                        if y > frame.size().height || y < -frame.size().height {
                            continue;
                        }
                        let circle = Path::circle(Point::new(x, y), radius);

                        let mut red = 45;
                        let mut green = 45;
                        let blue = 255;
                        if let Some(o) = self.selected_option {
                            match o {
                                FOptions::X => {
                                    red = (*r * 40) as u8;
                                }
                                FOptions::Y => {
                                    green = (*g * 40) as u8;
                                }
                                FOptions::Both => {
                                    red = (*r * 40) as u8;
                                    green = (*g * 40) as u8;
                                }
                            }
                        }

                        let color = Color::from_rgba8(red, green, blue, 1.0);
                        frame.fill(&circle, color);
                    }
                }
            });

        // Grid
        let canvas = self.grid_cache.draw(bounds.size(), |frame| {
            // Grid lines horizontal
            for i in 0..(horizontal_divisions + 1.0) as usize {
                let center = i == n as usize;
                let color = if center {
                    Color::BLACK
                } else {
                    Color::from_rgba8(192, 192, 192, 1.0)
                };
                let line = Path::line(
                    Point::new(0.0, i as f32 * scale),
                    Point::new(frame.size().width, i as f32 * scale),
                );
                frame.stroke(
                    &line,
                    Stroke {
                        width: 1.0,
                        color,
                        ..Stroke::default()
                    },
                );
            }

            // Grid lines vertical
            for i in 0..(vertical_divisions + 1.0) as usize {
                let center = i == m as usize;
                let color = if center {
                    Color::BLACK
                } else {
                    Color::from_rgba8(192, 192, 192, 1.0)
                };
                let line = Path::line(
                    Point::new(i as f32 * scale, 0.0),
                    Point::new(i as f32 * scale, bounds.height),
                );
                frame.stroke(
                    &line,
                    Stroke {
                        width: 1.0,
                        color,
                        ..Stroke::default()
                    },
                );
            }

            // Grid numbers
            for i in 0..(vertical_divisions + 1.0) as usize {
                for j in 0..(horizontal_divisions + 1.0) as usize {
                    let x = frame.size().width - scale * i as f32;
                    let y = frame.size().height - scale * j as f32;
                    // horizontal
                    if i == m as usize {
                        let text = CText {
                            color: Color::BLACK,
                            size: 14.0,
                            position: Point::new(x, y),
                            horizontal_alignment: HorizontalAlignment::Right,
                            vertical_alignment: VerticalAlignment::Bottom,
                            ..CText::default()
                        };
                        let num = if i == length as usize {
                            m as i32
                        } else {
                            j as i32 - n as i32
                        };
                        frame.fill_text(CText {
                            content: format!("{}", num),
                            position: text.position,
                            ..text
                        });
                    }
                    // vertical
                    if j == n as usize {
                        let text = CText {
                            color: Color::BLACK,
                            size: 14.0,
                            position: Point::new(x, y),
                            horizontal_alignment: HorizontalAlignment::Right,
                            vertical_alignment: VerticalAlignment::Bottom,
                            ..CText::default()
                        };
                        let num = if j == length as usize {
                            n as i32
                        } else {
                            m as i32 - i as i32
                        };
                        frame.fill_text(CText {
                            content: format!("{}", num),
                            position: text.position,
                            ..text
                        });
                    }
                }
            }
        });
        vec![canvas, function_points]
    }

    fn mouse_interaction(
        &self,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug)]
struct Function {
    operation: String,
    points: Vec<(Complex<f32>, i32, i32)>,
}

impl Function {
    pub fn new(
        operation: String,
        points: Vec<(Complex<f32>, i32, i32)>,
    ) -> Function {
        Function { operation, points }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FOptions {
    Both,
    X,
    Y,
}

impl FOptions {
    const ALL: [FOptions; 3] = [FOptions::Both, FOptions::X, FOptions::Y];
}

impl Default for FOptions {
    fn default() -> FOptions {
        FOptions::Both
    }
}
impl std::fmt::Display for FOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FOptions::Both => "Both",
                FOptions::X => "X",
                FOptions::Y => "Y",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftEqualities {
    Less,
    LessEqual,
}

impl LeftEqualities {
    const ALL: [LeftEqualities; 2] =
        [LeftEqualities::Less, LeftEqualities::LessEqual];
}

impl Default for LeftEqualities {
    fn default() -> LeftEqualities {
        LeftEqualities::Less
    }
}
impl std::fmt::Display for LeftEqualities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LeftEqualities::Less => "<",
                LeftEqualities::LessEqual => "<=",
            }
        )
    }
}
