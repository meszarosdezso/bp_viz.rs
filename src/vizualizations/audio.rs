use gtfs_structures::Gtfs;
use nannou::event::Update;
use nannou::math::map_range;
use nannou::{color, App, Event, Frame};
use rodio::source::SineWave;
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use std::sync::Arc;
use std::time::Duration;

use crate::constants::{CANVAS_WIDTH, GTFS_URL};
use crate::piano::Piano;
use crate::utils::meta::Meta;

use super::Model;

const NOTES: [&'static str; 9] = [
    "C#4/Db4", "D4", "E4", "F4", "G4", "A4", "A#4/Bb4", "C#5/Db5", "D5",
];

#[derive(Default)]
pub struct AudioContext {
    piano: Arc<Piano>,
}

pub fn model(app: &App) -> Model<AudioContext> {
    let model = Model::from_url(GTFS_URL)
        .meta(|model| Meta::from_gtfs(&model.gtfs))
        .context(|_| AudioContext {
            piano: Arc::new(Piano::new()),
        });

    app.set_loop_mode(nannou::LoopMode::loop_ntimes(model.gtfs.stops.len()));

    music(&model.gtfs);

    model
}

pub fn event(_app: &App, _model: &mut Model<AudioContext>, _event: Event) {}

pub fn update(_app: &App, _model: &mut Model<AudioContext>, _update: Update) {}

pub fn view(app: &App, model: &Model<AudioContext>, frame: Frame) {
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

fn music(gtfs: &Gtfs) {
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
