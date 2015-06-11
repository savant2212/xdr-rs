#![feature(convert)]
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
	let mut x = xdr::XdrReader::new(&wtr);

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

	let buf = &wr.get_buffer();
	let mut rdr = xdr::XdrReader::new(buf);

	assert_eq!(0xCCu8,rdr.unpack::<u8>().unwrap());
	assert_eq!(0xAAAAu16,rdr.unpack::<u16>().unwrap());
	assert_eq!(0xDEADBEEFu32,rdr.unpack::<u32>().unwrap());
	assert_eq!(-1i8,rdr.unpack::<i8>().unwrap());
	assert_eq!(-256i16,rdr.unpack::<i16>().unwrap());
	assert_eq!(-20i32,rdr.unpack::<i32>().unwrap());
	assert_eq!(100.500f32,rdr.unpack::<f32>().unwrap());
	assert_eq!(-100.500e10f64,rdr.unpack::<f64>().unwrap());
}

#[test]
fn variable_length_array_test() {
	let mut wr = xdr::XdrWriter::new();
	let vec = vec![0u32,1,2,3,4,5];

	wr.pack(vec);
	let buf = &wr.get_buffer();
	let mut rdr = xdr::XdrReader::new(buf);

	let res = rdr.unpack::<Vec<u32>>().unwrap();

	assert_eq!(vec![0u32,1,2,3,4,5], res)
}

#[test]
fn fixed_length_array_test() {
	let mut wr = xdr::XdrWriter::new();
	let vec = vec![0u32,1,2,3,4,5];

	wr.pack_array(vec);
	let buf = &wr.get_buffer();
	let mut rdr = xdr::XdrReader::new(buf);

	let res = rdr.unpack_array::<u32>(6).unwrap();

	assert_eq!(vec![0u32,1,2,3,4,5], res)
}
