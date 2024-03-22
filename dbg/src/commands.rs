use std::ffi::OsStr;

#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "unix")]
mod unix;

pub fn launch<I, S>(debugger: Option<String>, executable_path: &str, args: I)
where
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	#[cfg(target_family = "windows")]
	windows::launch(debugger, executable_path, args);

	#[cfg(target_family = "unix")]
	unix::launch(debugger, executable_path, args);
}

pub fn attach(debugger: Option<String>) {
	#[cfg(target_family = "windows")]
	windows::attach(debugger);

	#[cfg(target_family = "unix")]
	unix::attach(debugger);
}
