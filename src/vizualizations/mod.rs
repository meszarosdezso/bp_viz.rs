pub mod audio;
pub mod stops;
pub mod trips;

use gtfs_structures::Gtfs;
use nannou::app::EventFn;
use nannou::app::ModelFn;
use nannou::app::UpdateFn;
use nannou::app::ViewFn;
use nannou::Event;

use crate::utils::meta::Meta;

#[derive(Default)]
pub struct Model<Context> {
    meta: Meta,
    gtfs: Gtfs,
    context: Context,
}

impl<Context> Model<Context>
where
    Context: Default,
{
    fn from_url(url: &str) -> Self {
        eprintln!("Parsing GTFS at {url}...");
        let gtfs = Gtfs::new(url).expect("Failed to load GTFS");
        let meta = Meta::from_gtfs(&gtfs);

        Self {
            gtfs,
            meta,
            ..Default::default()
        }
    }

    fn context<F>(mut self, context_fn: F) -> Self
    where
        F: FnOnce(&Self) -> Context,
    {
        self.context = context_fn(&self);
        self
    }
}

pub trait Viz {
    fn model(&self) -> ModelFn<Model<Box<Self>>>;
    fn update(&self) -> UpdateFn<Model<Box<Self>>>;
    fn event(&self) -> EventFn<Model<Box<Self>>, Event>;
    fn view(&self) -> ViewFn<Model<Box<Self>>>;
}

pub trait Vizualization {
    fn run(&self);
}
