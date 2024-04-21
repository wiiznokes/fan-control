use hardware::Value;

#[derive(Debug)]
pub struct Affine {
    pub xa: f32,
    pub ya: f32,
    pub xb: f32,
    pub yb: f32,
}

impl Affine {
    pub fn calcule(&self, value: Value) -> f32 {
        let a = (self.yb - self.ya) / (self.xb - self.xa);
        let b = self.ya - a * self.xa;

        a * value as f32 + b
    }
}
