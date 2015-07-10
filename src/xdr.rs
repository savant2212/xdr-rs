/*!
  This moule contain XDR encoder and decoder
 */

use std::io;
use std::fmt;
use std::error;
use std::vec::Vec;
use byteorder;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

const PADDING : usize = 4;

#[derive(Debug)]
pub enum Error {
	Io(byteorder::Error),
	InvalidValue,
	InvalidType,
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

macro_rules! opt {
	($expr: expr) => ( match $expr {
		Err(v) => return Some(From::from(v)),
		Ok(_) => None,
	} )
}

pub struct XdrReader<'a> {
	reader : io::Cursor<&'a [u8]>
}

pub struct XdrWriter {
	writer : io::Cursor<Vec<u8>>
}

impl XdrWriter {
	pub fn new() -> XdrWriter {
		let v : Vec<u8>= Vec::new();
		XdrWriter{ writer: io::Cursor::new(v)}
	}

	pub fn into_buffer(self) -> Vec<u8> {
		self.writer.into_inner()
	}

	pub fn pack<T:XdrPrimitive>(&mut self, x:T) -> Option<Error>{
		XdrPrimitive::write_to_xdr(self, x)
	}

	pub fn pack_array<T:XdrPrimitive>(&mut self, x:Vec<T>) -> Option<Error> {
		for t in x {
			match XdrPrimitive::write_to_xdr(self,t) {
				Some(t) => return Some(t),
				None => (),
			}
		};
		None
	}
	pub fn pack_opaque_var_len(&mut self, x:Vec<u8>) -> Option<Error>{
		XdrPrimitive::write_to_xdr(self,x.len() as u32);
		for t in x {
			match self.writer.write_u8(t) {
				Err(t) => return Some(Error::Io(t)),
				Ok(_) => (),
			}
		}
		None
	}
	pub fn pack_opaque_fixed_len(&mut self, x:Vec<u8>) -> Option<Error>{
		for t in x {
			match self.writer.write_u8(t) {
				Err(t) => return Some(Error::Io(t)),
				Ok(_) => (),
			}
		}
		None
	}
	pub fn pad(&mut self, len : usize) -> Option<Error> {
		for _ in 0..len {
			match self.writer.write_u8(0) {
				Err(t) => return Some(Error::Io(t)),
				Ok(_) => (),
			}
		}
		None
	}

}
impl<'a> XdrReader<'a> {
	pub fn new(x:&'a Vec<u8>) -> XdrReader<'a> {
		XdrReader{ reader : io::Cursor::new(&x) }
	}

	pub fn from_array(x: &'a [u8]) -> XdrReader<'a> {
		XdrReader{ reader: io::Cursor::new(x) }
	}

	pub fn unpack<T: XdrPrimitive>(&mut self) -> Result<T,Error> {
		T::read_from_xdr(self)
	}

	pub fn unpack_array<T:XdrPrimitive>(&mut self, n : usize) -> Result<Vec<T>,Error> {
		let mut result : Vec<T> = Vec::with_capacity(n);

		for _ in 0..n {
			let t =  T::read_from_xdr(self);
			match t {
				Ok(v) => result.push(v),
				Err(v) => return Err(v),
			}
		};
		Ok(result)
	}
	pub fn unpack_opaque_var_len(&mut self) -> Result<Vec<u8>, Error> {
		let len = try!(self.unpack::<u32>()) as usize;
		let mut v : Vec<u8>= Vec::with_capacity(len);

		for _ in 0..len {
			match self.reader.read_u8() {
				Ok(t) => v.push(t),
				Err(t) => return Err(Error::Io(t))
			}
		}
		Ok(v)
	}
	pub fn unpack_opaque_fixed_len(&mut self, len : usize) -> Result<Vec<u8>, Error> {
		let mut v : Vec<u8>= Vec::with_capacity(len);

		for _ in 0..len {
			match self.reader.read_u8() {
				Ok(t) => v.push(t),
				Err(t) => return Err(Error::Io(t))
			}
		}
		Ok(v)
	}
}

pub trait XdrPrimitive {
	fn read_from_xdr(x: &mut XdrReader) -> Result<Self, Error>;
	fn write_to_xdr(x: &mut XdrWriter, v: Self) -> Option<Error>;
}

impl<T:XdrPrimitive> XdrPrimitive for Vec<T> {
	fn read_from_xdr(x: &mut XdrReader) -> Result<Self, Error>{
		let count = try!(x.unpack::<u32>()) as usize;
		let mut result : Vec<T> = Vec::with_capacity(count);

		for _ in 0..count {
			match x.unpack::<T>() {
				Ok(t) => result.push(t),
				Err(t) => return Err(t),
			}
		};
		Ok(result)
	}
	fn write_to_xdr(x: &mut XdrWriter, v: Self) -> Option<Error> {
		match x.pack( v.len() as u32 ) {
			Some(t) => return Some(t),
			None => (),
		}
		for t in v {
			match x.pack(t) {
				Some(t) => return Some(t),
				None => (),
			}
		};
		None
	}
}

impl XdrPrimitive for String {
	fn read_from_xdr(x: &mut XdrReader) -> Result<Self, Error>{
		let len = try!(x.unpack::<u32>()) as usize;
		let pad = PADDING - (len % PADDING);
		let bytes = try!(x.unpack_opaque_fixed_len(len));

		if pad != 0 {
			try!(x.unpack_opaque_fixed_len(pad));
		};
		match String::from_utf8(bytes) {
			Ok(s) => Ok(s),
			Err(_) => Err(Error::InvalidValue)
		}
	}

	fn write_to_xdr(x: &mut XdrWriter, v: Self) -> Option<Error>{
		let bytes = v.into_bytes();
		let bytes_len = bytes.len();
		let pad = PADDING - (bytes_len % PADDING);
		match x.pack_opaque_var_len(bytes) {
			Some(t) => return Some(t),
			None => (),
		};

		x.pad(pad)
	}
}

impl XdrPrimitive for u32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u32, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_u32::<byteorder::BigEndian>(v))
	}
}
impl XdrPrimitive for u16 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u16, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as u16),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self)  -> Option<Error> {
		opt!(x.writer.write_u32::<byteorder::BigEndian>(v as u32))
	}
}
impl XdrPrimitive for u8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u8, Error>{
		match x.reader.read_u32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as u8),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self)  -> Option<Error> {
		opt!(x.writer.write_u32::<byteorder::BigEndian>(v as u32))
	}
}
impl XdrPrimitive for i8 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i8, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as i8),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self)  -> Option<Error> {
		opt!(x.writer.write_i32::<byteorder::BigEndian>(v as i32))
	}
}
impl XdrPrimitive for i16 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i16, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v as i16),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_i32::<byteorder::BigEndian>(v as i32))
	}
}	
impl XdrPrimitive for i32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i32, Error>{
		match x.reader.read_i32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_i32::<byteorder::BigEndian>(v))
	}
}
impl XdrPrimitive for i64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<i64, Error>{
		match x.reader.read_i64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_i64::<byteorder::BigEndian>(v))
	}
}
impl XdrPrimitive for u64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<u64, Error>{
		match x.reader.read_u64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_u64::<byteorder::BigEndian>(v))
	}
}

impl XdrPrimitive for f32 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f32, Error>{
		match x.reader.read_f32::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_f32::<byteorder::BigEndian>(v))
	}
}
impl XdrPrimitive for f64 {
	fn read_from_xdr(x: &mut XdrReader) -> Result<f64, Error>{
		match x.reader.read_f64::<byteorder::BigEndian>() {
			Ok(v) => Ok(v),
			Err(v) => Err(Error::Io(v))
		}
	}
	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!(x.writer.write_f64::<byteorder::BigEndian>(v))
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

	fn write_to_xdr(x: &mut XdrWriter, v:Self) -> Option<Error> {
		opt!( match v {
			true => x.writer.write_u32::<byteorder::BigEndian>(1),
			false => x.writer.write_u32::<byteorder::BigEndian>(0),
		})
	}
}
