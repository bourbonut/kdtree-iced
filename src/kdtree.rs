use crate::lines;
use iced::Point;

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

#[derive(Default, Debug)]
pub struct KDTree {
    nodes: Vec<Node>,
}

impl KDTree {
    pub fn new(points: &[Point]) -> Self {
        let mut tree = KDTree::default();
        if let Some((root_point, points)) = points.split_first() {
            tree.nodes.push(Node {
                point: *root_point,
                left: None,
                right: None,
                split: Split::X,
            });
            for point in points {
                let node_index = tree.find_node(point, 0);
                let next_index = tree.nodes.len();

                let node = &mut tree.nodes[node_index];

                match node.split {
                    Split::X => {
                        if point.x <= node.point.x {
                            node.left = Some(next_index)
                        } else {
                            node.right = Some(next_index)
                        }
                    }
                    Split::Y => {
                        if point.y <= node.point.y {
                            node.left = Some(next_index)
                        } else {
                            node.right = Some(next_index)
                        }
                    }
                };

                let node = &tree.nodes[node_index];
                tree.nodes.push(Node {
                    point: *point,
                    left: None,
                    right: None,
                    split: node.split.opposite(),
                })
            }
        }
        tree
    }

    fn find_node(&self, point: &Point, node_index: usize) -> usize {
        match self.single_search(point, node_index) {
            Some(index) => self.find_node(point, index),
            None => node_index,
        }
    }

    fn single_search(&self, point: &Point, node_index: usize) -> Option<usize> {
        let node = &self.nodes[node_index];
        match node.split {
            Split::X => {
                if point.x <= node.point.x {
                    node.left
                } else {
                    node.right
                }
            }
            Split::Y => {
                if point.y <= node.point.y {
                    node.left
                } else {
                    node.right
                }
            }
        }
    }

    fn dfs_lines(
        &self,
        node_idx: usize,
        lines: &mut Vec<lines::Line>,
        rotation: i32,
        last_x: &f32,
        last_y: &f32,
    ) {
        let node = &self.nodes[node_idx];
        if let Some(index) = node.left {
            let left = &self.nodes[index];
            match left.split {
                Split::X => {
                    lines.push(if rotation.abs() > 3 {
                        lines::Line::PointToPoint(
                            Point::new(left.point.x, *last_y),
                            Point::new(*last_x, *last_y),
                        )
                    } else {
                        lines::Line::PointDirection(
                            Point::new(left.point.x, *last_y),
                            lines::Direction::Top,
                        )
                    });
                    self.dfs_lines(index, lines, rotation + 1, &left.point.x, last_y);
                }
                Split::Y => {
                    lines.push(if rotation.abs() > 3 {
                        lines::Line::PointToPoint(
                            Point::new(*last_x, left.point.y),
                            Point::new(*last_x, *last_y),
                        )
                    } else {
                        lines::Line::PointDirection(
                            Point::new(*last_x, left.point.y),
                            lines::Direction::Left,
                        )
                    });
                    self.dfs_lines(index, lines, rotation + 1, last_x, &left.point.y);
                }
            }
        }
        if let Some(index) = node.right {
            let right = &self.nodes[index];
            match right.split {
                Split::X => {
                    lines.push(if rotation.abs() > 3 {
                        lines::Line::PointToPoint(
                            Point::new(right.point.x, *last_y),
                            Point::new(*last_x, *last_y),
                        )
                    } else {
                        lines::Line::PointDirection(
                            Point::new(right.point.x, *last_y),
                            lines::Direction::Bottom,
                        )
                    });
                    self.dfs_lines(index, lines, rotation - 1, &right.point.x, last_y);
                }
                Split::Y => {
                    lines.push(if rotation.abs() > 3 {
                        lines::Line::PointToPoint(
                            Point::new(*last_x, right.point.y),
                            Point::new(*last_x, *last_y),
                        )
                    } else {
                        lines::Line::PointDirection(
                            Point::new(*last_x, right.point.y),
                            lines::Direction::Right,
                        )
                    });
                    self.dfs_lines(index, lines, rotation - 1, last_x, &right.point.y);
                }
            }
        }
    }

    pub fn lines(&self) -> Vec<lines::Line> {
        if let Some(root) = self.nodes.first() {
            let mut lines = Vec::new();
            lines.push(lines::Line::Vertical(root.point.x));
            self.dfs_lines(0, &mut lines, 0, &root.point.x, &root.point.y);
            lines
        } else {
            Vec::new()
        }
    }
}
