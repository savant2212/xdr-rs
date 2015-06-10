use std::io;
use std::fmt;
use std::error;
use std::vec::Vec;
use byteorder;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

pub struct XdrReader {
	reader : io::Cursor<Vec<u8>>
}

pub struct XdrWriter {
	writer : io::Cursor<Vec<u8>>
}

impl XdrWriter {
	pub fn new() -> XdrWriter {
		let v : Vec<u8>= Vec::new();
		XdrWriter{ writer: io::Cursor::new(v)}
	}

	pub fn get_buffer(self) -> Vec<u8> {
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
		match x.reader.read_u32::<BigEndian>() {
			Ok(v) => Ok(v as u16),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u32::<BigEndian>(v as u32).unwrap();
	}
}
impl XdrPrimitive for u8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u8, Error>{
		match x.reader.read_u32::<BigEndian>() {
			Ok(v) => Ok(v as u8),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u32::<BigEndian>(v as u32).unwrap();
	}
}
impl XdrPrimitive for i8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i8, Error>{
		match x.reader.read_i32::<BigEndian>() {
			Ok(v) => Ok(v as i8),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<BigEndian>(v as i32).unwrap();
	}
}
impl XdrPrimitive for i16 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i16, Error>{
		match x.reader.read_i32::<BigEndian>() {
			Ok(v) => Ok(v as i16),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<BigEndian>(v as i32).unwrap();
	}
}	
impl XdrPrimitive for i32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i32, Error>{
		match x.reader.read_i32::<BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for i64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i64, Error>{
		match x.reader.read_i64::<BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i64::<BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for u64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u64, Error>{
		match x.reader.read_u64::<BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u64::<BigEndian>(v).unwrap();
	}

}

impl XdrPrimitive for f32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f32, Error>{
		match x.reader.read_f32::<BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_f32::<BigEndian>(v).unwrap();
	}
}	
impl XdrPrimitive for f64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f64, Error>{
		match x.reader.read_f64::<BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(v)
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_f64::<BigEndian>(v).unwrap();
	}
}	
