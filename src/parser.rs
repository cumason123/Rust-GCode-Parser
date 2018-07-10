use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct parsable_file<TimeType, FileType, DistanceType> {
    pub file: FileType
}

pub fn <TimeType> parse_file(filename: &str) -> Option<TimeType> {
    let mut f = File::open(filename).unwrap();
    let mut file = BufReader::new(&f);
    let mut line = String::new();

    while file.read_line(&mut line).unwrap() > 0 {
        println!("{}", line);
    }
    return 1f32;
}

fn parse_line(line: &str) {

}

pub struct Linear {
    pub point: (DistanceType, DistanceType, DistanceType),
    pub prev: Option<(DistanceType, DistanceType, DistanceType)>,
    pub feedrate: f64, // meters/second
    pub accelerator_weight: Option<f64>
}

impl <TimeType, FileType, DistanceType> Linear <TimeType, FileType, DistanceType> {
    pub fn distance_from(&self, other: (DistanceType, DistanceType, DistanceType)) -> DistanceType {
        ((self.point.0-other.0).powi(2) + (self.point.1-other.1).powi(2) + (self.point.2-other.2).powi(2))
        .sqrt()
    },

    pub fn calc_time(self) -> TimeType {
        let mut a: TimeType = 1;
        match self.accelerator_weight {
            Some(weight) => {
                a = weight;
            },
            _ => {
                -1
            }
        };
        match self.prev {
            Some(previous_point) => {
                1/((self.feedrate * a) / self.point.distance_from(previous_point))
            },
            _ => {
                1/((self.feedrate * a) / self.point.distance_from())
            }
        }
    }
}
