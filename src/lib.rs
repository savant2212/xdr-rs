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

#[test]
fn u16_reader_test() {
	let wtr = vec![0,0,2,5];
	let mut x = xdr::XdrReader::new(wtr);

	let v = x.unpack::<u16>().unwrap();
	assert_eq!(v,517u16);
}

#[test]
fn r_w_primitive_test() {
	let mut wr = xdr::XdrWriter::new();
	wr.pack(0xCCu8);
	wr.pack(0xAAAAu16);
	wr.pack(0xDEADBEEFu32);
	wr.pack(-1i8);
	wr.pack(-256i16);
	wr.pack(-20i32);
	wr.pack(100.500f32);
	wr.pack(-100.500e10f64);

	let mut rdr = xdr::XdrReader::new(wr.get_buffer());

	assert_eq!(0xCCu8,rdr.unpack::<u8>().unwrap());
	assert_eq!(0xAAAAu16,rdr.unpack::<u16>().unwrap());
	assert_eq!(0xDEADBEEFu32,rdr.unpack::<u32>().unwrap());
	assert_eq!(-1i8,rdr.unpack::<i8>().unwrap());
	assert_eq!(-256i16,rdr.unpack::<i16>().unwrap());
	assert_eq!(-20i32,rdr.unpack::<i32>().unwrap());
	assert_eq!(100.500f32,rdr.unpack::<f32>().unwrap());
	assert_eq!(-100.500e10f64,rdr.unpack::<f64>().unwrap());
}
