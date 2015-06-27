xdr-rs
======

[![Build Status](https://travis-ci.org/savant2212/xdr-rs.svg?branch=master)](https://travis-ci.org/savant2212/xdr-rs)

This crate provides External Data Representation(XDR) encoding and decoding functions.

The XDR format RFC at https://tools.ietf.org/html/rfc4506

Example
-------

```rust
extern crate xdr;

use xdr::xdr::{XdrWriter,XdrReader};

let mut wr = XdrWriter::new();
wr.pack(0xCCu8);
wr.pack(0xAAAAu16);
wr.pack(0xDEADBEEFu32);
wr.pack(-1i8);
wr.pack(-256i16);
wr.pack(-20i32);
wr.pack(100.500f32);
wr.pack(-100.500e10f64);

let buf = &wr.into_buffer();
let mut rdr = XdrReader::new(buf);

assert_eq!(0xCCu8,rdr.unpack::<u8>().unwrap());
assert_eq!(0xAAAAu16,rdr.unpack::<u16>().unwrap());
assert_eq!(0xDEADBEEFu32,rdr.unpack::<u32>().unwrap());
assert_eq!(-1i8,rdr.unpack::<i8>().unwrap());
assert_eq!(-256i16,rdr.unpack::<i16>().unwrap());
assert_eq!(-20i32,rdr.unpack::<i32>().unwrap());
assert_eq!(100.500f32,rdr.unpack::<f32>().unwrap());
assert_eq!(-100.500e10f64,rdr.unpack::<f64>().unwrap());
```

Building
--------

- cargo build - build library
- cargo test - execute tests
- cargo doc - compile rustdoc (wish that docs were here...)
