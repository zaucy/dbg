mod commands;
pub mod util;

fn main() {
	let subcommand = std::env::args().nth(1).unwrap();

	match subcommand.as_str() {
		"launch" => commands::launch(),
		"attach" => commands::attach(),
		_ => panic!("Unknown subcommand: {}", subcommand),
	};
}
