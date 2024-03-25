use std::f64::consts::PI;

pub struct Circle{
    pub radius: f64,
    pub angle: f64
}
//TODO assign modes as Length and Volume / area
impl Circle{
    pub fn cal_circle_length(&self) -> f64{
        2.0 * PI * self.radius
    }

    pub fn cal_diameter(&self) -> f64{
        self.radius * 2.0
    }

    pub fn  cal_sectors_area(&self) -> f64{
        (1.0 * self.angle * self.radius.powf(2.0))/2.0
    }

    pub fn cal_segments_area(&self) -> f64{
        1.0 * (self.angle - self.angle.sin()) * self.radius.powf(2.0)
    }
}

