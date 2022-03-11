use gtfs_structures::{Gtfs, Trip};

use crate::utils::boundaries::Boundaries;

#[derive(Debug, Default)]
pub struct Meta {
    pub boundaries: Boundaries,
    pub width: f64,
    pub height: f64,
}

impl Meta {
    pub fn from_gtfs(gtfs: &Gtfs) -> Self {
        let coords = gtfs
            .stops
            .iter()
            .map(|(_, s)| (s.longitude.unwrap(), s.latitude.unwrap()));

        let boundaries = Boundaries::from_coords(coords);
        let (width, height) = boundaries.canvas_size();

        Self {
            boundaries,
            width,
            height,
        }
    }

    pub fn from_trip(trip: &Trip) -> Self {
        let coords = trip
            .stop_times
            .iter()
            .map(|s| (s.stop.longitude.unwrap(), s.stop.latitude.unwrap()));

        let boundaries = Boundaries::from_coords(coords);
        let (width, height) = boundaries.canvas_size();

        Self {
            boundaries,
            width,
            height,
        }
    }
}
