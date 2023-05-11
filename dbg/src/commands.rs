#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub fn launch() {
	windows::launch();
}

#[cfg(target_os = "windows")]
pub fn attach() {
	windows::attach();
}
