use core::f32;

use crate::geometry;
use iced::{Point, Rectangle};

#[derive(Debug)]
enum Split {
    X,
    Y,
}

impl Split {
    fn opposite(&self) -> Self {
        match self {
            Split::Y => Split::X,
            Split::X => Split::Y,
        }
    }
}

#[derive(Debug)]
struct Node {
    point: Point,
    left: Option<usize>,
    right: Option<usize>,
    split: Split,
}

impl Node {
    fn is_in_hypersphere(&self, point: &Point, radius: f32) -> bool {
        match self.split {
            Split::X => radius > (point.x - self.point.x).abs(),
            Split::Y => radius > (point.y - self.point.y).abs(),
        }
    }

    fn direction(&self, point: &Point) -> bool {
        match self.split {
            Split::X => point.x <= self.point.x,
            Split::Y => point.y <= self.point.y,
        }
    }
}

#[derive(Default, Debug)]
pub struct KDTree {
    nodes: Vec<Node>,
}

impl KDTree {
    #[allow(dead_code)]
    pub fn from_points(points: &[Point]) -> Self {
        let mut tree = KDTree::default();
        for point in points {
            tree.add_point(*point);
        }
        tree
    }

    pub fn add_point(&mut self, point: Point) {
        if self.nodes.is_empty() {
            self.nodes.push(Node {
                point,
                left: None,
                right: None,
                split: Split::X,
            });
        } else {
            let node_index = self.find_node(&point, 0);
            let next_index = self.nodes.len();

            let node = &mut self.nodes[node_index];

            if node.direction(&point) {
                node.left = Some(next_index)
            } else {
                node.right = Some(next_index)
            };

            let node = &self.nodes[node_index];
            self.nodes.push(Node {
                point,
                left: None,
                right: None,
                split: node.split.opposite(),
            });
        }
    }

    fn find_node(&self, point: &Point, node_index: usize) -> usize {
        match self.single_search(point, node_index) {
            Some(index) => self.find_node(point, index),
            None => node_index,
        }
    }

    fn single_search(&self, point: &Point, node_index: usize) -> Option<usize> {
        let node = &self.nodes[node_index];
        if node.direction(point) {
            node.left
        } else {
            node.right
        }
    }

    pub fn nearest_neighbor(&self, point: &Point) -> Option<Point> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(self.nearest_neighbor_search(point, 0))
        }
    }

    fn nearest_neighbor_search(&self, point: &Point, node_index: usize) -> Point {
        let node = &self.nodes[node_index];
        let (primary, secondary) = if node.direction(point) {
            (node.left, node.right)
        } else {
            (node.right, node.left)
        };

        let (mut best_point, mut best_distance) = match primary {
            Some(idx) => {
                let p = self.nearest_neighbor_search(point, idx);
                (p, point.distance(p))
            }
            None => (node.point, point.distance(node.point)),
        };

        if let Some(secondary_index) = secondary
            && node.is_in_hypersphere(point, best_distance)
        {
            let secondary_best = self.nearest_neighbor_search(point, secondary_index);
            let dist = point.distance(secondary_best);
            if dist < best_distance {
                best_point = secondary_best;
                best_distance = dist;
            }
        }

        let node_distance = point.distance(node.point);
        if node_distance < best_distance {
            node.point
        } else {
            best_point
        }
    }

    fn dfs_lines(&self, node_index: usize, lines: &mut Vec<geometry::Line>, bounds: Rectangle) {
        let node = &self.nodes[node_index];
        if let Some(index) = node.left {
            let left = &self.nodes[index];
            match left.split {
                Split::X => {
                    lines.push(geometry::Line::PointToPoint(
                        Point::new(left.point.x, node.point.y),
                        Point::new(left.point.x, bounds.y),
                    ));
                    let bounds = Rectangle {
                        height: node.point.y,
                        ..bounds
                    };
                    self.dfs_lines(index, lines, bounds);
                }
                Split::Y => {
                    lines.push(geometry::Line::PointToPoint(
                        Point::new(node.point.x, left.point.y),
                        Point::new(bounds.x, left.point.y),
                    ));
                    let bounds = Rectangle {
                        width: node.point.x,
                        ..bounds
                    };
                    self.dfs_lines(index, lines, bounds);
                }
            }
        }
        if let Some(index) = node.right {
            let right = &self.nodes[index];
            match right.split {
                Split::X => {
                    lines.push(geometry::Line::PointToPoint(
                        Point::new(right.point.x, node.point.y),
                        Point::new(right.point.x, bounds.height),
                    ));
                    let bounds = Rectangle {
                        y: node.point.y,
                        ..bounds
                    };
                    self.dfs_lines(index, lines, bounds);
                }
                Split::Y => {
                    lines.push(geometry::Line::PointToPoint(
                        Point::new(node.point.x, right.point.y),
                        Point::new(bounds.width, right.point.y),
                    ));
                    let bounds = Rectangle {
                        x: node.point.x,
                        ..bounds
                    };
                    self.dfs_lines(index, lines, bounds);
                }
            }
        }
    }

    pub fn lines(&self) -> Vec<geometry::Line> {
        if let Some(root) = self.nodes.first() {
            let mut lines = Vec::new();
            lines.push(geometry::Line::Vertical(root.point.x));
            self.dfs_lines(
                0,
                &mut lines,
                Rectangle {
                    x: 0.,
                    y: 0.,
                    width: 1.,
                    height: 1.,
                },
            );
            lines
        } else {
            Vec::new()
        }
    }

    pub fn points(&self) -> Vec<Point> {
        self.nodes.iter().map(|node| node.point).collect()
    }
}
