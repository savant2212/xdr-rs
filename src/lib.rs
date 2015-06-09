#![crate_id="xdr#1.0.1"]
#![crate_type="lib"]
extern crate byteorder;
pub mod xdr;

#[test]
fn u16_writer_test() {
	use std::io::{BufReader,BufWriter,Cursor};
	use std::str;
	use std::vec::Vec;
	use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt,Error};

	let mut x = xdr::XdrWriter::new();
	let mut wtr = vec![];

	wtr.write_u16::<BigEndian>(517).unwrap();
	xdr::XdrPrimitive::write_to_xdr(&mut x,517u16);
	assert_eq!(x.get_buffer(),wtr);
}
