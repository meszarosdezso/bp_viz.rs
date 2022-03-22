use gtfs_structures::Gtfs;
use nannou::color;
use nannou::math::map_range;
use nannou::Event;
use rodio::dynamic_mixer;
use rodio::source::SineWave;
use rodio::OutputStream;
use rodio::Sink;
use rodio::Source;
use std::sync::Arc;
use std::time::Duration;

use crate::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GTFS_URL};
use crate::piano::Piano;

use super::Model;
use super::Viz;
use super::Vizualization;

const NOTES: [&'static str; 9] = [
    "C#4/Db4", "D4", "E4", "F4", "G4", "A4", "A#4/Bb4", "C#5/Db5", "D5",
];

#[derive(Default)]
pub struct AudioViz {
    piano: Arc<Piano>,
}

impl Model<Box<AudioViz>> {
    fn music(&self, gtfs: &Gtfs) {
        let piano = Piano::new();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        for (_, stop) in &gtfs.stops {
            let keys = stop
                .name
                .split(" ")
                .map(|part| NOTES[part.len() % NOTES.len()])
                .collect::<Vec<&'static str>>();

            let (ctrl, mixer) = dynamic_mixer::mixer::<f32>(2, 44100);

            for note in keys.into_iter() {
                if let Some(note) = piano.keys.get(note) {
                    let source = SineWave::new(note.freq)
                        .take_duration(Duration::from_millis(120))
                        .amplify(0.18);

                    ctrl.add(source);
                }
            }

            sink.append(mixer);
        }

        sink.sleep_until_end();
        sink.detach();
    }
}

impl Viz for AudioViz {
    fn model(&self) -> nannou::app::ModelFn<Model<Box<Self>>> {
        |app| {
            let model = Model::from_url(GTFS_URL).context(|_| {
                Box::new(Self {
                    piano: Arc::new(Piano::new()),
                })
            });

            app.set_loop_mode(nannou::LoopMode::loop_ntimes(model.gtfs.stops.len()));

            model.music(&model.gtfs);

            model
        }
    }

    fn update(&self) -> nannou::app::UpdateFn<Model<Box<Self>>> {
        |_, _, _| {}
    }

    fn event(&self) -> nannou::app::EventFn<Model<Box<Self>>, Event> {
        |_, _, _| {}
    }

    fn view(&self) -> nannou::app::ViewFn<Model<Box<Self>>> {
        |app, model, frame| {
            let piano = Arc::clone(&model.context.piano);
            let draw = app.draw();
            frame.clear(color::BLACK);

            let stop_idx = frame.nth() as usize;
            let stop = model.gtfs.stops.values().nth(stop_idx).unwrap();

            let keys = stop
                .name
                .split(" ")
                .map(|part| NOTES[part.len() % NOTES.len()])
                .collect::<Vec<&'static str>>();

            for key in keys.iter() {
                let r = map_range(
                    NOTES.iter().position(|n| n == key).unwrap_or(0),
                    0,
                    8,
                    50,
                    200,
                );

                draw.ellipse()
                    .radius(r as f32)
                    .stroke_color(color::gray(0.1))
                    .stroke_weight(2.)
                    .no_fill();
            }

            draw.text(&keys.join(" "))
                .x_y(0., -50.)
                .color(color::WHITE)
                .font_size(24)
                .w(CANVAS_WIDTH as f32);

            draw.text(&stop.name)
                .x_y(0., 50.)
                .color(color::WHITE)
                .font_size(32)
                .w(CANVAS_WIDTH as f32);

            piano.press_keys(keys, Duration::from_millis(50));

            draw.to_frame(app, &frame).unwrap();
        }
    }
}

impl Vizualization for AudioViz {
    fn run(&self) {
        nannou::app(self.model())
            .simple_window(self.view())
            .size(CANVAS_WIDTH, CANVAS_HEIGHT)
            .run()
    }
}
