mod commands;
pub mod util;

use clap::Parser;
use util::resolve_executable_path;

#[derive(Parser)]
#[command(name = "dbg")]
#[command(bin_name = "dbg")]
enum Args {
	Launch(LaunchArgs),
	Attach(AttachArgs),
}

#[derive(clap::Args)]
#[command(version, about)]
struct LaunchArgs {
	/// Optionally supplied debugger name or path to debugger
	#[arg(long, short)]
	debugger: Option<String>,

	// Executable to debug
	executable: String,

	// Arguments to pass to executable when debugging
	#[arg(trailing_var_arg = true, allow_hyphen_values = true)]
	executable_args: Vec<String>,
}

#[derive(clap::Args)]
#[command(version, about)]
struct AttachArgs {
	#[arg(long, short)]
	debugger: Option<String>,
}

fn main() {
	let args = Args::parse();

	match args {
		Args::Launch(args) => commands::launch(
			args.debugger,
			&resolve_executable_path(args.executable),
			args.executable_args,
		),
		Args::Attach(args) => commands::attach(args.debugger),
	};
}
