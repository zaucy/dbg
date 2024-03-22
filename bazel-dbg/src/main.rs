use std::{env::consts::EXE_SUFFIX, process::Stdio};

use clap::Parser;
use which::which;

#[derive(Parser)]
#[command(name = "bazel-dbg", version, about)]
struct Args {
	// Path to bazel, defaults bazel, bazelisk, or aspect on your PATH
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

fn main() {
	let args = Args::parse();
	let dbg_executable_path = args.dbg_path.unwrap_or_else(|| {
		std::fs::canonicalize(
			find_dbg_executable().expect(
				format!("Cannot find 'dbg{EXE_SUFFIX}' locally or in PATH")
					.as_str(),
			),
		)
		.unwrap()
		.to_str()
		.unwrap()
		.to_owned()
	});

	let dbg_executable_path =
		if let Some(p) = dbg_executable_path.strip_prefix("\\\\?\\") {
			p.to_owned()
		} else {
			dbg_executable_path
		}
		.replace("\\", "/");

	let bazel_executable = args.bazel_path.unwrap_or_else(|| {
		which::which("aspect")
			.or_else(|_| which::which("bazelisk"))
			.or_else(|_| which::which("bazel"))
			.expect(
				"bazel, bazelisk, or aspect must be available in your PATH to use bazel-dbg",
			)
			.to_string_lossy()
			.to_string()
	});

	let mut bazel_proc = std::process::Command::new(bazel_executable)
		.arg("run")
		.arg(format!("--run_under={} launch ", dbg_executable_path))
		.args(args.bazel_args)
		.spawn()
		.expect("Failed to spawn bazel process");

	bazel_proc.wait().unwrap();
}
