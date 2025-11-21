use criterion::{Criterion, criterion_group, criterion_main};
use iced::Point;
use kdtree_iced::KDTree;

fn random_point() -> Point {
    Point::new(rand::random::<f32>(), rand::random::<f32>())
}

pub fn creating_100_000_points(c: &mut Criterion) {
    c.bench_function("creating_100_000_points", |b| {
        let points: Vec<Point> = (0..100_000).map(|_| random_point()).collect();
        b.iter(|| KDTree::from_points(&points))
    });
}

pub fn insertion(c: &mut Criterion) {
    let mut tree = KDTree::default();
    for _ in 0..100_000 {
        tree.add_point(random_point());
    }
    c.bench_function("insertion", |b| {
        let point = random_point();
        b.iter(|| tree.add_point(point))
    });
}

pub fn nearest_neighbor(c: &mut Criterion) {
    let points: Vec<Point> = (0..100_000).map(|_| random_point()).collect();
    let tree = KDTree::from_points(&points);
    c.bench_function("nearest_neighbor", |b| {
        let point = random_point();
        b.iter(|| tree.nearest_neighbor(&point))
    });
}

pub fn deletion(c: &mut Criterion) {
    let points: Vec<Point> = (0..100_000).map(|_| random_point()).collect();
    let mut tree = KDTree::from_points(&points);
    c.bench_function("deletion", |b| {
        let point = points[rand::random_range(0..10_000)];
        b.iter(|| tree.remove_point(point));
    });
}

criterion_group!(
    benches,
    creating_100_000_points,
    insertion,
    nearest_neighbor,
    deletion
);
criterion_main!(benches);
