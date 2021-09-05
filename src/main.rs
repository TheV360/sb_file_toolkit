use std::fs;
use std::path::Path;
use std::io::Write;
use std::ffi;

use clap::{App, Arg, crate_version, crate_authors, crate_description};

// This syntax looks antiquated (i.e. from Rust 2015)
// but no, this *is* how you include macros.
// (You can also `use` each item individually, actually)
#[macro_use]
extern crate log;

mod sb_file;

fn main() {
	let matches = App::new("SmileBASIC CLI Tools")
		.version(crate_version!())
		.author(crate_authors!(", "))
		.about(crate_description!())
		.arg(Arg::new("input")
			.about("The input file to use.")
			.index(1)
			.required(true)
		)
		.arg(Arg::new("output")
			.about("The output file to write.")
			.index(2)
			.required(true)
			// FIXME: would be nice if the output name was generated if not present
			// i.e. prg.txt / prg.prg / prg.sb / prg.sb3 / prg.sb4 => TPRG
			// i.e. sprites.png / sprites.bmp => BSPRITES
			// i.e. TPRG => prg.sb3 / prg.sb4 (opinionated file ext.s) (dependant on FileVersion)
			// i.e. BSPRITES => sprites.png
		)
		.get_matches();
	
	env_logger::init();
	
	// https://doc.rust-lang.org/std/path/struct.PathBuf.html
	let input_file = Path::new(matches.value_of("input").unwrap());
	let output_file = Path::new(matches.value_of("output").unwrap());
	
	let mut file_text = fs::read_to_string(input_file).expect("sorry");
	info!("Loaded SmileBASIC 3 source file from `{:?}`", input_file);
	
	
	if file_text.contains('\r') {
		// i really wish someone would spend roughly 1.5 minutes
		// talking about this in a video or something.
		file_text = file_text.replace("\r\n", "\n");
		info!("Replaced CRLFs with LFs.");
	}
	
	let file_size = file_text.len() as i32;
	
	let header = {
		use sb_file::*;
		
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
	
	let expected_length = sb_file::HEADER_LENGTH + file_text.len() + sb_file::FOOTER_LENGTH;
	info!("Expecting file size to be {}.", expected_length);
	
	let header_bytes = header.as_byte_vec();
	
	let footer_bytes = sb_file::compute_footer(&header_bytes, file_text.as_bytes());
	
	let output_file_prefixed = output_file.with_file_name(
		"T".to_owned() +
		output_file.file_name()
		.and_then(ffi::OsStr::to_str)
		.expect("Invalid output file path encoding")
	);
	debug!("uhhh lol {:?}", output_file_prefixed);
	
	{
		let mut out_file = fs::File::create(output_file)
			.expect("Couldn't open file");
		out_file.set_len(expected_length as u64).expect("Couldn't resize output file");
		
		out_file.write_all(&header_bytes).expect("Couldn't write output file header");
		out_file.write_all(file_text.as_bytes()).expect("Couldn't write output file body");
		out_file.write_all(&footer_bytes).expect("Couldn't write output file footer");
	}
	
	info!("Okay I think I wrote the file :)");
}
