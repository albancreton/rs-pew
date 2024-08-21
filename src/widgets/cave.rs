use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::noise::Perlin;

pub struct CaveConfig {
    pub opening_max: f64,
    pub opening_min: f64,
    pub frequency: f64,
    pub smooth: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct Cave {
    pub seed: f64,
    pub offset_x: i64,
    pub speed_x: i64,

    opening_ratio: f64,
    opening_min: f64,
    frequency: f64,
    smooth: f64,

    pub color: Color,
    pub pixels: Vec<(u16, u16)>,
}

impl Cave {
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
            pixels: vec![],
        }
    }
    pub fn scroll(&mut self, area: Rect) {
        self.offset_x += self.speed_x;
        self.calculate_pixels(area);
    }

    pub fn set_speed_x(&mut self, speed: i64) {
        self.speed_x = speed;
    }

    pub fn has_pixel(&self, x: u16, y: u16) -> bool {
        find(&(self.pixels), x, y)
    }

    fn calculate_pixels(&mut self, area: Rect) {
        self.pixels = Vec::new();
        let noise_base_x = area.x as f64 + self.offset_x as f64;

        let mut perlin_top = Perlin::new(self.seed, true);
        perlin_top.set_interval(0.2, 0.8);

        let mut perlin_bot = Perlin::new(self.seed + 25.0, true);
        perlin_bot.set_interval(self.opening_min, self.opening_ratio);

        let frequency = self.frequency;
        let smooth = self.smooth;

        for (_, x_pos) in (area.left()..area.right()).enumerate() {
            let x = x_pos as f64 + noise_base_x;

            let noise_center = perlin_top.noise2d(x / frequency, 0.0);
            let noise_opening = perlin_bot.noise2d(x / smooth, 2.0);

            let center = area.height as f64 * noise_center;
            let opening = area.height as f64 * noise_opening;

            let top = (center - (opening / 2.0)) as u16;
            let bot = (center + (opening / 2.0)) as u16;

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
            for (_, y_pos) in (area.top()..area.bottom()).enumerate() {
                if y_pos < top || y_pos > bot {
                    self.pixels.push((x_pos, y_pos));
                }
            }
        }
    }
}

pub struct CaveWidget<'a> {
    color: Color,
    pixels: &'a Vec<(u16, u16)>,
}
impl<'a> CaveWidget<'a> {
    pub fn new(color: Color, pixels: &'a Vec<(u16, u16)>) -> Self {
        Self { color, pixels }
    }
}
impl Widget for CaveWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // for (x_pos, y_pos) in self.pixels.iter() {
        //     let x: u16 = *x_pos;
        //     let y: u16 = *y_pos;
        //     if x < area.height - 1 && y < area.width - 1 {
        //         buf.get_mut(x, y).set_char('█').set_fg(self.color);
        //     }
        // }
        for (_, y_pos) in (area.top()..area.bottom()).enumerate() {
            for (_, x_pos) in (area.left()..area.right()).enumerate() {
                if find(self.pixels, x_pos, y_pos) {
                    buf.get_mut(x_pos, y_pos).set_char('█').set_fg(self.color);
                }
            }
        }
    }
}

fn find(v: &Vec<(u16, u16)>, x: u16, y: u16) -> bool {
    for (x_pos, y_pos) in v.iter() {
        if *x_pos == x && *y_pos == y {
            return true;
        }
    }
    return false;
}
