pub fn find_vswhere() -> Option<std::path::PathBuf> {
	// https://github.com/Microsoft/vswhere/wiki/Installing
	let vswhere_path = std::path::PathBuf::from(
		std::env::var("ProgramFiles(x86)").unwrap()
			+ "\\Microsoft Visual Studio\\Installer\\vswhere.exe",
	);

	if !vswhere_path.exists() {
		return None;
	}

	return Some(vswhere_path);
}

pub fn find_vs_installation_path() -> Option<std::path::PathBuf> {
	let vswhere_path = find_vswhere()?;
	let vswhere_output = std::process::Command::new(vswhere_path)
		.arg("-property")
		.arg("installationPath")
		.output()
		.unwrap();

	let vswhere_output = std::str::from_utf8(&vswhere_output.stdout)
		.unwrap()
		.trim()
		.to_string();

	return Some(std::path::PathBuf::from(vswhere_output));
}

pub fn find_vs_devenv() -> Option<std::path::PathBuf> {
	let vswhere_path = find_vswhere()?;

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
