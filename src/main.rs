use std::fs;
use std::io::Write;

#[macro_use]
extern crate log;

fn main() {
	env_logger::init();
	
    const FILENAME: &str = "hi.sb3";
	const FILENAME_OUT: &str = "HI";
	
	let mut file_text = fs::read_to_string(FILENAME).expect("sorry");
	info!("Loaded SmileBASIC 3 source file `{}`", FILENAME);
	if file_text.contains('\r') {
		file_text = file_text.replace("\r\n", "\n"); // Just in case...
		info!("Replaced CRLFs with LFs.");
	}
	
	let file_size = file_text.len() as i32;
	
	let header = {
		use sb3_file::*;
		
		let author = Author {
			nnid: *b"Hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", // FIXME: ew
			id: 69,
		};
		
		CommonHeader {
			first_author: author,
			curr_author: author,
			file_size,
			..Default::default()
		}
	};
	
	let expected_length = sb3_file::HEADER_LENGTH + file_text.len() + sb3_file::FOOTER_LENGTH;
	info!("Expecting file size to be {}.", expected_length);
	
	let header_bytes = header.as_byte_vec();
	
	let footer_bytes = sb3_file::compute_footer(&header_bytes, file_text.as_bytes());
	
	{
		let mut out_file = fs::File::create("T".to_owned() + FILENAME_OUT)
			.expect("Couldn't open file");
		out_file.set_len(expected_length as u64).expect("Couldn't resize");
		
		out_file.write_all(&header_bytes).expect("Couldn't write header");
		out_file.write_all(file_text.as_bytes()).expect("Couldn't write body");
		out_file.write_all(&footer_bytes).expect("Fucked it up");
	}
	
	info!("Okay I think I wrote the file :)");
}

mod sb3_file {
	use sha1::Sha1;
	use hmac::{Hmac, Mac, NewMac};

	type Sha1Hmac = Hmac<Sha1>;
	
	// hmac key
	const HMAC_KEY: [u8; 64] = *include_bytes!("hmac_key.bin");
	
	// file version
	#[derive(Clone, Copy)]
	pub enum FileVersion { Internal3 = 0, User3 = 1 }
	
	// file type
	#[derive(Clone, Copy)]
	pub enum FileType { Txt, Prg, Dat, Grp } // Icon is determined by this, too.
	
	// compression - `bool`
	
	// icon - inside `FileType`
	impl FileType {
		fn get_icon(&self) -> i16 {
			use FileType::*;
			match self {
				Txt | Dat => 0,
				Prg => 1,
				Grp => 2,
			}
		}
	}
	
	// file size - `i32`
	
	// date stuff (yes everything can be negative for some reason)
	// year - `i16`
	// month - `i8`
	// day - `i8`
	// hour - `i8`
	// minute - `i8`
	// second - `i8`
	#[derive(Clone, Copy)]
	pub struct DateTime {
		pub year: i16, pub month: i8, pub day: i8,
		pub hour: i8, pub minute: i8, pub second: i8
	}
	impl DateTime {
		/// It's assumed it's Little Endian bytes here. :)
		fn to_byte_vec(&self) -> Vec<u8> {
			let year_bytes = self.year.to_le_bytes();
			vec![
				year_bytes[0], year_bytes[1],
				self.month as u8, self.day as u8,
				self.hour as u8, self.minute as u8, self.second as u8
			]
		}
	}
	impl Default for DateTime {
		fn default() -> Self {
			DateTime {
				year: 2069, month: 4, day: 20,
				hour: 13, minute: 37, second: 30,
			}
		}
	}
	
	// 8 unknown bytes
	
	// author stuff
	pub const AUTHOR_NAME_MAX: usize = 18;
	#[derive(Clone, Copy)]
	pub struct Author {
		// Nintendo Network ID
		pub nnid: [u8; AUTHOR_NAME_MAX],
		// User ID (for blacklisting)
		pub id: i32,
	}
	impl Author {
		/// It's assumed it's Little Endian bytes here. :)
		fn to_byte_vec(&self) -> Vec<u8> {
			unimplemented!();
		}
	}
	impl Default for Author {
		fn default() -> Self {
			const NAME: &[u8] = b"Author\n\n\nHi\0"; // FIXME: doesn't look good
			const ID: i32 = i32::from_le_bytes([b'A', b'u', b't', b'h']);
			let mut nnid = [0; AUTHOR_NAME_MAX];
			for (i, &c) in NAME.iter().enumerate() { nnid[i] = c; }
			Author { nnid, id: ID, }
		}
	}
	
	pub const HEADER_LENGTH: usize = 0x50;
	pub struct CommonHeader {
		pub version: FileVersion,
		pub file_type: FileType,
		pub compressed: bool,
		pub file_size: i32,
		pub mod_date: DateTime,
		pub first_author: Author,
		pub curr_author: Author,
	}
	impl CommonHeader {
		pub fn as_byte_vec(&self) -> Vec<u8> {
			let mut header: Vec<u8> = Vec::with_capacity(HEADER_LENGTH);
			// little-endian
			
			// FIXME: beh this looks bad
			
			header.extend_from_slice(&(self.version as i16).to_le_bytes());
			header.extend_from_slice(&(self.file_type as i16).to_le_bytes());
			header.extend_from_slice(&(self.compressed as i16).to_le_bytes());
			header.extend_from_slice(&(self.file_type.get_icon()).to_le_bytes());
			header.extend_from_slice(&(self.file_size).to_le_bytes());
			
			header.append(&mut self.mod_date.to_byte_vec());
			
			header.push(3);
			
			header.extend_from_slice(&self.first_author.nnid);
			header.extend_from_slice(&self.curr_author.nnid);
			
			header.extend_from_slice(&self.first_author.id.to_le_bytes());
			header.extend_from_slice(&self.curr_author.id.to_le_bytes());
			
			header.extend_from_slice(&[0u8; 16]);
			
			assert_eq!(header.len(), HEADER_LENGTH);
			
			header
		}
	}
	impl Default for CommonHeader {
		fn default() -> Self {
			CommonHeader {
				version: FileVersion::User3,
				file_type: FileType::Txt,
				compressed: false,
				file_size: -1,
				mod_date: DateTime::default(),
				first_author: Author::default(),
				curr_author: Author::default(),
			}
		}
	}
	
	pub const FOOTER_LENGTH: usize = 20;
	pub fn compute_footer(header_bytes: &[u8], file_bytes: &[u8]) -> [u8; FOOTER_LENGTH] {
		let mut hasher = Sha1Hmac::new_from_slice(&HMAC_KEY)
			.expect("Failed to create hasher while computing footer.");
		hasher.update(header_bytes);
		hasher.update(file_bytes);
		let res = hasher.finalize().into_bytes();
		let mut bytes = [0u8; FOOTER_LENGTH];
		bytes.clone_from_slice(&res);
		bytes
	}
	
	enum DataDataType { U16, I32, F64 }
}