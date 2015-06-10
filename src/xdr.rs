use std::io;
use std::fmt;
use std::error;
use std::vec::Vec;
use byteorder;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

#[derive(Debug)]
pub enum Error {
	Io(byteorder::Error),
	InvalidValue,
	InvalidType
}

impl From<byteorder::Error> for Error {
	fn from(err: byteorder::Error) -> Error { Error::Io(err) }
}

impl From<Error> for byteorder::Error {
	fn from(err: Error) -> byteorder::Error {
		match err {
			Error::Io(err) => err,
			Error::InvalidValue => byteorder::Error::Io(io::Error::new(io::ErrorKind::Other, "Invalid value")),
			Error::InvalidType => byteorder::Error::Io(io::Error::new(io::ErrorKind::Other, "Invalid type")),
		}
	}
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	        match *self {
				Error::InvalidValue => write!(f, "Invalid value."),
				Error::InvalidType => write!(f, "Invalid type."),
				Error::Io(ref err) => err.fmt(f),
			}
	}
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidValue => "Invalid value.",
            Error::InvalidType => "Invalid type.",
            Error::Io(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidValue => None,
            Error::InvalidType => None,
            Error::Io(ref err) => err.cause(),
        }
    }
}

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

	pub fn pack<T:XdrPrimitive>(&mut self, x:T) {
		XdrPrimitive::write_to_xdr(self, x)
	}
		
}

impl XdrReader {
	pub fn new(x: Vec<u8>) -> XdrReader {
		XdrReader{ reader : io::Cursor::new(x) }
	}

	pub fn unpack<T: XdrPrimitive>(&mut self) -> Result<T,Error> {
		T::read_from_xdr(self)
	}
}

pub trait XdrPrimitive {
	fn read_from_xdr(x: &mut XdrReader) -> Result<Self, Error>;
	fn write_to_xdr(x: &mut XdrWriter, v: Self);
}

impl XdrPrimitive for u32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u32, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u32::<byteorder::BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for u16 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u16, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as u16),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u32::<byteorder::BigEndian>(v as u32).unwrap();
	}
}
impl XdrPrimitive for u8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u8, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as u8),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u32::<byteorder::BigEndian>(v as u32).unwrap();
	}
}
impl XdrPrimitive for i8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i8, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as i8),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<byteorder::BigEndian>(v as i32).unwrap();
	}
}
impl XdrPrimitive for i16 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i16, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as i16),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<byteorder::BigEndian>(v as i32).unwrap();
	}
}	
impl XdrPrimitive for i32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i32, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i32::<byteorder::BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for i64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i64, Error>{
		match x.reader.read_i64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_i64::<byteorder::BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for u64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u64, Error>{
		match x.reader.read_u64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_u64::<byteorder::BigEndian>(v).unwrap();
	}

}

impl XdrPrimitive for f32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f32, Error>{
		match x.reader.read_f32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_f32::<byteorder::BigEndian>(v).unwrap();
	}
}
impl XdrPrimitive for f64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f64, Error>{
		match x.reader.read_f64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
			
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		x.writer.write_f64::<byteorder::BigEndian>(v).unwrap();
	}
}

impl XdrPrimitive for bool {
	fn read_from_xdr(x: &mut XdrReader) -> Result<bool, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(0) => Ok(false),
			Ok(1) => Ok(true),
			Ok(_) => Err(Error::InvalidValue),
			Err(v) => Err(Error::Io(v))
		}
	}

	fn write_to_xdr(x: &mut XdrWriter, v:Self) {
		match v {
			true => x.writer.write_u32::<byteorder::BigEndian>(1).unwrap(),
			false => x.writer.write_u32::<byteorder::BigEndian>(0).unwrap(),
		}
	}
}
