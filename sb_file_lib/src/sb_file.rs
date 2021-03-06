use std::fmt;

use sha1::Sha1;
use hmac::{Hmac, Mac, NewMac};

type Sha1Hmac = Hmac<Sha1>;

// hmac key
const HMAC_KEY: &[u8; 64] = include_bytes!("hmac_key.bin");

// file version
#[repr(i16)]
#[derive(Debug, Clone, Copy)]
pub enum FileVersion { Sb3 = 1, /* Sb4 = 4, */ }
impl TryFrom<i16> for FileVersion {
	type Error = &'static str;
	fn try_from(value: i16) -> Result<Self, Self::Error> {
		use FileVersion::*;
		match value {
			0..=3 => Ok(Sb3),
			4 => Err("Unimplemented Version"),
			_ => Err("Invalid Version"),
		}
	}
}
impl fmt::Display for FileVersion {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use FileVersion::*;
		write!(f, "{}", match self {
			Sb3 => "SmileBASIC 3",
		})
	}
}

// file type
#[repr(i16)]
#[derive(Debug, Clone, Copy)]
pub enum FileType { Txt = 0, Dat = 1, /* Grp = 2, Meta = 4, */ }
impl TryFrom<i16> for FileType {
	type Error = &'static str;
	fn try_from(value: i16) -> Result<Self, Self::Error> {
		use FileType::*;
		match value {
			0 => Ok(Txt), 1 => Ok(Dat),
			2 | 4 => Err("Unimplemented File Type"),
			_ => Err("Invalid File Type"),
		}
	}
}
impl fmt::Display for FileType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use FileType::*;
		write!(f, "{} file", match self {
			Txt => "text",
			Dat => "data",
		})
	}
}

// compression - `bool`

// icon - i8
#[repr(i16)]
#[derive(Debug, Clone, Copy)]
pub enum FileIcon { Normal = 0, Prg = 1, Grp = 2 }
impl TryFrom<i16> for FileIcon {
	type Error = &'static str;
	fn try_from(value: i16) -> Result<Self, Self::Error> {
		use FileIcon::*;
		match value {
			0 => Ok(Normal),
			1 => Ok(Prg), 2 => Ok(Grp),
			_ => Err("Invalid Icon"),
		}
	}
}
impl fmt::Display for FileIcon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use FileIcon::*;
		write!(f, "{}", match self {
			Normal => "normal",
			Prg => "program",
			Grp => "graphics",
		})
	}
}

// file size - `i32`

// year - `i16`
// month - `i8`
// day - `i8`
// hour - `i8`
// minute - `i8`
// second - `i8`

/// One of the few things that's the same between both versions.
pub const DATETIME_LENGTH: usize = 8;

/// The SmileBASIC file's last modification date & time.
/// Thanks to these being signed integers, they can indeed
/// be negative. Negative months are allowed. It's fun!
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
	pub year: i16, pub month: i8, pub day: i8,
	pub hour: i8, pub minute: i8, pub second: i8,
	pub weekday: i8,
}
impl DateTime {
	pub fn to_le_bytes(self) -> [u8; DATETIME_LENGTH] {
		let year_bytes = self.year.to_le_bytes();
		[
			year_bytes[0], year_bytes[1],
			self.month as u8, self.day as u8,
			self.hour as u8, self.minute as u8, self.second as u8,
			self.weekday as u8,
		]
	}
}
impl Default for DateTime {
	fn default() -> Self {
		DateTime {
			year: 2069, month: 4, day: 20,
			hour: 13, minute: 37, second: 30,
			weekday: 6,
		}
	}
}
impl From<[u8; 8]> for DateTime {
	fn from(value: [u8; 8]) -> Self {
		DateTime {
			year: i16::from_le_bytes(value[0..2].try_into().unwrap()),
			month: value[2] as i8, day: value[3] as i8,
			hour: value[4] as i8, minute: value[5] as i8,
			second: value[6] as i8,
			weekday: value[7] as i8,
		}
	}
}
impl fmt::Display for DateTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let wday = *["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
			.get(self.weekday as usize).unwrap_or(&"???");
		write!(
			f, "{:}/{:02}/{:02} ({}) {:02}:{:02}:{:02}",
			self.year, self.month, self.day, wday,
			self.hour, self.minute, self.second
		)
	}
}

// 8 bytes likely to be padding

