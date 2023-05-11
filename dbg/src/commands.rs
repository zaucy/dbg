#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn launch(executable_path: &str) {
	windows::launch(executable_path);
}

#[cfg(target_os = "windows")]
pub fn attach() {
	windows::attach();
}
