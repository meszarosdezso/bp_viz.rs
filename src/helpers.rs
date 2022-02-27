use nannou::math;

use crate::{
    constants::{HEIGHT, WIDTH},
    meta::Meta,
};

pub fn calc_boundaries(coords: impl Iterator<Item = (f64, f64)>) -> (f64, f64, f64, f64) {
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

    (min_lng, max_lng, min_lat, max_lat)
}

pub fn calc_canvas_size(min_lng: f64, max_lng: f64, min_lat: f64, max_lat: f64) -> (f64, f64) {
    let earth_w = max_lng - min_lng;
    let earth_h = max_lat - min_lat;

    let scale = if earth_h > earth_w {
        HEIGHT as f64 / earth_h
    } else {
        WIDTH as f64 / earth_w
    };

    let width = earth_w * scale - 150.;
    let height = earth_h * scale - 50.;

    (width, height)
}

pub fn coordinate_to_xy(lng: f64, lat: f64, meta: &Meta) -> (f64, f64) {
    let x = math::map_range(
        lng,
        meta.min_lng,
        meta.max_lng,
        -meta.width / 2.,
        meta.width / 2.,
    );

    let y = math::map_range(
        lat,
        meta.min_lat,
        meta.max_lat,
        -meta.height / 2.,
        meta.height / 2.,
    );

    (x, y)
}

pub fn distance(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()
}
