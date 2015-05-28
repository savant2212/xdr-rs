#![crate_id="xdr#0.0.1"]
#![crate_type="lib"]
extern crate byteorder;
mod xdr {
	use std::io::{BufReader,BufWriter,Cursor};
	use std::str;
	use std::vec::Vec;
	use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt,Error};
	static PAD_LEN:u32 = 4;

	pub struct XdrReader<'a> {
		reader : BufReader<&'a [u8]>
	}

	pub struct XdrWriter {
		writer : Cursor<Vec<u8>>
	}

	impl XdrWriter {
		pub fn new() -> XdrWriter {
			let v : Vec<u8>= Vec::new();
			XdrWriter{ writer: Cursor::new(v)}
		}

		pub fn get_data(self) -> Vec<u8> {
			self.writer.into_inner()
		}
			
	}	
	pub trait XdrPrimitive {
		fn read_from_xdr(x: &mut XdrReader) -> Result<Self, Error>;
		fn write_to_xdr(x: &mut XdrWriter, v: Self);
	}

	impl XdrPrimitive for u32 {
		fn read_from_xdr(x: &mut XdrReader) -> Result<u32, Error>{
			match x.reader.read_u32::<BigEndian>() {
				Ok(v) => Ok(v),
				Err(v) => Err(v)
			}
				
		}
		fn write_to_xdr(x: &mut XdrWriter, v:Self) {
			x.writer.write_u32::<BigEndian>(v).unwrap();
		}
	}
	impl XdrPrimitive for u16 {
		fn read_from_xdr(x: &mut XdrReader) -> Result<u16, Error>{
			match x.reader.read_u16::<BigEndian>() {
				Ok(v) => Ok(v),
				Err(v) => Err(v)
			}
				
		}
		fn write_to_xdr(x: &mut XdrWriter, v:Self) {
			x.writer.write_u16::<BigEndian>(v).unwrap();
		}
	}
}


#[test]
fn u32_test() {
	use std::io::{BufReader,BufWriter,Cursor};
	use std::str;
	use std::vec::Vec;
	use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt,Error};

	let mut x = xdr::XdrWriter::new();
	let mut wtr = vec![];

	wtr.write_u16::<BigEndian>(517).unwrap();
	xdr::XdrPrimitive::write_to_xdr(&mut x,517u16);
	assert_eq!(x.get_data(),wtr);
}
