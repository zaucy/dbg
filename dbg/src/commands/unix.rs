use std::{ffi::OsStr, path::PathBuf};

fn default_debugger() -> PathBuf {
	which::which("lldb").unwrap()
}

fn resolve_debugger_path(debugger: String) -> Option<PathBuf> {
	which::which(debugger).ok()
}

pub fn launch<I, S>(debugger: Option<String>, executable_path: &str, args: I)
where
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	let debugger = debugger
		.and_then(|debugger| resolve_debugger_path(debugger))
		.unwrap_or_else(|| default_debugger());

	std::process::Command::new(debugger)
		.args(vec!["-O", "settings set auto-confirm 1"])
		.args(vec!["-o", "run"])
		.arg(executable_path)
		.arg("--")
		.args(args)
		.spawn()
		.expect("Failed to spawn debugger");
}

pub fn attach(_debugger: Option<String>) {
	panic!("Cannot attach to processes yet");
}
