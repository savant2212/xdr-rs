#![crate_id="xdr#0.0.1"]
#![crate_type="lib"]

mod xdr {

	use std::str;
	use std::vec::Vec;

	static pad_len:u32 = 4;

	pub struct XdrReader {
		reader : std::io::BufReader;
	}

	pub struct XdrWriter {
		writer : std::io::BufWriter;
	}

	pub trait XdrPrimitive {
		fn read_from_xdr(&x: &mut XdrReader, _ : Option<Self>) -> Result<Self>;
		fn write_to_xdr(&x: &mut XdrWriter, _: Option<Self>);
	}
	impl XdrPrimitive for u32 {
		fn read_from_xdr(&x: &mut XdrReader, _ : Option<u32>) -> Result<u32>{
			let mut s:&[u8:4];

			x.read(s);

			
		}
	}
}
