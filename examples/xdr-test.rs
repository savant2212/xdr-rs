use std::io::Write;
use std::io::Read;
use std::net::TcpListener;
use std::thread;

extern crate xdr;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
	println!("listening started, ready to accept");
	for stream in listener.incoming() {
		thread::spawn(|| {
				let mut stream = stream.unwrap();
				let mut data = [0u8;36];
				let _ = stream.read(&mut data);

				let mut strs : Vec<String>= data.iter().map(|b| format!("{:02X}", b)).collect();
				println!("{}",strs.connect(" "));
				let mut rdr = xdr::xdr::XdrReader::from_array(&data);

				let d_u8 : u8 = rdr.unpack().unwrap();
				let d_i8 : i8 = rdr.unpack().unwrap();
				let d_u16 : u16 = rdr.unpack().unwrap();
				let d_i16 : i16 = rdr.unpack().unwrap();
				let d_u32 : u32 = rdr.unpack().unwrap();
				let d_i32 : i32 = rdr.unpack().unwrap();
				let d_f32 : f32 = rdr.unpack().unwrap();
				let d_f64 : f64 = rdr.unpack().unwrap();

				let mut wr = xdr::xdr::XdrWriter::new();

				wr.pack(d_u8);
				wr.pack(d_i8);
				wr.pack(d_u16);
				wr.pack(d_i16);
				wr.pack(d_u32);
				wr.pack(d_i32);
				wr.pack(d_f32);
				wr.pack(d_f64);

				let buf = wr.into_buffer();
				strs = buf.iter().map(|b| format!("{:02X}", b)).collect();
				println!("{}",strs.connect(" "));
				stream.write(&buf[..]).unwrap();
		});
	}
}
