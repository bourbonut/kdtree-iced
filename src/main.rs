mod app;
mod geometry;
mod kdtree;

use app::App;
use iced::Theme;

fn main() -> iced::Result {
    iced::application("Iced Visualization - KDTree", App::update, App::view)
        .theme(|_| Theme::Light)
        .antialiasing(true)
        .run()?;
    Ok(())
}
