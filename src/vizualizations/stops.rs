#![allow(dead_code)]

use gtfs_structures::{Gtfs, Stop};
use nannou::color;
use nannou::event::Update;
use nannou::{App, Frame, LoopMode};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::Model;
use crate::constants::GTFS_URL;
use crate::utils::math::{coordinate_to_xy, distance};
use crate::utils::meta::Meta;

const START: &'static str = "088453";

#[derive(Default, Debug)]
pub struct StopsContext {
    start: Arc<Stop>,
    nexts: Vec<Arc<Stop>>,
    visited: HashSet<String>,
}

impl StopsContext {
    fn from_gtfs(gtfs: &Gtfs, start_id: &str) -> Self {
        let start = Arc::clone(&gtfs.stops.get(start_id).unwrap());

        let mut visited = HashSet::new();
        visited.insert(String::from(start_id));

        Self {
            start,
            nexts: vec![],
            visited,
        }
    }
}

pub fn model(app: &App) -> Model<StopsContext> {
    let model = Model::from_url(GTFS_URL)
        .meta(|model| Meta::from_gtfs(&model.gtfs))
        .context(|model| StopsContext::from_gtfs(&model.gtfs, START));

    app.set_loop_mode(LoopMode::loop_ntimes(model.gtfs.stops.len()));

    model
}

pub fn update(_app: &App, model: &mut Model<StopsContext>, _update: Update) {
    let mut nexts = vec![];
    for _ in 0..20 {
        let (id, next) = find_closest(
            &model.context.start,
            &model.gtfs.stops,
            &mut model.context.visited,
        );

        model.context.visited.insert(id.clone());
        nexts.push(next);
    }
    model.context.nexts = nexts;
}

pub fn view(app: &App, model: &Model<StopsContext>, frame: Frame) {
    if frame.nth() == 0 {
        frame.clear(color::BLACK);
    }

    let draw = app.draw();

    for stop in model.context.nexts.iter() {
        let (x, y) = coordinate_to_xy(stop.longitude.unwrap(), stop.latitude.unwrap(), &model.meta);

        draw.ellipse()
            .x_y(x as f32, y as f32)
            .radius(1.)
            .color(color::WHITE);
    }

    // app.main_window()
    //     .capture_frame(format!("./export/stops/frame_{}.png", frame.nth()));

    if model.context.visited.len() >= model.gtfs.stops.len() {
        app.quit();
    }

    draw.to_frame(app, &frame).unwrap();
}

fn find_closest<'a>(
    start: &Stop,
    stops: &'a HashMap<String, Arc<Stop>>,
    visited: &mut HashSet<String>,
) -> (&'a String, Arc<Stop>) {
    let (mut winner_id, mut winner) = stops.iter().next().unwrap();
    let mut record = 5.;

    for (id, stop) in stops.iter() {
        if !visited.contains(id) {
            let x0 = start.longitude.unwrap();
            let y0 = start.latitude.unwrap();
            let x1 = stop.longitude.unwrap();
            let y1 = stop.latitude.unwrap();

            let dist = distance(x0, y0, x1, y1);

            if dist < record {
                winner = stop;
                winner_id = id;
                record = dist;
            }
        }
    }

    (winner_id, Arc::clone(winner))
}
