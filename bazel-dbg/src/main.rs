use std::env::consts::EXE_SUFFIX;

use which::which;

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
	let dbg_executable_path = std::fs::canonicalize(
		find_dbg_executable().expect(
			format!("Cannot find 'dbg{EXE_SUFFIX}' locally or in PATH")
				.as_str(),
		),
	)
	.unwrap()
	.to_str()
	.unwrap()
	.to_owned();

	let dbg_executable_path =
		if let Some(p) = dbg_executable_path.strip_prefix("\\\\?\\") {
			p.to_owned()
		} else {
			dbg_executable_path
		}
		.replace("\\", "/");

	dbg!(&dbg_executable_path);

	let mut bazel_proc = std::process::Command::new("bazel")
		.arg("run")
		.arg(format!("--run_under={} launch ", dbg_executable_path))
		.args(std::env::args().skip(1))
		.spawn()
		.expect("Failed to spawn bazel process");

	bazel_proc.wait().unwrap();
}
