use iced::{Element, Length, Theme, widget::canvas};

mod lines;

struct App {
    lines: Vec<lines::Line>,
}

impl Default for App {
    fn default() -> Self {
        let lines = vec![lines::Line::Vertical];
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
    iced::application("Iced Visualization - KDTree", App::update, App::view)
        .theme(|_| Theme::Light)
        .run()?;
    Ok(())
}
