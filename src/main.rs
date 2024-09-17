use windows_sys::{core::*, Win32::UI::WindowsAndMessaging::*};
use std::ffi::{CString}; 

/*
   5 bits (1 & 0.2 bytes lololol)
   all messages encoded in NOT (NOT yet (haha))
   a-z = 0-25
   26 = ' '
   27 = .
   28 = ?
   29 = !
   30 = (
   31 = )
*/

fn sstring_to_str(sstring: &[u8]) -> Option<String> {
	let mut result = String::from(""); 

	for schar in sstring.iter() {
		result.push(schar_to_char(*schar)?);
	}
	
	Some(result)
}

fn schar_to_char(schar: u8) -> Option<char> {
	if schar > 25 {
		return match schar {
			26 => Some(' '),
			27 => Some('.'),
			28 => Some('?'),
			29 => Some('!'),
			30 => Some('('),
			31 => Some(')'),
			_ => None 
		};
	} else {
		// 65 = A in ASCII
		match char::from_u32(schar as u32 + 65) {
			Some(c) => Some(c),
			None => None
		}
	}
}

fn main() {
	let text: [u8; 7] = [5, 0, 6, 6, 14, 19, 29];
	let result: String = sstring_to_str(&text).unwrap();
	let mut message = String::from("Decoded Message: \"");
	message.push_str(&result);
	message.push('"');

	let message = CString::new(message).unwrap();
	unsafe {
		MessageBoxA(0 as _, message.as_ptr() as *const u8, s!("Secret Message..."), MB_ICONINFORMATION);
	}
}
