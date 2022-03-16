mod constants;
mod piano;
mod utils;
mod vizualizations;

// update the module name for different vizualizations
//                  ▼
use vizualizations::audio as viz;

fn main() {
    use viz::{event, model, update, view};

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .event(event)
        .size(constants::CANVAS_WIDTH, constants::CANVAS_HEIGHT)
        .run();
}