// author stuff
/// The maximum length of an author's NNID.
/// This differs between SmileBASIC 3 and 4.
/// I'm SmileBASIC 3.
/// THIS IS NOT THE SIZE OF THE AUTHOR TYPE!!!
pub const AUTHOR_NAME_MAX: usize = 18;
/// The file's author.
#[derive(Debug, Clone, Copy)]
pub struct Author {
	/// Nintendo Network ID
	pub nnid: [u8; AUTHOR_NAME_MAX],
	
	/// User ID (for blacklisting)
	pub id: i32,
}
impl Author {
	pub fn new(name: &[u8], id: i32) -> Self {
		let nnid = Author::nnid_from_byte_str(name);
		Author { nnid, id, }
	}
	fn nnid_from_byte_str(bstr: &[u8]) -> [u8; AUTHOR_NAME_MAX] {
		// does smilebasic  like entirely filling the buffer w/o termination
		// (it's likely fine)
		assert!(bstr.len() <= AUTHOR_NAME_MAX);
		
		let mut nnid = [0; AUTHOR_NAME_MAX];
		nnid.split_at_mut(bstr.len()).0.copy_from_slice(bstr);
		nnid
	}
}
impl Default for Author {
	fn default() -> Self {
		let nnid = Author::nnid_from_byte_str(b"Author");
		let id = i32::from_le_bytes([nnid[0], nnid[1], nnid[2], nnid[3]]);
		Author { nnid, id, }
	}
}
impl fmt::Display for Author {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let nnid = String::from_utf8_lossy(&self.nnid);
		write!(f, "`{}` (ID: {})", nnid, self.id)
	}
}

pub const HEADER_LENGTH_SB3: usize = 0x50;
type RawCommonHeader = [u8; HEADER_LENGTH_SB3];

#[derive(Debug)]
pub struct CommonHeader {
	pub version: FileVersion,
	pub file_type: FileType,
	pub compressed: bool,
	pub file_icon: FileIcon,
	pub file_size: i32,
	pub mod_date: DateTime,
	pub first_author: Author,
	pub curr_author: Author,
	//  unknown: [u8; 16] | sb4: [u8; 20],
}
impl CommonHeader {
	pub fn read_header(data: &[u8]) -> Result<Self, &'static str> {
		if data.len() < HEADER_LENGTH_SB3 { return Err("Not long enough"); }
		
		let data = &data[..HEADER_LENGTH_SB3];
		
		let version_int = i16::from_le_bytes(data[0..2].try_into().unwrap());
		let version = FileVersion::try_from(version_int)?;
		
		let file_type_int = i16::from_le_bytes(data[2..4].try_into().unwrap());
		let file_type = FileType::try_from(file_type_int)?;
		
		let compressed = i16::from_le_bytes(data[4..6].try_into().unwrap()) > 0;
		
		let file_icon_int = i16::from_le_bytes(data[6..8].try_into().unwrap());
		let file_icon = FileIcon::try_from(file_icon_int)?;
		
		let file_size = i32::from_le_bytes(data[8..0x0c].try_into().unwrap());
		
		// if you put the first line's rhs into the second's DateTime::from,
		// it just fucks up this whole scene. what????
		let mod_date_arr: [u8; DATETIME_LENGTH] = data[0x0c..0x14].try_into().unwrap();
		let mod_date = DateTime::from(mod_date_arr);
		
		let first_author_nnid = &data[0x14..][..AUTHOR_NAME_MAX];
		let curr_author_nnid  = &data[0x26..][..AUTHOR_NAME_MAX];
		
		let first_author_id = i32::from_le_bytes(data[0x38..][..4].try_into().unwrap());
		let curr_author_id = i32::from_le_bytes(data[0x3c..][..4].try_into().unwrap());
		
		let first_author = Author::new(first_author_nnid, first_author_id);
		let curr_author = Author::new(curr_author_nnid, curr_author_id);
		
