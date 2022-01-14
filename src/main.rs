use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::ffi;

use argh::{FromArgValue, FromArgs};

// This syntax looks antiquated (i.e. from Rust 2015)
// but no, this *is* how you include macros.
// (You can also `use` each item individually, actually)
#[macro_use]
extern crate log;
use simple_logger::SimpleLogger;

mod sb_file;
use sb_file::{CommonHeader, DataHeader};

fn main() {
	// FIXME: would be nice if the output name was generated if not present
	// i.e. prg.txt / prg.prg / prg.sb / prg.sb3 / prg.sb4 => TPRG
	// i.e. sprites.png / sprites.bmp => BSPRITES
	// i.e. TPRG => prg.sb3 / prg.sb4 (opinionated file ext.s) (dependant on FileVersion)
	// i.e. BSPRITES => sprites.png
	
	/* To Clap...
	.version(crate_version!())
	.author(crate_authors!(", "))
	.about(crate_description!())
	...I'll miss you. */
	
	enum ConvertDirection { ToSb, ToPc }
	impl FromArgValue for ConvertDirection {
		fn from_arg_value(value: &str) -> Result<Self, String> {
			use ConvertDirection::*;
			match value {
				"sb" => Ok(ToSb),
				"pc" => Ok(ToPc),
				_ => Err("Invalid Conversion".to_owned()),
			}
		}
	}
	
	#[derive(FromArgs)]
	/// SmileBASIC File Toolkit
	///
	/// Tool to read/write SmileBASIC files in the command line.
	/// (Currently only supports SB3 TXT/PRG files)
	struct FileArgs {
		/// the input file to use
		#[argh(positional)]
		input: PathBuf,
		
		/// the type of the input
		/// if not included, will be assumed based on the extension
		/// txt, png/bmp
		#[argh(option)]
		convert_to: Option<ConvertDirection>,
		
		/// the output file to write
		#[argh(positional)]
		output: Option<PathBuf>,
		
		/// disables regarding/writing human-readable metadata
		#[argh(switch, short = 'b')]
		verbatim: bool,
		
		/// writes more info to stderr
		#[argh(switch, short = 'v')]
		verbose: bool,
		
		/// don't prompt user on overwrite
		#[argh(switch, short = 'f')]
		force: bool,
	}
	
	let fa: FileArgs = argh::from_env();
	
	let simp = SimpleLogger::new();
	if fa.verbose { // TODO: better way?
		simp.init().expect("Failed to initialize logger");
	}
	
	// https://doc.rust-lang.org/std/path/struct.PathBuf.html
	let input_file = &fa.input;
	
	// this makes me wonder if in a distant future version
	// i could replace input_file with a pipe...
	let input_bytes = fs::read(input_file).expect("File Read Error?");
	
	match CommonHeader::read_header(&input_bytes) {
		Ok(h) => {
			info!("Likely is SB file");
			
			// TODO: more than this.
			println!("{}", h);
		},
		Err(m) => {
			info!("Likely is other file (because {})", m);
			match String::from_utf8(input_bytes) {
				Ok(mut file_text) => {
					info!("Loaded source file from `{:?}`", input_file);
					
					if file_text.contains("\r\n") {
						// i really wish someone would spend roughly 1.5 minutes
						// talking about this in a video or something.
						file_text = file_text.replace("\r\n", "\n");
						info!("Replaced CRLFs with LFs.");
					}
					
					let file_size = file_text.len() as i32;
					
					let header = {
						use sb_file::*;
						
						let author = Author::new(b"V360", 42069);
						
						CommonHeader {
							file_type: FileType::Txt,
							file_icon: FileIcon::Prg,
							first_author: author,
							curr_author: author,
							file_size,
							..Default::default()
						}
					};
					
					let expected_length = sb_file::HEADER_LENGTH_SB3 + file_text.len() + sb_file::FOOTER_LENGTH;
					info!("Expecting file size to be {}.", expected_length);
					
					let header_bytes = header.make_header();
					
					let footer_bytes = sb_file::compute_footer(&header_bytes, file_text.as_bytes());
					
					if let Some(output_file) = &fa.output {
						let output_file_prefixed = output_file.with_file_name(
							"T".to_owned() +
							output_file.file_name()
							.and_then(ffi::OsStr::to_str)
							.expect("Invalid output file path encoding")
						);
						debug!("uhhh lol {:?}", output_file_prefixed);
						
						let mut out_file = fs::File::create(output_file)
							.expect("Couldn't open file");
						out_file.set_len(expected_length as u64).expect("Couldn't resize output file");
						
						out_file.write_all(header_bytes.as_ref()).expect("Couldn't write output file header");
						out_file.write_all(file_text.as_bytes()).expect("Couldn't write output file body");
						out_file.write_all(&footer_bytes).expect("Couldn't write output file footer");
					
						info!("Okay I think I wrote the file :)");
					}
				},
				Err(m) => {
					info!("Doesn't seem to be text file (because {})", m);
				},
			}
		}
	}
}
