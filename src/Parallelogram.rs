
pub struct Parallelogram{
    side_a: f64,
    side_b: f64,
    angle:f64,
}

impl Parallelogram{
    pub fn area(&self) -> f64{
        self.side_a * self.side_b * self.angle.sin()
    }
    pub fn perimeter(&self) -> f64{
        2.0 * (self.side_a + self.side_b)
    }
    pub fn diagonal(&self) -> f64{
        (self.side_a.powf(2.0) + self.side_b.powf(2.0) - 2.0 * self.side_a * self.side_b * self.angle.cos() ).sqrt()
    }
}
