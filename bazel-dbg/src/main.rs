use std::{env::consts::EXE_SUFFIX, process::Stdio};

use clap::Parser;
use which::which;

#[derive(Parser)]
#[command(name = "bazel-dbg")]
#[command(bin_name = "bazel-dbg")]
enum Args {
	Run(RunArgs),
	Attach(AttachArgs),
}

#[derive(clap::Args)]
#[command(version, about)]
struct RunArgs {
	#[arg(long)]
	bazel_path: Option<String>,

	// Path to dbg, defaults to one on your PATH
	#[arg(long)]
	dbg_path: Option<String>,

	// Arguments passed to bazel
	#[arg(trailing_var_arg = true, allow_hyphen_values = true)]
	bazel_args: Vec<String>,
}

#[derive(clap::Args)]
#[command(version, about)]
struct AttachArgs {
	#[arg(long, short)]
	debugger: Option<String>,

	#[arg(long)]
	bazel_path: Option<String>,

	// Path to dbg, defaults to one on your PATH
	#[arg(long)]
	dbg_path: Option<String>,

	// Arguments passed to bazel
	#[arg(trailing_var_arg = true, allow_hyphen_values = true)]
	bazel_args: Vec<String>,
}

fn find_dbg_executable() -> Option<std::path::PathBuf> {
	let current_exe = std::env::current_exe();
	let local_dir = match current_exe {
		Ok(ref p) => p.parent().or(None),
		Err(_) => None,
	};

	if let Some(local_dir) = local_dir {
		let local_dbg_exe_path = local_dir.join("dbg".to_string() + EXE_SUFFIX);

		if local_dbg_exe_path.try_exists().unwrap() {
			return Some(local_dbg_exe_path);
		}
	};

	return match which("dbg") {
		Ok(dbg_path) => Some(dbg_path),
		Err(_) => None,
	};
}

fn default_bazel() -> String {
	which::which("aspect")
			.or_else(|_| which::which("bazelisk"))
			.or_else(|_| which::which("bazel"))
			.expect(
				"bazel, bazelisk, or aspect must be available in your PATH to use bazel-dbg",
			)
			.to_string_lossy()
			.to_string()
}

fn default_dbg() -> String {
	std::fs::canonicalize(find_dbg_executable().expect(
		format!("Cannot find 'dbg{EXE_SUFFIX}' locally or in PATH").as_str(),
	))
	.unwrap()
	.to_str()
	.unwrap()
	.to_owned()
}

fn do_run(args: RunArgs) {
	let dbg_executable_path = args.dbg_path.unwrap_or_else(|| default_dbg());

	let dbg_executable_path =
		if let Some(p) = dbg_executable_path.strip_prefix("\\\\?\\") {
			p.to_owned()
		} else {
			dbg_executable_path
		}
		.replace("\\", "/");

	let bazel_executable = args.bazel_path.unwrap_or_else(|| default_bazel());

	let mut bazel_proc = std::process::Command::new(bazel_executable)
		.arg("run")
		.arg(format!("--run_under={} launch ", dbg_executable_path))
		.args(args.bazel_args)
		.spawn()
		.expect("Failed to spawn bazel process");

	bazel_proc.wait().unwrap();
}

fn do_attach(args: AttachArgs) {
	let dbg_executable_path = args.dbg_path.unwrap_or_else(|| default_dbg());
	let dbg_executable_path =
		if let Some(p) = dbg_executable_path.strip_prefix("\\\\?\\") {
			p.to_owned()
		} else {
			dbg_executable_path
		}
		.replace("\\", "/");

	let bazel_executable = args.bazel_path.unwrap_or_else(|| default_bazel());

	let execution_root = std::process::Command::new(bazel_executable)
		.arg("info")
		.arg("execution_root")
		.output()
		.unwrap();

	let execution_root = std::str::from_utf8(&execution_root.stdout)
		.unwrap()
		.trim()
		.to_string();

	std::process::Command::new(dbg_executable_path)
		.arg("attach")
		.arg(execution_root)
		.spawn()
		.ok();
}

fn main() {
	let args = Args::parse();

	match args {
		Args::Run(args) => do_run(args),
		Args::Attach(args) => do_attach(args),
	};
}
