use byteorder::{ByteOrder, LittleEndian};

pub struct DataFactory;

pub trait Decoder {
    fn from_bytes(bytes: Vec<u8>) -> Self;
}


fn _string_to_next(bytes: &[u8]) -> String {
    string_iter_to_next(&mut bytes.iter())
}

pub fn string_iter_to_next(iter: &mut std::slice::Iter<'_, u8>) -> String {
    let str = String::from_utf8(iter.take_while(|x| **x != 0x0).map(|x| *x).collect()).unwrap();
    iter.next();
    str
}


fn _f32_to_next(bytes: &[u8]) -> f32 {
    f32_iter_to_next(&mut bytes.iter())
}

pub fn f32_iter_to_next(iter: &mut std::slice::Iter<'_, u8>) -> f32 {
    let bytes: Vec<u8> = iter.take(4).map(|x| *x).collect();
    LittleEndian::read_f32(bytes.as_slice())
}

fn _u32_to_next(bytes: &[u8]) -> u32 {
    u32_iter_to_next(&mut bytes.iter())
}

pub fn u32_iter_to_next(iter: &mut std::slice::Iter<'_, u8>) -> u32 {
    let bytes: Vec<u8> = iter.take(4).map(|x| *x).collect();
    LittleEndian::read_u32(bytes.as_slice())
}
