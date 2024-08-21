use ratatui::style::Color;
use ratatui::widgets::Widget;

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

pub struct SpaceshipModel {
    pub position: Point,
}

impl SpaceshipModel {
    pub fn new(x: u16, y: u16) -> Self {
        SpaceshipModel {
            position: Point::new(x, y),
        }
    }
}

pub struct SpaceshipWidget<'a> {
    model: &'a SpaceshipModel,
    color: Color,
}

impl SpaceshipWidget<'_> {
    pub fn new(model: &SpaceshipModel, color: Color) -> SpaceshipWidget {
        SpaceshipWidget { model, color }
    }
}

impl Widget for SpaceshipWidget<'_> {
    fn render(self, _area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        buf.get_mut(self.model.position.x, self.model.position.y)
            .set_char('►')
            .set_fg(self.color);
    }
}
