use core::fmt;
use std::{fs::File, io::BufReader};

use serde::{de::{DeserializeSeed, Visitor, SeqAccess, self}, Deserializer};
use serde_json::Value;

struct GrepDeserialize<'de> {
	keyword: &'de str
}

impl<'de> GrepDeserialize<'de> {
	fn new(keyword: &'de str) -> Self {
		Self { keyword }
	}
}

impl<'de> DeserializeSeed<'de> for GrepDeserialize<'de> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de> {
		struct GrepVisitor<'de> {
			keyword: &'de str,
			current_path: String
		}

		impl<'de> Visitor<'de> for GrepVisitor<'de> {
			type Value = ();

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				write!(formatter, "")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de>, {
				let mut index = 0;
				while let Some(value) = seq.next_element::<Value>()? {
					_ = value.deserialize_any(GrepVisitor{
						keyword: self.keyword,
						current_path: format!("{}.{}", self.current_path, index)
					});
					index += 1;
				}
				Ok(())
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: de::MapAccess<'de>, {
				while let Some((key, value)) = map.next_entry::<String, Value>()? {
					let value_str = value.as_str().unwrap_or("[...]");
					if key.contains(self.keyword) || value_str.contains(self.keyword) {
						println!("➜ {}.{} = {}", self.current_path, key, value_str);
					}

					_ = value.deserialize_any(GrepVisitor{
						keyword: self.keyword,
						current_path: format!("{}.{}", self.current_path, key)
					});
				}
				Ok(())
			}
		}

		_ = deserializer.deserialize_any(GrepVisitor{
			keyword: &self.keyword,
			current_path: String::new()
		});

		Ok(())
    }
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let file_name = &args[1];
	let keyword = &args[2..].join(" ");
	println!("Searching {} for '{}':\n", file_name, keyword);
	let json_reader = BufReader::new(File::open(file_name).unwrap());
	let grep_deserialize = GrepDeserialize::new(keyword);
	let mut deserializer = serde_json::Deserializer::from_reader(json_reader);
	_ = grep_deserialize.deserialize(&mut deserializer);
}
