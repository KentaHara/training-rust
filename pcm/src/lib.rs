fn super_print_message(message: &String) {
	println!("{}", message);
}
pub mod todo_list {
	fn print_message(message: &String) {
		super::super_print_message(message);
	}

	pub mod comment {
		pub fn get(id: u8) -> Option<String> {
			match id {
				1 => Some(String::from("No.1 Message")),
				_ => None,
			}
		}
		pub fn create(message: String) -> Option<u8> {
			super::print_message(&message);
			Some(10)
		}
		pub fn update(id: Option<u8>, message: String) -> Option<u8> {
			match id {
				Some(i) => Some(i),
				None => self::create(message),
			}
		}
	}

	pub mod description {
		fn _get(id: u8) -> Option<String> {
			match id {
				1 => Some(String::from("No.1 Message")),
				_ => None,
			}
		}
		pub fn get(id: u8) -> Option<String> {
			self::_get(id)
		}
		pub fn create(message: String) -> Option<u8> {
			super::print_message(&message);
			Some(10)
		}
		pub fn update(id: Option<u8>, message: String) -> Option<u8> {
			match id {
				Some(i) => Some(i),
				None => self::create(message),
			}
		}
	}
}

pub fn todo_list_comment_create(message: String) -> Option<u8> {
	crate::todo_list::comment::create(message)
}

pub fn todo_list_comment_update(message: String) -> Option<u8> {
	todo_list::comment::update(Some(1), message)
}
