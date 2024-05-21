use perlin_rust::PerlinNoise;

pub struct Perlin {
    output_min: f64,
    output_max: f64,

    x_offset: f64,
    y_offset: f64,

    perlin: PerlinNoise,
}

impl Perlin {
    pub fn new(seed: f64, should_center: bool) -> Self {
        let x_offset = if should_center { 0.0 } else { 1.0E+9f64 };
        let y_offset = if should_center { 0.0 } else { 1.0E+9f64 };

        Self {
            output_min: -1.0,
            output_max: 1.0,
            x_offset,
            y_offset,
            perlin: PerlinNoise::new(seed),
        }
    }

    pub fn set_interval(&mut self, min: f64, max: f64) {
        self.output_min = min;
        self.output_max = max;
    }

    pub fn noise2d(&self, x: f64, y: f64) -> f64 {
        let noise = self.perlin.perlin2(x + self.x_offset, y + self.y_offset);
        self.normalize(noise)
    }

    fn normalize(&self, val: f64) -> f64 {
        (val + 1.0) / 2.0 * (self.output_max - self.output_min) + self.output_min
    }
}
