
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct ArgumentHelp {
	description: String,
	default: Option<String>,
}

pub struct ArgMan {
	args: HashMap<String, String>,
	args_help: HashMap<String, ArgumentHelp>,
}

impl ArgMan {

	pub fn new() -> ArgMan {
		ArgMan {
			args_help: HashMap::new(),
			args: HashMap::new(),
		}
	}

	pub fn add_arg_unset(&mut self, name: &str, description: &str) {
		self.args_help.insert(name.to_string(), ArgumentHelp{
			default: None,
			description: description.to_string(),
		});
	}

	pub fn add_arg(&mut self, name: &str, default: String, description: &str) {
		self.args_help.insert(name.to_string(), ArgumentHelp{
			default: Some(default),
			description: description.to_string(),
		});
	}

	pub fn print_help(&self) {
		println!("\nUSAGE:\n");

		for (name, arg_help) in &self.args_help {
			println!("{}:", name);
			let common_text = format!("	{}", arg_help.description).to_string();
			match &arg_help.default {
				Some(default) => println!("{} (Default: {})", common_text, default),
				None => println!("{}", common_text),
			}
		}
	}

	fn set_arg(&mut self, name: &str, value_to_add: String) -> bool {

		if self.args.contains_key(name) {
			println!("'{}' is being set twice", name);
			return false;
		}

		self.args.insert(name.to_string(), value_to_add);

		true
	}

	pub fn set_defaults(&mut self) {
		for (name, arg_help) in &self.args_help {

			if !self.args.contains_key(name) {
				match &arg_help.default {
					None => println!("Warning: No default for unset argument {}", name),
					Some(default_value) => {
						println!("Insert default argument : {}: {:?}", name, default_value);
						self.args.insert(name.to_string(), default_value.to_string());
					},
				}
			}
		}
	}

	pub fn parse_args(&mut self) -> bool {
		return self.parse_args_vec(env::args().collect());
	}

	fn parse_args_vec(&mut self, raw_args: Vec<String>) -> bool {

		println!("\nraw_args: {:?}", raw_args);
		for raw_arg in raw_args.iter().skip(1) {

			if raw_arg == "--help" {
				self.print_help();
				return false;
			}

			let raw_arg_split : Vec<&str> = raw_arg.split("=").collect();
			if raw_arg_split.len() != 2 {
				println!("Incorrect argument syntax: {}\n", raw_arg);
				println!("There must be one and only one '=' symbol per argument.");
				println!("Try '{} --help'\n", raw_args[0]);
				return false;
			}

			let name = raw_arg_split[0];
			if !self.args_help.contains_key(name) {
				println!("Unknown argument {}\n", name);
				println!("Try '{} --help'\n", raw_args[0]);
				return false;
			}

			if !self.set_arg(name, raw_arg_split[1].to_string()) {
				println!("Try '{} --help'\n", raw_args[0]);
				return false;
			}
		}

		// Set defaults last if they haven't been set
		self.set_defaults();

		true
	}

	pub fn is_none(&self, arg_name: &str) -> bool {
		return self.args.get(arg_name).is_none();
	}

	pub fn get(&self, arg_name: &str) -> &str {

		if !self.args_help.contains_key(arg_name) {
			panic!("Argument {} is not defined.", arg_name);
		}

		if self.args.get(arg_name).is_none() {
			panic!("Argument {} is not set.", arg_name);
		}

		return &self.args.get(arg_name).unwrap()[..];
	}

	pub fn print_selected_args(&self) {
		println!("\nThe following args were selected:\n");
		for (name, arg) in &self.args {
			println!("{}: {:?}", name, arg);
		}
	}
}

// Tests: Let's make sure ArgMan behaves the way is supposed to in a way that's simple to read
#[cfg(test)]
mod tests {
	use argman::ArgMan;

	#[test]
	fn test_get_str_arg() {
		let raw_args = vec!["binname".to_string(), "-aaa=EXPECTED_STR".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg_unset("-aaa", "Simple string arg");
		assert!(g_args.parse_args_vec(raw_args));
		assert_eq!(g_args.get("-aaa"), "EXPECTED_STR".to_string());
	}

	#[test]
	fn test_help_returns_false() {
		let raw_args = vec!["binname".to_string(), "--help".to_string()];
		let mut g_args = ArgMan::new();
		assert!(!g_args.parse_args_vec(raw_args));
	}

	#[test]
	fn test_2_equals_returns_false() {
		let raw_args = vec!["binname".to_string(), "--aaa=bbb=ccc".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg_unset("-aaa", "Simple string arg");
		assert!(!g_args.parse_args_vec(raw_args));
	}

	#[test]
	fn test_set_0_equals_returns_false() {
		let raw_args = vec!["binname".to_string(), "-aaa".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg_unset("-aaa", "Simple string arg");
		assert!(!g_args.parse_args_vec(raw_args));
	}

	#[test]
	fn test_unknown_argument_returns_false() {
		let raw_args = vec!["binname".to_string(), "-aaa=bbb".to_string()];
		let mut g_args = ArgMan::new();
		assert!(!g_args.parse_args_vec(raw_args));
	}

	#[test]
	#[should_panic(expected = "Argument -aaa is not defined.")]
	fn test_undefined() {
		let raw_args = vec!["binname".to_string()];
		let mut g_args = ArgMan::new();
		assert!(g_args.parse_args_vec(raw_args));
		g_args.get("-aaa");
	}

	#[test]
	#[should_panic(expected = "Argument -aaa is not set.")]
	fn test_defined_unset() {
		let raw_args = vec!["binname".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg_unset("-aaa", "Simple string arg");
		assert!(g_args.parse_args_vec(raw_args));
		g_args.get("-aaa");
	}

	#[test]
	fn argman_defined_default() {
		let raw_args = vec!["binname".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg("-aaa", "mydefault".to_string().clone(), "Simple string arg");
		assert!(g_args.parse_args_vec(raw_args));
		assert_eq!("mydefault".to_string(), g_args.get("-aaa"));
	}

	#[test]
	fn argman_changed_default() {
		let raw_args = vec!["binname".to_string(), "-aaa=notdefault".to_string()];
		let mut g_args = ArgMan::new();
		let default_str = "mydefault".to_string();
		g_args.add_arg("-aaa", default_str.clone(), "Simple string arg");
		assert!(g_args.parse_args_vec(raw_args));
		assert_eq!("notdefault".to_string(), g_args.get("-aaa"));
	}

	#[test]
	fn argman_is_none() {
		let raw_args = vec!["binname".to_string()];
		let mut g_args = ArgMan::new();
		g_args.add_arg_unset("-aaa", "Simple string arg");
		assert!(g_args.parse_args_vec(raw_args));
		assert!(g_args.is_none("-aaa"));
	}

}
