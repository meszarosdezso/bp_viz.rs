use nannou::math;

use crate::utils::meta::Meta;

pub fn coordinate_to_xy(lng: f64, lat: f64, meta: &Meta) -> (f64, f64) {
    let x = math::map_range(
        lng,
        meta.boundaries.min_lng,
        meta.boundaries.max_lng,
        -meta.width / 2.,
        meta.width / 2.,
    );

    let y = math::map_range(
        lat,
        meta.boundaries.min_lat,
        meta.boundaries.max_lat,
        -meta.height / 2.,
        meta.height / 2.,
    );

    (x, y)
}

pub fn distance(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt()
}
