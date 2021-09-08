mod error;

use crate::source::Source;
use crate::options::Options;

pub use self::error::ApplicationError;

pub struct Application {
    options: Options
}

impl Application {
    pub fn new (options: Options) -> Application {
        Application {
            options
        }
    }

    pub fn run (&self) -> Result<(), ApplicationError> {
        let path = self.options.get_path();
        let policy = self.options.get_policy();

        let mut source = Source::open(path)?;

        let header = source.header()?;

        let mut cache = policy.build(
            header.get_cache_total_lines(),
            header.get_cache_line_size(),
        );

        while let Some (address) = source.address()? {
            println!("{}", cache.fetch(address.into()));
        }

        Ok(())
    }
}