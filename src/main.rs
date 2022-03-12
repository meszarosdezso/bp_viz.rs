mod constants;
mod utils;
mod vizualizations;

// update the module name for different vizualizations
//                  ▼
use vizualizations::trips as viz;

fn main() {
    use viz::{event, model, update, view};

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .event(event)
        .size(constants::CANVAS_WIDTH, constants::CANVAS_HEIGHT)
        .run();
}
