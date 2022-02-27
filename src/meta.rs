use gtfs_structures::{Gtfs, Trip};

use crate::helpers::{calc_boundaries, calc_canvas_size};

#[derive(Debug, Default)]
pub struct Meta {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lng: f64,
    pub max_lng: f64,
    pub width: f64,
    pub height: f64,
}

impl Meta {
    pub fn from_gtfs(gtfs: &Gtfs) -> Self {
        let coords = gtfs
            .stops
            .iter()
            .map(|(_, s)| (s.longitude.unwrap(), s.latitude.unwrap()));

        let (min_lng, max_lng, min_lat, max_lat) = calc_boundaries(coords);
        let (width, height) = calc_canvas_size(min_lng, max_lng, min_lat, max_lat);

        Self {
            min_lat,
            max_lat,
            min_lng,
            max_lng,
            width,
            height,
        }
    }

    pub fn from_trip(trip: &Trip) -> Self {
        let coords = trip
            .stop_times
            .iter()
            .map(|s| (s.stop.longitude.unwrap(), s.stop.latitude.unwrap()));

        let (min_lng, max_lng, min_lat, max_lat) = calc_boundaries(coords);
        let (width, height) = calc_canvas_size(min_lng, max_lng, min_lat, max_lat);

        Self {
            min_lat,
            max_lat,
            min_lng,
            max_lng,
            width,
            height,
        }
    }
}
