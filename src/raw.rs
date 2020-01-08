//! Primitive functions for serializing and deserializing NBT data.

use std::io;

use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use cesu8::{from_java_cesu8, to_java_cesu8};

use error::{Error, Result};

/// A convenience function for closing NBT format objects.
///
/// This function writes a single `0x00` byte to the `io::Write` destination,
/// which in the NBT format indicates that an open Compound is now closed.
pub fn close_nbt<W>(dst: &mut W) -> Result<()>
    where W: io::Write {

    dst.write_u8(0x00).map_err(From::from)
}

#[inline]
pub fn write_bare_byte<W>(dst: &mut W, value: i8) -> Result<()>
   where W: io::Write
{
    dst.write_i8(value).map_err(From::from)
}

#[inline]
pub fn write_bare_short<W, E: ByteOrder>(dst: &mut W, value: i16) -> Result<()>
   where W: io::Write
{
    dst.write_i16::<E>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_int<W, E: ByteOrder>(dst: &mut W, value: i32) -> Result<()>
   where W: io::Write
{
    dst.write_i32::<E>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_long<W, E: ByteOrder>(dst: &mut W, value: i64) -> Result<()>
   where W: io::Write
{
    dst.write_i64::<E>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_float<W, E: ByteOrder>(dst: &mut W, value: f32) -> Result<()>
   where W: io::Write
{
    dst.write_f32::<E>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_double<W, E: ByteOrder>(dst: &mut W, value: f64) -> Result<()>
   where W: io::Write
{
    dst.write_f64::<E>(value).map_err(From::from)
}

#[inline]
pub fn write_bare_byte_array<W, E: ByteOrder>(dst: &mut W, value: &[i8]) -> Result<()>
   where W: io::Write
{
    try!(dst.write_i32::<E>(value.len() as i32));
    for &v in value {
        try!(dst.write_i8(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_int_array<W, E: ByteOrder>(dst: &mut W, value: &[i32]) -> Result<()>
   where W: io::Write
{
    try!(dst.write_i32::<E>(value.len() as i32));
    for &v in value {
        try!(dst.write_i32::<E>(v));
    }
    Ok(())
}

#[inline]
pub fn write_bare_long_array<W, E: ByteOrder>(dst: &mut W, value: &[i64]) -> Result<()>
   where W: io::Write
{
    dst.write_i32::<E>(value.len() as i32)?;
    for &v in value {
        dst.write_i64::<E>(v)?;
    }
    Ok(())
}

#[inline]
pub fn write_bare_string<W, E: ByteOrder>(dst: &mut W, value: &str) -> Result<()>
   where W: io::Write
{
    let encoded = to_java_cesu8(value);
    try!(dst.write_u16::<E>(encoded.len() as u16));
    dst.write_all(&encoded).map_err(From::from)
}

/// Extracts the next header (tag and name) from an NBT format source.
///
/// This function will also return the `TAG_End` byte and an empty name if it
/// encounters it.
pub fn emit_next_header<R, E>(src: &mut R) -> Result<(u8, String)>
    where R: io::Read,
          E: ByteOrder,
{
    let tag  = try!(src.read_u8());

    match tag {
        0x00 => { Ok((tag, "".to_string())) },
        _    => {
            let name = try!(read_bare_string::<_, E>(src));
            Ok((tag, name))
        },
    }
}

#[inline]
pub fn read_bare_byte<R>(src: &mut R) -> Result<i8>
    where R: io::Read
{
    src.read_i8().map_err(From::from)
}

#[inline]
pub fn read_bare_short<R, E: ByteOrder>(src: &mut R) -> Result<i16>
    where R: io::Read
{
    src.read_i16::<E>().map_err(From::from)
}

#[inline]
pub fn read_bare_int<R, E: ByteOrder>(src: &mut R) -> Result<i32>
    where R: io::Read
{
    src.read_i32::<E>().map_err(From::from)
}

#[inline]
pub fn read_bare_long<R, E: ByteOrder>(src: &mut R) -> Result<i64>
    where R: io::Read
{
    src.read_i64::<E>().map_err(From::from)
}

#[inline]
pub fn read_bare_float<R, E: ByteOrder>(src: &mut R) -> Result<f32>
    where R: io::Read
{
    src.read_f32::<E>().map_err(From::from)
}

#[inline]
pub fn read_bare_double<R, E: ByteOrder>(src: &mut R) -> Result<f64>
    where R: io::Read
{
    src.read_f64::<E>().map_err(From::from)
}

#[inline]
pub fn read_bare_byte_array<R, E: ByteOrder>(src: &mut R) -> Result<Vec<i8>>
    where R: io::Read
{
    // FIXME: Is there a way to return [u8; len]?
    let len = try!(src.read_i32::<E>()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(src.read_i8()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_int_array<R, E: ByteOrder>(src: &mut R) -> Result<Vec<i32>>
    where R: io::Read
{
    // FIXME: Is there a way to return [i32; len]?
    let len = try!(src.read_i32::<E>()) as usize;
    let mut buf = Vec::with_capacity(len);
    // FIXME: Test performance vs transmute.
    for _ in 0..len {
        buf.push(try!(src.read_i32::<E>()));
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_long_array<R, E: ByteOrder>(src: &mut R) -> Result<Vec<i64>>
    where R: io::Read
{
    let len = src.read_i32::<E>()? as usize;
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        buf.push(src.read_i64::<E>()?);
    }
    Ok(buf)
}

#[inline]
pub fn read_bare_string<R, E: ByteOrder>(src: &mut R) -> Result<String>
    where R: io::Read
{
    let len = try!(src.read_u16::<E>()) as usize;

    if len == 0 { return Ok("".to_string()); }

    let mut bytes = vec![0; len];
    let mut n_read = 0usize;
    while n_read < bytes.len() {
        match try!(src.read(&mut bytes[n_read..])) {
            0 => return Err(Error::IncompleteNbtValue),
            n => n_read += n
        }
    }

    let decoded = from_java_cesu8(&bytes)?;
    Ok(decoded.into_owned())
}
