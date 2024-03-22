use super::super::util::windows::find_vs_devenv;
use std::ffi::OsStr;

pub fn launch<I, S>(debugger: Option<String>, executable_path: &str, args: I)
where
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	assert!(matches!(debugger.as_deref(), Some("msvc") | None));

	if let Some(devenv) = find_vs_devenv() {
		std::process::Command::new(devenv)
			.arg("/NoSplash")
			.arg("/Command")
			.arg("Debug.Start")
			.arg("/DebugExe")
			.arg(executable_path)
			.args(args)
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	} else {
		panic!("No available debuggers");
	}
}

pub fn attach(debugger: Option<String>) {
	assert!(matches!(debugger.as_deref(), Some("msvc") | None));

	if let Some(devenv) = find_vs_devenv() {
		std::process::Command::new(devenv)
			.arg("/NoSplash")
			.arg("/Command")
			.arg("Debug.AttachToProcess")
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	}
}
