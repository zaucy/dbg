use super::super::util::resolve_executable_path;
use super::super::util::windows::find_vs_devenv;

pub fn launch() {
	if let Some(devenv) = find_vs_devenv() {
		let executable_path = resolve_executable_path(std::env::args().nth(2).expect("First parameter after lauch must be executable or name of executable in PATH"));

		std::process::Command::new(devenv)
			.arg("/NoSplash")
			.arg("/Command")
			.arg("Debug.Start")
			.arg("/DebugExe")
			.arg(executable_path)
			.args(std::env::args().skip(3))
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	} else {
		panic!("No available debuggers");
	}
}

pub fn attach() {
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
