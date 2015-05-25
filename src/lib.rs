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

	pub struct XdrWriter<'a> {
		writer : Box<BufWriter<&'a mut Vec<u8>>>
	}

	impl<'a> XdrWriter<'a> {
		fn new() -> XdrWriter<'a> {
			let v = Box::new(Vec::new());
			XdrWriter{ writer: BufWriter::new(&mut v)}
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
			x.writer.unwrap().write_u32::<BigEndian>(v).unwrap();
		}
	}
}


#[test]
fn u32_test() {
	assert!(false)
}
