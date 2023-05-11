use super::super::util::windows::find_vs_devenv;

pub fn launch(executable_path: &str) {
	if let Some(devenv) = find_vs_devenv() {
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
