#![crate_type="lib"]
extern crate byteorder;
pub mod xdr;

#[test]
fn u16_writer_test() {
	let mut x = xdr::XdrWriter::new();
	let wtr = vec![0,0,2,5];

	x.pack(517u16);
	assert_eq!(x.get_buffer(),wtr);
}
