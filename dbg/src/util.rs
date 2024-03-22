pub fn resolve_executable_path(mut executable_path: String) -> String {
	if !std::path::PathBuf::from(&executable_path).exists() {
		if !executable_path.contains('/') && !executable_path.contains('\\') {
			if let Ok(p) = which::which(&executable_path) {
				executable_path = p.to_string_lossy().to_string();
			}
		}
	}

	return executable_path;
}
