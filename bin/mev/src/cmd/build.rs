use crate::cmd::config::Config;
use clap::Args;
use reth::{chainspec::EthereumChainSpecParser, cli::Cli};

#[derive(Debug, Args)]
pub struct CliArgs {
    #[clap(env, long = "mev-builder-config", default_value = "config.toml")]
    pub config_file: String,
}

impl TryFrom<CliArgs> for Config {
    type Error = eyre::Error;

    fn try_from(value: CliArgs) -> Result<Self, Self::Error> {
        Self::from_toml_file(value.config_file)
    }
}

pub type Command = Cli<EthereumChainSpecParser, CliArgs>;