		Ok(CommonHeader {
			version, file_type, compressed,
			file_icon, file_size, mod_date,
			first_author, curr_author,
		})
	}
	pub fn write_header(&self, data: &mut RawCommonHeader) {
		data[0..2].copy_from_slice(&i16::to_le_bytes(self.version as i16));
		data[2..4].copy_from_slice(&i16::to_le_bytes(self.file_type as i16));
		data[4..6].copy_from_slice(&i16::to_le_bytes(self.compressed as i16));
		data[6..8].copy_from_slice(&i16::to_le_bytes(self.file_icon as i16));
		data[8..0x0c].copy_from_slice(&i32::to_le_bytes(self.file_size as i32));
		data[0x0c..0x14].copy_from_slice(&self.mod_date.to_le_bytes());
		data[0x14..][..18].copy_from_slice(&self.first_author.nnid);
		data[0x26..][..18].copy_from_slice(&self.curr_author.nnid);
		data[0x38..][..4].copy_from_slice(&i32::to_le_bytes(self.first_author.id));
		data[0x3C..][..4].copy_from_slice(&i32::to_le_bytes(self.curr_author.id));
	}
	pub fn make_header(&self) -> RawCommonHeader {
		let mut bytes = [0_u8; HEADER_LENGTH_SB3];
		self.write_header(&mut bytes);
		bytes
	}
}
impl Default for CommonHeader {
	fn default() -> Self {
		CommonHeader {
			version: FileVersion::Sb3,
			file_type: FileType::Txt,
			compressed: false,
			file_icon: FileIcon::Prg,
			file_size: -1,
			mod_date: DateTime::default(),
			first_author: Author::default(),
			curr_author: Author::default(),
		}
	}
}
impl fmt::Display for CommonHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let compress_str = if self.compressed { " (compressed)" } else { "" };
		write!(
			f, "{} {}{}\n~{} bytes\t{} icon\nmodified: {}\nfirst author: {}\ncurr. author: {}",
			self.version, self.file_type, compress_str,
			self.file_size, self.file_icon,
			self.mod_date,
			self.first_author, self.curr_author
		)
	}
}

pub const FOOTER_LENGTH: usize = 20;
pub fn compute_footer(header_bytes: &[u8; HEADER_LENGTH_SB3], file_bytes: &[u8]) -> [u8; FOOTER_LENGTH] {
	let mut hasher = Sha1Hmac::new_from_slice(HMAC_KEY)
		.expect("Failed to create hasher while computing footer.");
	hasher.update(header_bytes.as_ref());
	hasher.update(file_bytes);
	let res = hasher.finalize().into_bytes();
	let mut bytes = [0_u8; FOOTER_LENGTH];
	bytes.clone_from_slice(&res);
	bytes
}

// data stuff

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DataType { I8 = 0, U8 = 1, I16 = 2, U16 = 3, I32 = 4, F64 = 5 }
impl TryFrom<i16> for DataType {
	type Error = &'static str;
	fn try_from(value: i16) -> Result<Self, Self::Error> {
		use DataType::*;
		match value {
			0 => Ok(I8),  1 => Ok(U8),
			2 => Ok(I16), 3 => Ok(U16),
			4 => Ok(I32),
			5 => Ok(F64),
			_ => Err("Unknown Data Type"),
		}
	}
}

const DATA_MAGIC: &[u8] = b"PCBN000";
const DATA_HEADER_LENGTH: usize = 26;
const DATA_DIMENSIONS_MAX: usize = 4;

type RawDataHeader = [u8; DATA_HEADER_LENGTH];
pub struct DataHeader {
	pub device_type: FileVersion,
	pub data_type: DataType,
	pub dimensions: i16,
	pub dimension_sizes: [i32; DATA_DIMENSIONS_MAX],
}
impl DataHeader {
	pub fn read_header(data: &[u8]) -> Result<Self, &'static str> {
		if data.len() < DATA_HEADER_LENGTH { return Err("Not long enough"); }
		if &data[0..7] != DATA_MAGIC { return Err("No Magic String"); }
		if !&data[4..8].is_ascii() { return Err("Invalid Device Type"); }
		
		// I was about to be silly and parse the entire god dang 4-digit number
		// but that's unnecessary and I'm trying to stop being like that.
		
		let device_type_int = data[7].checked_sub(b'0')
			.ok_or("Invalid Device Type Digit")?;
		let device_type = (device_type_int as i16).try_into()?;
		
		let data_type_int = i16::from_le_bytes(data[8..10].try_into().unwrap());
		let data_type = data_type_int.try_into()?;
		
		let dimensions_int = i16::from_le_bytes(data[0x0a..][..2].try_into().unwrap());
		let dimensions = dimensions_int;
		
		// FIXME: Incomplete!!
		Ok(DataHeader {
			device_type, data_type,
			dimensions,
			..unimplemented!()
		})
	}
	pub fn write_header(&self, _data: &mut RawDataHeader) {
		unimplemented!()
	}
	pub fn make_header(&self) -> RawDataHeader {
		let mut bytes = [0_u8; DATA_HEADER_LENGTH];
		self.write_header(&mut bytes);
		bytes
	}
}
