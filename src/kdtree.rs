use core::f32;
use std::collections::{HashMap, VecDeque};

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
    /// Checks if distance (`radius` of the hypersphere) is greater than the absolute distance
    /// between the point and the current node point:
    ///
    /// $$
    /// d(T, P) > |\overrightarrow{NT} \cdot \overrightarrow{\text{dir}}|
    /// $$
    ///
    /// where:
    /// - $T$ is the target point (`point`)
    /// - $P$ is the current best neighbor, $d(T, P)$ is the euclidian distance between $T$ and $P$
    ///   (`radius`)
    /// - $N$ is the node point (`self.point`)
    /// - $\overrightarrow{\text{dir}}$ is the split direction (i.e. $\vec x$ or $\vec y$)
    fn is_in_hypersphere(&self, point: &Point, radius: f32) -> bool {
        match self.split {
            Split::X => radius > (point.x - self.point.x).abs(),
            Split::Y => radius > (point.y - self.point.y).abs(),
        }
    }

    /// Returns the direction of the next node child given the specified point where `true`
    /// represents "left" and `false` represents "right".
    fn direction(&self, point: &Point) -> bool {
        match self.split {
            Split::X => point.x <= self.point.x,
            Split::Y => point.y <= self.point.y,
        }
    }
}

#[derive(Default, Debug)]
pub struct KDTree {
    free_indices: VecDeque<usize>,
    nodes: HashMap<usize, Node>,
    root_index: usize,
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
            self.root_index = if let Some(index) = self.free_indices.pop_front() {
                index
            } else {
                self.nodes.len()
            };
            self.nodes.insert(
                self.root_index,
                Node {
                    point,
                    left: None,
                    right: None,
                    split: Split::X,
                },
            );
        } else {
            let node_index = self.find_node(&point, self.root_index);
            let next_index = if let Some(index) = self.free_indices.pop_front() {
                index
            } else {
                self.nodes.len()
            };

            self.nodes.entry(node_index).and_modify(|node| {
                if node.direction(&point) {
                    node.left = Some(next_index)
                } else {
                    node.right = Some(next_index)
                };
            });

            let node = &self.nodes[&node_index];
            self.nodes.insert(
                next_index,
                Node {
                    point,
                    left: None,
                    right: None,
                    split: node.split.opposite(),
                },
            );
        }
    }

    pub fn remove_point(&mut self, point: Point) {
        let (node_index, parent_index) = self.find_parent(point, self.root_index, self.root_index);
        if node_index != parent_index {
            self.nodes.entry(parent_index).and_modify(|node| {
                if Some(node_index) == node.left {
                    node.left = None;
                }
                if Some(node_index) == node.right {
                    node.right = None;
                }
            });
        }
        let mut points = Vec::new();
        self.pop_nodes(node_index, &mut points);
        for point in points[1..].iter() {
            self.add_point(*point);
        }
    }

    fn find_parent(&self, point: Point, node_index: usize, parent_index: usize) -> (usize, usize) {
        if let Some(node) = self.nodes.get(&node_index) {
            if node.point == point {
                (node_index, parent_index)
            } else {
                match self.single_search(&point, node_index) {
                    Some(index) => self.find_parent(point, index, node_index),
                    None => (node_index, parent_index),
                }
            }
        } else {
            (node_index, parent_index)
        }
    }

    fn pop_nodes(&mut self, node_index: usize, points: &mut Vec<Point>) {
        if let Some(node) = self.nodes.remove(&node_index) {
            points.push(node.point);
            self.free_indices.push_back(node_index);
            if let Some(left_index) = node.left {
                self.pop_nodes(left_index, points);
            }
            if let Some(right_index) = node.right {
                self.pop_nodes(right_index, points);
            }
        }
    }

    fn find_node(&self, point: &Point, node_index: usize) -> usize {
        match self.single_search(point, node_index) {
            Some(index) => self.find_node(point, index),
            None => node_index,
        }
    }

    fn single_search(&self, point: &Point, node_index: usize) -> Option<usize> {
        let node = &self.nodes[&node_index];
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
            Some(self.nearest_neighbor_search(point, self.root_index))
        }
    }

    fn nearest_neighbor_search(&self, point: &Point, node_index: usize) -> Point {
        let node = &self.nodes[&node_index];
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
        let node = &self.nodes[&node_index];
        if let Some(index) = node.left {
            let left = &self.nodes[&index];
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
            let right = &self.nodes[&index];
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
        if let Some(root) = self.nodes.get(&self.root_index) {
            let mut lines = Vec::new();
            lines.push(geometry::Line::Vertical(root.point.x));
            self.dfs_lines(
                self.root_index,
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
        self.nodes.values().map(|node| node.point).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    fn random_point() -> Point {
        Point::new(random::<f32>(), random::<f32>())
    }

    #[test]
    fn test_nearest_point() {
        for _ in 0..100 {
            let points: Vec<Point> = (0..1_000).map(|_| random_point()).collect();
            let target = random_point();
            let tree = KDTree::from_points(&points);
            let actual_neighbor = tree.nearest_neighbor(&target).unwrap();
            let expected_neighbor = points
                .iter()
                .min_by(|a, b| {
                    a.distance(target)
                        .partial_cmp(&b.distance(target))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            assert_eq!(actual_neighbor, *expected_neighbor);
        }
    }

    #[test]
    fn test_deletion() {
        for _ in 0..100 {
            let points: Vec<Point> = (0..1_000).map(|_| random_point()).collect();
            let target = rand::random_range(0..1_000);
            let mut tree = KDTree::from_points(&points);
            let point = points[target];
            tree.remove_point(point);
            let points = tree.points();
            assert_eq!(tree.nodes.len(), 999);
            assert!(!points.contains(&point));
        }
    }
}
