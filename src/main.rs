use iced::{Element, Length, Point, Theme, widget::canvas};

mod kdtree;
mod lines;

struct App {
    lines: Vec<lines::Line>,
}

impl Default for App {
    fn default() -> Self {
        // let lines = vec![lines::Line::PointToPoint(
        //     Point::new(0.5, 0.5),
        //     Point::new(0.8, 0.5),
        // )];
        let points = vec![
            Point::new(0.5, 1. - 0.6),
            Point::new(0.1, 1. - 0.3),
            Point::new(0.2, 1. - 0.15),
            Point::new(0.4, 1. - 0.45),
            Point::new(0.8, 1. - 0.8),
            Point::new(0.6, 1. - 0.18),
            Point::new(0.45, 1. - 0.45),
            Point::new(0.15, 1. - 0.2),
        ];
        // let points: Vec<Point> = (0..100)
        //     .map(|_| Point::new(rand::random::<f32>(), rand::random::<f32>()))
        //     .collect();
        let tree = kdtree::KDTree::new(&points);
        let lines = tree.lines();
        Self { lines }
    }
}

#[derive(Debug)]
enum Message {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        canvas::Canvas::new(lines::Lines::new(&self.lines))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    // let points = vec![
    //     Point::new(0.5, 0.6),
    //     Point::new(0.1, 0.3),
    //     Point::new(0.2, 0.15),
    //     Point::new(0.4, 0.45),
    //     Point::new(0.8, 0.8),
    //     Point::new(0.6, 0.18),
    // ];
    // dbg!(&points);
    // let tree = kdtree::KDTree::new(&points);
    // dbg!(tree);

    iced::application("Iced Visualization - KDTree", App::update, App::view)
        .theme(|_| Theme::Light)
        .run()?;
    Ok(())
}
