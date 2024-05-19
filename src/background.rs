use perlin_rust::PerlinNoise;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols;
use ratatui::widgets::canvas::{Canvas, Context, Points};
use ratatui::widgets::{Block, Borders};

pub struct Background {
    pub seed: f64,
    pub offset_x: i64,
    pub offset_y: i64,
    pub speed_x: i64,
    pub speed_y: i64,
    pub resolution: i64,
}

impl Background {
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

    fn generate_background(&self, area: Rect) -> Vec<(f64, f64, u8)> {
        let perlin = PerlinNoise::new(self.seed);

        let full_area = (area.width * area.height) as usize;
        let noise_base_x = area.x as f64 + self.offset_x as f64 + 1.0E+9f64;
        let noise_base_y = area.y as f64 + self.offset_y as f64 + 1.0E+9f64;

        let divider = (256 / self.resolution) as u8;

        let mut pixels = vec![];

        for n in 0..full_area {
            let x_pos = (n % area.width as usize) as f64;
            let y_pos = (n / area.width as usize) as f64;

            let x = x_pos + noise_base_x;
            let y = y_pos + noise_base_y;

            let noise = perlin.perlin2(x / 25.0, y / 25.0);
            let color = ((noise + 1.0) / 2.0 * 255.0) as u8;

            pixels.push((x_pos, y_pos, color / divider));
        }

        pixels
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

    pub fn render(&self, area: Rect) -> Canvas<impl Fn(&mut Context)> {
        let noise = self.generate_background(area);
        let res = self.resolution;

        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Move: ← ({}) → / Exit: (q) ", self.speed_x)),
            )
            .x_bounds([0.0, area.width as f64])
            .y_bounds([0.0, area.height as f64])
            .marker(symbols::Marker::Block)
            .paint(move |ctx| {
                ctx.layer();
                for n in 0..res {
                    let coords: Vec<(f64, f64)> = noise
                        .iter()
                        .map(|v| *v)
                        .filter(|(_, _, c)| *c == n as u8)
                        .map(|(x, y, _)| (x, y))
                        .collect::<Vec<(f64, f64)>>();

                    if coords.is_empty() {
                        continue;
                    }

                    let atten = (256 / res / 4) as f64;
                    let grey = (n as f64) * atten;
                    let points = Points {
                        coords: &coords,
                        color: Color::Rgb(grey as u8, grey as u8, grey as u8),
                    };

                    ctx.draw(&points);
                }
            })
    }
}
