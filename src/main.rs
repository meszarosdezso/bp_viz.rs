mod constants;
mod helpers;
mod meta;
mod vizualizations;

// update the module name for different vizualizations
//                  ▼
use vizualizations::stops as viz;

fn main() {
    use viz::{model, update, view};

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(constants::WIDTH, constants::HEIGHT)
        .run();
}
