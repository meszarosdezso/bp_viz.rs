use crate::constants::{CANVAS_HEIGHT, CANVAS_WIDTH};

/// Earth boundaries.
#[derive(Debug)]
pub struct Boundaries {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}

impl Boundaries {
    pub fn from_coords(coords: impl Iterator<Item = (f64, f64)>) -> Self {
        let mut min_lat = 90.;
        let mut max_lat = -90.;
        let mut min_lng = 180.;
        let mut max_lng = -180.;

        for (longitude, latitude) in coords {
            if latitude < min_lat {
                min_lat = latitude;
            }

            if latitude > max_lat {
                max_lat = latitude;
            }

            if longitude < min_lng {
                min_lng = longitude;
            }

            if longitude > max_lng {
                max_lng = longitude;
            }
        }

        Self {
            min_lng,
            max_lng,
            min_lat,
            max_lat,
        }
    }

    pub fn width(&self) -> f64 {
        self.max_lng - self.min_lng
    }

    pub fn height(&self) -> f64 {
        self.max_lat - self.min_lat
    }

    pub fn size(&self) -> (f64, f64) {
        (self.width(), self.height())
    }

    pub fn canvas_size(&self) -> (f64, f64) {
        let (earth_w, earth_h) = self.size();

        let scale = if earth_h > earth_w {
            CANVAS_HEIGHT as f64 / earth_h
        } else {
            CANVAS_WIDTH as f64 / earth_w
        };

        let width = earth_w * scale - 150.;
        let height = earth_h * scale - 50.;

        (width, height)
    }
}

impl Default for Boundaries {
    fn default() -> Self {
        Self {
            max_lat: 90.,
            min_lat: -90.,
            max_lng: 180.,
            min_lng: -180.,
        }
    }
}
