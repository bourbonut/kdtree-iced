use crate::Message;
use iced::{Color, Point, Rectangle, Renderer, Theme, mouse, widget::canvas};

pub static LINE_STROKE_WIDTH: f32 = 2.;
pub static CIRCLE_RADIUS: f32 = 5.;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Line {
    Vertical(f32),
    Horizontal(f32),
    PointDirection(Point, Direction),
    PointToPoint(Point, Point),
}

pub struct Geometry {
    target: Option<Point>,
    neighbor: Option<Point>,
    points: Vec<Point>,
    lines: Vec<Line>,
}

impl Geometry {
    pub fn new(
        points: Vec<Point>,
        lines: Vec<Line>,
        target: Option<Point>,
        neighbor: Option<Point>,
    ) -> Self {
        Self {
            points,
            lines,
            target,
            neighbor,
        }
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

#[inline]
fn invert(point: &Point, bounds: &Rectangle) -> Point {
    Point::new(
        (point.x - bounds.x) / bounds.width,
        (point.y - bounds.y) / bounds.height,
    )
}

impl canvas::Program<Message> for Geometry {
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

        for point in self.points.iter() {
            let circle = canvas::Path::circle(scale(point, &bounds), CIRCLE_RADIUS);

            frame.fill(&circle, theme.palette().primary);
        }

        if let Some(point) = self.target {
            let circle = canvas::Path::circle(scale(&point, &bounds), CIRCLE_RADIUS);

            frame.fill(&circle, Color::new(0.0, 1.0, 0.0, 1.0));
        }

        if let Some(point) = self.neighbor {
            let circle = canvas::Path::circle(scale(&point, &bounds), CIRCLE_RADIUS);

            frame.fill(&circle, Color::new(1.0, 0.0, 0.0, 1.0));
        }
        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        if let Some(position) = cursor.position() {
            match event {
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::AddPoint(invert(&position, &bounds))),
                    );
                }
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::FindNeighbor(invert(&position, &bounds))),
                    );
                }
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => {
                    return (
                        canvas::event::Status::Captured,
                        Some(Message::DeletePoint(invert(&position, &bounds))),
                    );
                }
                _ => (),
            }
        }
        (canvas::event::Status::Ignored, None)
    }
}
