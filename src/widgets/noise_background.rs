use palette::{FromColor, Hsl, IntoColor, Srgb};
use perlin_rust::PerlinNoise;
use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::noise::Perlin;

#[derive(Clone, Copy)]
pub struct NoiseBackground {
    pub seed: f64,
    pub offset_x: i64,
    pub offset_y: i64,
    pub speed_x: i64,
    pub speed_y: i64,
    pub resolution: i64,
}

impl NoiseBackground {
    pub fn new(seed: f64) -> Self {
        Self {
            seed,
            offset_x: 0,
            offset_y: 0,
            speed_x: 0,
            speed_y: 0,
            resolution: 16,
        }
    }

    pub fn scroll(&mut self) {
        self.offset_x += self.speed_x;
        self.offset_y += self.speed_y;
    }

    pub fn set_speed_x(&mut self, speed: i64) {
        self.speed_x = speed;
    }
    pub fn set_speed_y(&mut self, speed: i64) {
        self.speed_y = speed;
    }

    pub fn set_speed(&mut self, speedx: i64, speedy: i64) {
        self.speed_x = speedx;
        self.speed_y = speedy;
    }
}

impl Widget for NoiseBackground {
    #[allow(clippy::cast_precision_loss, clippy::similar_names)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut perlin = Perlin::new(self.seed, true);
        perlin.set_interval(0.0, 255.0);

        let noise_base_x = area.x as f64 + self.offset_x as f64;
        let noise_base_y = area.y as f64 + self.offset_y as f64;
        let res_devider = (256 / self.resolution) as u8;

        for (_, y_pos) in (area.top()..area.bottom()).enumerate() {
            for (_, x_pos) in (area.left()..area.right()).enumerate() {
                let x = x_pos as f64 + noise_base_x;
                let y = y_pos as f64 + noise_base_y;

                let noise = perlin.noise2d(x / 25.0, y / 25.0);
                let color = (noise / (res_devider as f64)) as u8 * res_devider;

                let mut hsl: Hsl = Srgb::new(
                    color as f32 / 255.0,
                    color as f32 / 255.0,
                    color as f32 / 255.0,
                )
                .into_color();
                hsl.lightness = hsl.lightness / 2.5;
                let my_new_rgb = Srgb::from_color(hsl);

                let fg = Color::Rgb(
                    (my_new_rgb.red * 255.0) as u8,
                    (my_new_rgb.green * 255.0) as u8,
                    (my_new_rgb.blue * 255.0) as u8,
                );

                buf.get_mut(x_pos as u16, y_pos as u16)
                    .set_char('▓')
                    .set_fg(fg)
                    .set_bg(Color::Black);
            }
        }
    }
}
