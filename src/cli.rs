// crates.io
use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{reload::Handle, EnvFilter, Registry};

#[derive(Debug, Parser)]
#[command(
	version = concat!(
		env!("CARGO_PKG_VERSION"),
		"-",
		env!("VERGEN_GIT_SHA"),
		"-",
		env!("VERGEN_CARGO_TARGET_TRIPLE"),
	),
	about,
	rename_all = "kebab",
)]
pub struct Cli {
	/// Placeholder.
	#[arg(long, short, value_name = "NUM", default_value_t = String::from("Welcome to use rust-initializer!"))]
	pub placeholder: String,
}
impl Cli {
	pub fn run(&self, _log_filter_handle: Handle<EnvFilter, Registry>) -> Result<()> {
		dbg!(self);

		Ok(())
	}
}
