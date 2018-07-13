use parsing_error::ParsingError;
pub struct Linear {
    pub point: (f32, f32, f32),
    pub prev: Option<(f32, f32, f32)>,
    pub feedrate: f32, // meters/second
    pub accelerator_weight: Option<f32>
}

pub fn distance_from(first: (f32, f32, f32), second: (f32, f32, f32)) -> f32 {
    ((first.0 - second.0).powi(2) + (first.1 - second.1).powi(2) + (first.2 - second.2).powi(2))
    .sqrt()
}

impl Linear {
    pub fn calc_time(self) -> u32 {
        let mut a = 1f32;
        match self.accelerator_weight {
            Some(weight) => {
                a = weight;
            },
            None => {

            }
        };
        match self.prev {
            Some(previous_point) => {
                1/((self.feedrate * a) / distance_from(self.point, previous_point)) as u32
            },
            None => {
                1/((self.feedrate * a) / distance_from(self.point, (0f32, 0f32, 0f32))) as u32
            }
        }
    }
}
