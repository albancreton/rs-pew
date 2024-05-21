use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::noise::Perlin;

pub struct CaveConfig {
    pub opening_max: f64,
    pub opening_min: f64,
    pub frequency: f64,
    pub smooth: f64,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Cave {
    pub seed: f64,
    pub offset_x: i64,
    pub speed_x: i64,

    opening_ratio: f64,
    opening_min: f64,
    frequency: f64,
    smooth: f64,

    color: Color,
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
        }
    }
    pub fn scroll(&mut self) {
        self.offset_x += self.speed_x;
    }

    pub fn set_speed_x(&mut self, speed: i64) {
        self.speed_x = speed;
    }
}

impl Widget for Cave {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
                    buf.get_mut(x_pos, y_pos)
                        .set_char('█')
                        .set_fg(self.color);
                }
            }
        }
    }
}
