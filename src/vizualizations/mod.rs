pub mod stops;
pub mod trips;

use gtfs_structures::Gtfs;

use crate::meta::Meta;

pub struct Model<C>
where
    C: Default + Sized,
{
    meta: Meta,
    gtfs: Gtfs,
    context: C,
}

impl<C: Default + Sized> Model<C> {
    fn from_url(url: &str) -> Model<C> {
        println!("Reading data from {}...", url);
        let gtfs = Gtfs::new(url).expect("Error while reading data.");

        Self {
            gtfs,
            meta: Meta::default(),
            context: Default::default(),
        }
    }

    fn meta<F>(mut self, meta_fn: F) -> Self
    where
        F: FnOnce(&Self) -> Meta,
    {
        self.meta = meta_fn(&self);
        self
    }

    fn context<F>(mut self, context_fn: F) -> Self
    where
        F: FnOnce(&Self) -> C,
    {
        self.context = context_fn(&self);
        self
    }
}
