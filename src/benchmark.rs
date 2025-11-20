mod app;
mod geometry;
mod kdtree;
use iced::Point;

fn main() -> std::io::Result<()> {
    use std::time::Instant;
    let mut tree = kdtree::KDTree::default();
    let mut durations = Vec::new();

    for _ in 0..1_000_000 {
        let point = Point::new(rand::random::<f32>(), rand::random::<f32>());
        let start = Instant::now();
        tree.add_point(point);
        let end = Instant::now();
        let duration = (end - start).as_micros();
        durations.push(duration);
    }
    let avg = durations.iter().sum::<u128>() as f64 / durations.iter().len() as f64;
    let std = durations
        .iter()
        .map(|d| (*d as f64 - avg).powi(2))
        .sum::<f64>()
        .sqrt();
    println!("Insertion: {:?}µs +/- {:?}µs", avg, std);

    let mut durations = Vec::new();
    for _ in 0..1000 {
        let points: Vec<Point> = (0..10_000)
            .map(|_| Point::new(rand::random::<f32>(), rand::random::<f32>()))
            .collect();
        let tree = kdtree::KDTree::from_points(&points);
        let target = Point::new(rand::random::<f32>(), rand::random::<f32>());
        let start = Instant::now();
        tree.nearest_neighbor(&target);
        let end = Instant::now();
        let duration = (end - start).as_micros();
        durations.push(duration);
    }

    let avg = durations.iter().sum::<u128>() as f64 / durations.iter().len() as f64;
    let std = durations
        .iter()
        .map(|d| (*d as f64 - avg).powi(2))
        .sum::<f64>()
        .sqrt();
    println!("Nearest neighbor: {:?}µs +/- {:?}µs", avg, std);

    let mut durations = Vec::new();
    for _ in 0..1000 {
        let points: Vec<Point> = (0..10_000)
            .map(|_| Point::new(rand::random::<f32>(), rand::random::<f32>()))
            .collect();
        let mut tree = kdtree::KDTree::from_points(&points);
        let target = points[rand::random_range(0..10_000)];
        let start = Instant::now();
        tree.remove_point(target);
        let end = Instant::now();
        let duration = (end - start).as_micros();
        durations.push(duration);
    }

    let avg = durations.iter().sum::<u128>() as f64 / durations.iter().len() as f64;
    let std = durations
        .iter()
        .map(|d| (*d as f64 - avg).powi(2))
        .sum::<f64>()
        .sqrt();
    println!("Nearest neighbor: {:?}µs +/- {:?}µs", avg, std);
    Ok(())
}
