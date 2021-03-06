use std::ptr;
use rustc_serialize::hex::FromHex as RustcFromHex;
use bloomchain::Bloom;

pub trait FromHex {
	fn from_hex(s: &str) -> Self where Self: Sized;
}

impl FromHex for Bloom {
	fn from_hex(s: &str) -> Self {
		let mut res = [0u8; 256];
		let v = s.from_hex().unwrap();
		assert_eq!(res.len(), v.len());
		unsafe {
			ptr::copy(v.as_ptr(), res.as_mut_ptr(), res.len());
		}
		From::from(res)
	}
}
