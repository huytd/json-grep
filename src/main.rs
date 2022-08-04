use std::{fs::File, io::BufReader};
use serde_json::Value;

fn travel(path: &str, data: &Value, keyword: &str) {
	if data.is_boolean() || data.is_number() || data.is_string() {
		if data.to_string().contains(keyword) {
			println!("► {} = {}", path, data);
		}
	} else if path.contains(keyword) {
		println!("► {} = [...]", path);
	}
	
	if data.is_object() {
		for (key, value) in data.as_object().unwrap() {
			travel(format!("{}.{}", path, key).as_str(), value, keyword);
		}
	} else if data.is_array() {
		for (index, value) in data.as_array().unwrap().iter().enumerate() {
			travel(format!("{}.{}", path, index).as_str(), value, keyword);
		}
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let file_name = &args[1];
	let keyword = &args[2..].join(" ");
	println!("Searching {} for '{}':\n", file_name, keyword);
	let json_stream = BufReader::new(File::open(file_name).unwrap());
	let json_object: Value = serde_json::from_reader(json_stream).unwrap();
	travel("", &json_object, &keyword);
}
