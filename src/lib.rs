#[macro_use]
extern crate custom_error;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod clients;
pub mod models;
mod validators;

pub use clients::mojang::MojangUpdater;
pub use clients::forge::ForgeUpdater;
pub use clients::fabric::FabricUpdater;
pub use clients::liteloader::LiteloaderUpdater;