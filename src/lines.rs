use crate::Message;
use iced::{Point, Rectangle, Renderer, Theme, mouse, widget::canvas};

pub static LINE_STROKE_WIDTH: f32 = 2.;

pub(crate) enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

pub enum Line {
    Vertical(f32),
    Horizontal(f32),
    PointDirection(Point, Direction),
    PointToPoint(Point, Point),
}

pub struct Lines<'a> {
    lines: &'a Vec<Line>,
}

impl<'a> Lines<'a> {
    pub fn new(lines: &'a Vec<Line>) -> Self {
        Self { lines }
    }
}

#[derive(Default)]
pub(crate) struct State {}

#[inline]
fn scale(point: &Point, bounds: &Rectangle) -> Point {
    Point::new(
        bounds.x + bounds.width * point.x,
        bounds.y + bounds.height * point.y,
    )
}

impl<'a> canvas::Program<Message> for Lines<'a> {
    type State = State;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        for line in self.lines.iter() {
            let line = match line {
                Line::Horizontal(y) => {
                    let y = bounds.y + bounds.height * y;
                    canvas::Path::line(
                        Point::new(bounds.x, y),
                        Point::new(bounds.x + bounds.width, y),
                    )
                }
                Line::Vertical(x) => {
                    let x = bounds.x + bounds.width * x;
                    canvas::Path::line(
                        Point::new(x, bounds.y),
                        Point::new(x, bounds.y + bounds.height),
                    )
                }
                Line::PointDirection(point, direction) => {
                    let point = scale(point, &bounds);
                    match direction {
                        Direction::Top => canvas::Path::line(point, Point::new(point.x, bounds.y)),
                        Direction::Bottom => {
                            canvas::Path::line(point, Point::new(point.x, bounds.y + bounds.height))
                        }
                        Direction::Left => canvas::Path::line(point, Point::new(bounds.x, point.y)),
                        Direction::Right => {
                            canvas::Path::line(point, Point::new(bounds.x + bounds.width, point.y))
                        }
                    }
                }
                Line::PointToPoint(from, to) => {
                    canvas::Path::line(scale(from, &bounds), scale(to, &bounds))
                }
            };
            frame.stroke(
                &line,
                canvas::Stroke::default()
                    .with_width(LINE_STROKE_WIDTH)
                    .with_color(theme.palette().primary),
            );
        }
        vec![frame.into_geometry()]
    }
}
