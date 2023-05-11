mod commands;
pub mod util;

use util::resolve_executable_path;

fn main() {
	let subcommand = std::env::args().nth(1).unwrap();

	match subcommand.as_str() {
		"launch" => {
			let executable_path = resolve_executable_path(std::env::args().nth(2).expect("First parameter after lauch must be executable or name of executable in PATH"));
			commands::launch(&executable_path, std::env::args().skip(3));
		}
		"attach" => commands::attach(),
		_ => panic!("Unknown subcommand: {}", subcommand),
	};
}
