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
	fn as_byte_vec(&self) -> Vec<u8> {
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
		
		// FIXME: beh this looks bad and probably isn't "idiomatic"
		
		header.extend_from_slice(&(self.version as i16).to_le_bytes());
		header.extend_from_slice(&(self.file_type as i16).to_le_bytes());
		header.extend_from_slice(&(self.compressed as i16).to_le_bytes());
		header.extend_from_slice(&(self.file_type.get_icon()).to_le_bytes());
		header.extend_from_slice(&(self.file_size).to_le_bytes());
		
		header.append(&mut self.mod_date.as_byte_vec());
		
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

// data stuff

// TODO: write this when not in a car
enum DataType { U16 = 3, I32 = 4, F64 = 5 }

const DIMENSIONS_MAX: usize = 4;
pub struct DataHeader {
	data_type: DataType,
	dimensions: u8,
	dimension_sizes: [u32; DIMENSIONS_MAX],
}
impl DataHeader {
	pub fn as_byte_vec(&self) -> Vec<u8> {
		// TODO
	}
}
