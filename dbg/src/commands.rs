use std::ffi::OsStr;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn launch<I, S>(executable_path: &str, args: I)
where
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	windows::launch(executable_path, args);
}

#[cfg(target_os = "windows")]
pub fn attach() {
	windows::attach();
}
