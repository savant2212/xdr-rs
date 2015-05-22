#![crate_id="xdr#0.0.1"]
#![crate_type="lib"]
extern crate byteorder;
mod xdr {
	use std::io::{BufReader,BufWriter,Cursor};
	use std::str;
	use std::vec::Vec;
	use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};
	static pad_len:u32 = 4;

	pub enum Error {
		UnexpectedValue,
	}
	pub struct XdrReader<'a> {
		reader : BufReader<&'a [u8]>
	}

	pub struct XdrWriter<'a> {
		writer : BufWriter<&'a mut [u8]>
	}

	pub trait XdrPrimitive {
		fn read_from_xdr(&x: &mut XdrReader, _ : Option<Self>) -> Result<Self,Error>;
		fn write_to_xdr(&x: &mut XdrWriter, v: Option<Self>);
	}

	impl XdrPrimitive for u32 {
		fn read_from_xdr(&x: &mut XdrReader, _ : Option<u32>) -> Result<u32, Error>{
			x.read_u32::<BigEndian>().unwrap();	
		}
		fn write_to_xdr(&x: &mut XdrWriter, v:Option<Self>) {
			x.write_u32::<BigEndian>(Some(v)).unwrap();
		}
	}
}
