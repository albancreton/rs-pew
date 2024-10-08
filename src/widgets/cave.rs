use crate::noise::Perlin;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

pub struct CaveConfig {
    pub opening_max: f64,
    pub opening_min: f64,
    pub frequency: f64,
    pub smooth: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct CaveModel {
    pub seed: f64,
    pub offset_x: i64,
    pub speed_x: i64,

    opening_ratio: f64,
    opening_min: f64,
    frequency: f64,
    smooth: f64,

    pub color: Color,
    pub openings: Vec<(u16, u16)>,
}

impl CaveModel {
    pub fn new(seed: f64, config: CaveConfig) -> Self {
        Self {
            seed,

            offset_x: 0,
            speed_x: 0,

            opening_ratio: config.opening_max,
            opening_min: config.opening_min,
            frequency: config.frequency,
            smooth: config.smooth,

            color: config.color,
            openings: vec![],
        }
    }
    pub fn scroll(&mut self, area: Rect) {
        self.offset_x += self.speed_x;
        self.calculate_openings(area);
    }

    pub fn set_speed_x(&mut self, speed: i64) {
        self.speed_x = speed;
    }

    fn calculate_openings(&mut self, area: Rect) {
        self.openings = Vec::new();

        let noise_base_x = area.x as f64 + self.offset_x as f64;

        let mut perlin_center = Perlin::new(self.seed, true);
        perlin_center.set_interval(0.2, 0.8);

        let mut perlin_opening = Perlin::new(self.seed + 25.0, true);
        perlin_opening.set_interval(self.opening_min, self.opening_ratio);

        let mut perlin_t = Perlin::new(self.seed + 30.0, true);
        perlin_t.set_interval(-1.0, 1.0);
        let mut perlin_b = Perlin::new(self.seed + 40.0, true);
        perlin_b.set_interval(-1.0, 1.0);

        let frequency = self.frequency;
        let smooth = self.smooth;

        for (_, x_pos) in (area.left()..area.right()).enumerate() {
            let x = x_pos as f64 + noise_base_x;

            let noise_center = perlin_center.noise2d(x / frequency, 0.0);
            let noise_opening = perlin_opening.noise2d(x / smooth, 2.0);
            let noise_t = perlin_t.noise2d(x / smooth, 3.0) * 10.0;
            let noise_b = perlin_b.noise2d(x / smooth, 4.0) * 10.0;

            let center = area.height as f64 * noise_center;
            let opening = area.height as f64 * noise_opening;
            let t = noise_t as u16;
            let b = noise_b as u16;

            /*
            -
            | Solid
            | Solid
            - center - opening / 2
            |
            - center
            |
            - center + opening / 2
            | Solid
            | Solid
            -
            */
            let top = (center - (opening / 2.0)) as u16;
            let bot = (center + (opening / 2.0)) as u16;
            self.openings.push((top + t, bot - b));
        }
    }
}

pub struct CaveWidget<'a> {
    color: Color,
    openings: &'a Vec<(u16, u16)>,
}
impl<'a> CaveWidget<'a> {
    pub fn new(color: Color, openings: &'a Vec<(u16, u16)>) -> Self {
        Self { color, openings }
    }
}
impl Widget for CaveWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (_, y_pos) in (area.top()..area.bottom()).enumerate() {
            for (_, x_pos) in (area.left()..area.right()).enumerate() {
                let (top, bot) = self.openings[x_pos as usize];
                if y_pos < top || y_pos > bot {
                    buf.get_mut(x_pos, y_pos).set_char('█').set_fg(self.color);
                }
            }
        }
    }
}
