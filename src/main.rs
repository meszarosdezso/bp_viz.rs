mod constants;
mod piano;
mod utils;
mod vizualizations;

use vizualizations::audio::AudioViz;
use vizualizations::stops::StopsViz;
use vizualizations::trips::TripsViz;
use vizualizations::Vizualization;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let viz_type = args.get(1);

    let viz: Box<dyn Vizualization> = if let Some(s) = viz_type {
        match s.as_str() {
            "stops" => Box::new(StopsViz::default()),
            "trips" => Box::new(TripsViz::new()),
            "audio" => Box::new(AudioViz::default()),
            _ => Box::new(TripsViz::new()),
        }
    } else {
        Box::new(TripsViz::new())
    };

    viz.run();
}
