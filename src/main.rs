use clap::Clap;

use self::options::Options;
use self::application::{
    Application,
    ApplicationError,
};

pub mod cache;
pub mod source;
pub mod policy;
pub mod options;
pub mod application;

fn main () -> Result<(), ApplicationError> {
    let application = Application::new(
        Options::parse()
    );

    application.run()
}