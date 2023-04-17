fn find_vs_devenv() -> Option<std::path::PathBuf> {
	// https://github.com/Microsoft/vswhere/wiki/Installing
	let vswhere_path = std::path::PathBuf::from(
		std::env::var("ProgramFiles(x86)").unwrap()
			+ "\\Microsoft Visual Studio\\Installer\\vswhere.exe",
	);

	if !vswhere_path.exists() {
		return None;
	}

	let vswhere_output = std::process::Command::new(vswhere_path)
		.arg("-property")
		.arg("productPath")
		.output()
		.unwrap();

	let vswhere_output = std::str::from_utf8(&vswhere_output.stdout)
		.unwrap()
		.trim()
		.to_string();

	let devenv_path = std::path::PathBuf::from(vswhere_output);

	if !devenv_path.exists() {
		return None;
	}

	return Some(devenv_path);
}

fn launch_debugger() {
	if let Some(devenv) = find_vs_devenv() {
		let mut executable_path = std::env::args().nth(2).expect("First parameter after lauch must be executable or name of executable in PATH");
		if !std::path::PathBuf::from(&executable_path).exists() {
			if !executable_path.contains('/') && !executable_path.contains('\\')
			{
				if let Ok(p) = which::which(&executable_path) {
					executable_path = p.to_string_lossy().to_string();
				}
			}
		}

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

fn attach_debugger() {
	if let Some(devenv) = find_vs_devenv() {
		std::process::Command::new(devenv)
			.arg("/NoSplash")
			.arg("/Command")
			.arg("Debug.AttachToProcess hello.exe")
			.spawn()
			.unwrap()
			.wait()
			.unwrap();
	}
}

fn main() {
	let subcommand = std::env::args().nth(1).unwrap();

	match subcommand.as_str() {
		"launch" => launch_debugger(),
		"attach" => attach_debugger(),
		_ => panic!("Unknown subcommand: {}", subcommand),
	};
}
