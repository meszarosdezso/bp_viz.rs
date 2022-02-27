#![allow(dead_code)]

use nannou::{color, event::Update, prelude::pt2, App, Frame, LoopMode};

use crate::constants::{GTFS_URL, HEIGHT, WIDTH};
use crate::helpers::coordinate_to_xy;
use crate::meta::Meta;

use super::Model;

const TRIP_ID: &'static str = "C32177100";

pub fn model(app: &App) -> Model<()> {
    app.set_loop_mode(LoopMode::loop_ntimes(1));
    Model::from_url(GTFS_URL)
}

pub fn update(_app: &App, _model: &mut Model<()>, _update: Update) {}

pub fn view(app: &App, model: &Model<()>, frame: Frame) {
    let draw = app.draw();
    let color = color::SALMON;

    let trip = model.gtfs.trips.get(TRIP_ID).unwrap();
    let shape = model
        .gtfs
        .shapes
        .get(trip.shape_id.as_ref().unwrap())
        .unwrap();

    let meta = Meta::from_trip(trip);

    let poli = shape.iter().map(|s| {
        let (x, y) = coordinate_to_xy(s.longitude, s.latitude, &meta);
        (pt2(x as f32, y as f32), color)
    });

    draw.polyline()
        .weight(2.0)
        .join_round()
        .points_colored(poli);

    for st in trip.stop_times.iter() {
        let (x, y) = coordinate_to_xy(
            st.stop.longitude.unwrap(),
            st.stop.latitude.unwrap(),
            &model.meta,
        );

        draw.ellipse()
            .stroke(color)
            .stroke_weight(2.)
            .color(color::BLACK)
            .radius(8.)
            .x_y(x as f32, y as f32);
    }

    let first = &trip.stop_times.first().unwrap().stop;
    let last = &trip.stop_times.last().unwrap().stop;

    draw.text(&format!("{} ► {}", last.name, first.name))
        .x_y(0., -((HEIGHT / 2 - 50) as f32))
        .color(color)
        .font_size(12)
        .w((WIDTH - 100) as f32)
        .left_justify();

    app.main_window()
        .capture_frame(format!("./export/trips/trip_{}.png", TRIP_ID));

    draw.to_frame(app, &frame).unwrap();
}
