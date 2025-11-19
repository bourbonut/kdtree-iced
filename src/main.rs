use iced::{Element, Length, Point, Theme, widget::canvas};

mod geometry;
mod kdtree;

struct App {
    tree: kdtree::KDTree,
}

impl Default for App {
    fn default() -> Self {
        // let points = vec![
        //     Point::new(0.5, 0.6),
        //     Point::new(0.1, 0.3),
        //     Point::new(0.2, 0.15),
        //     Point::new(0.4, 0.45),
        //     Point::new(0.8, 0.8),
        //     Point::new(0.6, 0.18),
        // ];
        Self {
            tree: kdtree::KDTree::default(),
        }
    }
}

#[derive(Debug)]
enum Message {
    AddPoint(Point),
}

impl App {
    fn update(&mut self, message: Message) {
        let Message::AddPoint(point) = message;
        self.tree.add_point(point);
    }

    fn view(&self) -> Element<'_, Message> {
        canvas::Canvas::new(geometry::Geometry::new(
            self.tree.points(),
            self.tree.lines(),
        ))
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
        .antialiasing(true)
        .run()?;
    Ok(())
}
