use crate::geometry;
use crate::kdtree;
use iced::{Element, Length, Point, widget::canvas};

/// Minimum distance between two points to be considered the same.
static MIN_DISTANCE: f32 = 0.005;

/// The main application structure
pub struct App {
    /// `KDTree` tree
    tree: kdtree::KDTree,
    /// Current nearest neighbor point from tree's points
    nearest_neighbor: Option<Point>,
    /// Target point
    target: Option<Point>,
}

impl Default for App {
    fn default() -> Self {
        let points = vec![
            Point::new(0.5, 1. - 0.6),
            Point::new(0.1, 1. - 0.3),
            Point::new(0.2, 1. - 0.15),
            Point::new(0.4, 1. - 0.45),
            Point::new(0.8, 1. - 0.8),
            Point::new(0.6, 1. - 0.18),
        ];
        Self {
            tree: kdtree::KDTree::from_points(&points),
            nearest_neighbor: None,
            target: None,
        }
    }
}

/// Message variants sent by mouse events
#[derive(Debug)]
pub enum Message {
    /// Message for adding a point into the tree
    AddPoint(Point),
    /// Message for adding a target point and finding the nearest neighbor point into the tree
    FindNeighbor(Point),
    /// Message for removing a point into the tree
    DeletePoint(Point),
}

impl App {
    /// Updates the application state given the specified message.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddPoint(point) => {
                self.tree.add_point(point);
                match self.target {
                    Some(point) => self.nearest_neighbor = self.tree.nearest_neighbor(&point),
                    None => self.nearest_neighbor = None,
                }
            }
            Message::FindNeighbor(point) => {
                self.nearest_neighbor = self.tree.nearest_neighbor(&point);
                self.target = Some(point);
            }
            Message::DeletePoint(point) => {
                if let Some(point_to_remove) = self.tree.nearest_neighbor(&point)
                    && point_to_remove.distance(point) <= MIN_DISTANCE
                {
                    self.tree.remove_point(point_to_remove);
                    match self.target {
                        Some(point) => self.nearest_neighbor = self.tree.nearest_neighbor(&point),
                        None => self.nearest_neighbor = None,
                    }
                }
            }
        }
    }

    /// Returns the widget displayed on the screen
    pub fn view(&self) -> Element<'_, Message> {
        canvas::Canvas::new(geometry::Geometry::new(
            self.tree.points(),
            self.tree.lines(),
            self.target,
            self.nearest_neighbor,
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
