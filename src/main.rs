use iced::{Element, Length, Point, Theme, widget::canvas};

mod geometry;
mod kdtree;

struct App {
    tree: kdtree::KDTree,
    closest_neighbor: Option<Point>,
    target: Option<Point>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tree: kdtree::KDTree::default(),
            closest_neighbor: None,
            target: None,
        }
    }
}

#[derive(Debug)]
enum Message {
    AddPoint(Point),
    FindNeighbor(Point),
}

impl App {
    fn update(&mut self, message: Message) {
        println!();
        match message {
            Message::AddPoint(point) => {
                match self.target {
                    Some(point) => self.closest_neighbor = self.tree.nearest_neighbor(&point),
                    None => self.closest_neighbor = None,
                }
                self.tree.add_point(point);
            }
            Message::FindNeighbor(point) => {
                self.closest_neighbor = self.tree.nearest_neighbor(&point);
                self.target = Some(point);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        canvas::Canvas::new(geometry::Geometry::new(
            self.tree.points(),
            self.tree.lines(),
            self.target,
            self.closest_neighbor,
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
