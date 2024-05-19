use perlin_rust::PerlinNoise;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

pub struct CaveConfig {
    pub opening_ratio: f64,
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
            seed: seed,

            offset_x: 0,
            speed_x: 0,

            opening_ratio: config.opening_ratio,
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
        let noise_base_x = area.x as f64 + self.offset_x as f64 + 1.0E+9f64;
        let perlin_top = PerlinNoise::new(self.seed);
        let perlin_bot = PerlinNoise::new(self.seed + 25.0);

        let opening_ratio = self.opening_ratio;
        let opening_min = self.opening_min;
        let frequency = self.frequency;
        let smooth = self.smooth;

        for (_, x_pos) in (area.left()..area.right()).enumerate() {
            let x = x_pos as f64 + noise_base_x;

            let noise_center = perlin_top.perlin2(x / frequency, 0.0);
            let noise_opening = perlin_bot.perlin2(x / smooth, 2.0);

            let normal_noise_center = 0.1 + ((noise_center + 1.0) / 2.0) * 0.9;
            let normal_noise_opening =
                (1.0 - opening_ratio) / 2.0 + ((noise_opening + 1.0) / 2.0) * opening_ratio;

            let center = area.height as f64 * normal_noise_center;
            let mut opening = area.height as f64 * normal_noise_opening;
            if opening < opening_min * area.height as f64 {
                opening = opening_min * area.height as f64;
            }

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
                    buf.get_mut(x_pos as u16, y_pos as u16)
                        .set_char('█')
                        .set_fg(self.color);
                }
            }
        }
    }
}
