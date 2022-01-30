#[macro_use]
extern crate custom_error;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod models;
mod validators;
pub mod clients;

pub use clients::mojang::MojangUpdater;