use std::path::{Path, PathBuf};

use clap::{AppSettings, Clap};

use crate::policy::Policy;

#[derive(Clap)]
#[clap(version = "1.0", setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(about = "Especifica o caminho do arquivo a ser aberto.", parse(from_os_str))]
    path: PathBuf,

    #[clap(about = "Especifica a política de substituição a ser utilizada.", parse(try_from_str))]
    policy: Policy,
}

impl Options {
    pub fn get_path (&self) -> &Path {
        &self.path
    }

    pub fn get_policy(&self) -> &Policy {
        &self.policy
    }
}
