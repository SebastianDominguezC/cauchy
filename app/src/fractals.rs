use crate::{ComplexApp, Message};
use iced::{
    canvas::{self, Canvas, Cursor, Frame, Geometry, Path},
    mouse, Color, Element, Length, Point, Rectangle, Size,
};
#[derive(Default)]
pub struct FractalsState {
    graph_state: GraphState,
}

#[derive(Debug, Clone)]
pub enum FractalsMessage {
    M,
}

pub fn render_fractals(app: &mut ComplexApp) -> Element<Message> {
    Canvas::new(&mut app.fractals.graph_state)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn process_fractals_message(
    app: &mut ComplexApp,
    message: FractalsMessage,
) {
    match message {
        FractalsMessage::M => {}
    };
}

#[derive(Debug)]
struct GraphState {
    grid_cache: canvas::Cache,
    cursor_position: Point,
}
impl Default for GraphState {
    fn default() -> GraphState {
        GraphState {
            grid_cache: Default::default(),
            cursor_position: Point::ORIGIN,
        }
    }
}
impl GraphState {
    pub fn update(&mut self) {
        self.grid_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for GraphState {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let canvas = self.grid_cache.draw(bounds.size(), |mut frame| {
            let size = frame.size().height;

            Fractal::draw(
                size,
                2.0,
                &mut frame,
                Color::from_rgb(0.0, 1.0, 0.0),
                Point::new(0.0, 0.0),
            );
        });
        vec![canvas]
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

struct Fractal {}

impl Fractal {
    fn draw(size: f32, min: f32, frame: &mut Frame, color: Color, pos: Point) {
        let rect = Path::rectangle(pos, Size::new(size, size));
        frame.fill(&rect, color);

        if size > min {
            let size = size / 3.0;
            let x = pos.x;
            let y = pos.y;

            // top
            let tl = Point::new(x, y);
            let tm = Point::new(x + size, y + 0.0);
            let tr = Point::new(x + size * 2.0, y + 0.0);

            // middle
            let ml = Point::new(x, y + size);
            let mm = Point::new(x + size, y + size);
            let mr = Point::new(x + size * 2.0, y + size);

            // bottom
            let bl = Point::new(x, y + size * 2.0);
            let bm = Point::new(x + size, y + size * 2.0);
            let br = Point::new(x + size * 2.0, y + size * 2.0);

            let squares = vec![tl, tm, tr, ml, mr, bl, bm, br];

            for p in squares.iter() {
                Fractal::draw(size, min, frame, color, p.clone());
            }
            Fractal::draw(size, min, frame, Color::from_rgb(0.0, 0.0, 1.0), mm);
        }
    }
}
