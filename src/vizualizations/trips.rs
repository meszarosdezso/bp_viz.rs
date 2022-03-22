use gtfs_structures::Trip;
use nannou::app::EventFn;
use nannou::app::ModelFn;
use nannou::app::UpdateFn;
use nannou::app::ViewFn;
use nannou::color::Rgb8;
use nannou::event::Key;
use nannou::event::WindowEvent;
use nannou::prelude::pt2;
use nannou::Event;
use nannou::LoopMode;
use rand::Rng;
use std::sync::Arc;

use crate::constants::CANVAS_HEIGHT;
use crate::constants::CANVAS_WIDTH;
use crate::constants::GTFS_URL;
use crate::utils::math::coordinate_to_xy;
use crate::utils::meta::Meta;

use super::Model;
use super::Viz;
use super::Vizualization;

#[derive(Default)]
pub struct TripsViz {
    pub history: Vec<Arc<Trip>>,
    pub light: bool,
}

impl TripsViz {
    pub fn new() -> Self {
        Self {
            history: vec![],
            light: false,
        }
    }
}

impl Model<Box<TripsViz>> {
    fn new_trip(&mut self) {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.gtfs.trips.len());
        let id = self.gtfs.trips.keys().nth(idx).unwrap();
        let trip = Arc::new(self.gtfs.get_trip(id).unwrap().clone());

        self.meta = Meta::from_trip(&trip);
        self.context.history.push(Arc::clone(&trip));
    }

    fn back(&mut self) {
        self.context.history.pop();
        self.meta = Meta::from_trip(self.context.history.last().unwrap());
    }
}

impl Viz for TripsViz {
    fn model(&self) -> ModelFn<Model<Box<Self>>> {
        |app| {
            app.set_loop_mode(LoopMode::Wait);

            let mut model = Model::from_url(GTFS_URL).context(|_| Box::new(Self::new()));

            model.new_trip();

            model
        }
    }

    fn update(&self) -> UpdateFn<Model<Box<Self>>> {
        |_, _, _| {}
    }

    fn event(&self) -> EventFn<Model<Box<Self>>, Event> {
        |app, model, event| match event {
            Event::WindowEvent { simple, .. } => {
                if let Some(event) = simple {
                    match event {
                        WindowEvent::KeyPressed(code) if code == Key::Back => {
                            if model.context.history.len() > 1 {
                                model.back();
                            }
                        }
                        WindowEvent::ReceivedCharacter(c) => match c {
                            ' ' => {
                                model.new_trip();
                            }
                            'r' => {
                                if let Some(trip) = model.context.history.last() {
                                    let filename = format!("./export/trips/trip_{}.png", trip.id);
                                    app.main_window().capture_frame(filename);
                                }
                            }
                            'i' => {
                                model.context.light = !model.context.light;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    fn view(&self) -> ViewFn<Model<Box<Self>>> {
        |app, model, frame| {
            let draw = app.draw();

            let trip = model.context.history.last().unwrap();
            let route = model.gtfs.get_route(&trip.route_id).unwrap();
            let shape = model
                .gtfs
                .shapes
                .get(trip.shape_id.as_ref().unwrap())
                .unwrap();

            let color = {
                let color = route.route_color;
                Rgb8::new(color.r, color.g, color.b)
            };

            // TODO nicer bg color calculation
            let darken_amount = if !model.context.light { 6. } else { 0.02 };
            let bg_color = Rgb8::new(
                (color.red as f32 / darken_amount) as u8,
                (color.green as f32 / darken_amount) as u8,
                (color.blue as f32 / darken_amount) as u8,
            );

            draw.background().color(bg_color);

            let poli = shape.iter().map(|s| {
                let (x, y) = coordinate_to_xy(s.longitude, s.latitude, &model.meta);
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
                    .color(bg_color)
                    .radius(8.)
                    .x_y(x as f32, y as f32);
            }

            let first = &trip.stop_times.first().unwrap().stop;
            let last = &trip.stop_times.last().unwrap().stop;

            draw.text(&format!("{} ► {}", last.name, first.name))
                .x_y(0., -((CANVAS_HEIGHT / 2 - 50) as f32))
                .color(color)
                .font_size(16)
                .w((CANVAS_WIDTH - 100) as f32)
                .left_justify();

            draw.text(&route.short_name)
                .x_y(0., (CANVAS_HEIGHT / 2 - 50) as f32)
                .color(color)
                .font_size(32)
                .w((CANVAS_WIDTH - 100) as f32)
                .right_justify();

            draw.to_frame(app, &frame).unwrap();
        }
    }
}

impl Vizualization for TripsViz {
    fn run(&self) {
        nannou::app(self.model())
            .event(self.event())
            .simple_window(self.view())
            .size(CANVAS_WIDTH, CANVAS_HEIGHT)
            .run()
    }
}
