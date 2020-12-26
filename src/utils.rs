use std::io::Read;
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};

pub fn fetch_bytes<T: Read>(reader: &mut T, size : usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);
    let mut part_reader = reader.take(size as u64);
    part_reader.read_to_end(&mut buf).unwrap();

    buf
}

pub fn fetch_u16<T: Read>(reader: &mut T) -> u16 {
    BigEndian::read_u16(&fetch_bytes(reader, 2))
}

pub fn fetch_u32<T: Read>(reader: &mut T) -> u32 {
    BigEndian::read_u32(&fetch_bytes(reader, 4))
}