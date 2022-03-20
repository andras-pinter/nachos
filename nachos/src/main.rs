mod payload;

use std::io::Write;
use flate2::Compression;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
enum OpCode {
    TestArg,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Test {
    opcode: OpCode,
    payload: Vec<u8>
}

fn C() -> Compression {
    Compression::new(4)
}

fn compression() {
    let c = Test {
        opcode: OpCode::TestArg,
        payload: payload::payload(2usize.pow(22)),
    };

    println!("{}", c.payload.len());

    let json = serde_json::to_vec(&c).unwrap();
    let binc = bincode::serialize(&c).unwrap();
    let cbor = serde_cbor::to_vec(&c).unwrap();
    let msgp = rmp_serde::to_vec(&c).unwrap();
    let c_json = serde_json::from_slice::<Test>(&json).unwrap();
    let c_binc = bincode::deserialize::<Test>(&binc).unwrap();
    let c_cbor = serde_cbor::from_slice::<Test>(&cbor).unwrap();
    let c_msgp = rmp_serde::from_slice::<Test>(&msgp).unwrap();
    assert_eq!(c_json, c);
    assert_eq!(c_binc, c);
    assert_eq!(c_cbor, c);
    assert_eq!(c_msgp, c);

    println!("\n\ngz");
    let t0 = std::time::SystemTime::now();
    let comp = compress_gz(&msgp);
    println!("{} - {}", msgp.len(), comp.len());
    println!("compress: {}us", t0.elapsed().unwrap().as_millis());
    let t0 = std::time::SystemTime::now();
    let decompressed = decompress_gz(comp);
    println!("decompress: {}us", t0.elapsed().unwrap().as_micros());
    assert_eq!(rmp_serde::from_slice::<Test>(&decompressed).unwrap(), c);

    println!("\n\nflate");
    let t0 = std::time::SystemTime::now();
    let comp = compress_flate(&msgp);
    println!("{} - {}", msgp.len(), comp.len());
    println!("compress: {}us", t0.elapsed().unwrap().as_millis());
    let t0 = std::time::SystemTime::now();
    let decompressed = decompress_flate(comp);
    println!("decompress: {}us", t0.elapsed().unwrap().as_micros());
    assert_eq!(rmp_serde::from_slice::<Test>(&decompressed).unwrap(), c);

    println!("\n\nzlib");
    let t0 = std::time::SystemTime::now();
    let comp = compress_zlib(&msgp);
    println!("{} - {}", msgp.len(), comp.len());
    println!("compress: {}us", t0.elapsed().unwrap().as_millis());
    let t0 = std::time::SystemTime::now();
    let decompressed = decompress_zlib(comp);
    println!("decompress: {}us", t0.elapsed().unwrap().as_micros());
    assert_eq!(rmp_serde::from_slice::<Test>(&decompressed).unwrap(), c);
}

fn compress_gz(data: &[u8]) -> Vec<u8> {
    let buffer = Vec::with_capacity(data.len());
    let mut enc = flate2::write::GzEncoder::new(buffer, C());
    enc.write_all(data).expect("Failed to encode");
    enc.finish().expect("Failed to finish")
}

fn decompress_gz(data: Vec<u8>) -> Vec<u8> {
    let buffer = Vec::new();
    let mut dec = flate2::write::GzDecoder::new(buffer);
    dec.write_all(&data).expect("Failed to decode");
    dec.finish().expect("Failed to finish")
}

fn compress_flate(data: &[u8]) -> Vec<u8> {
    let buffer = Vec::with_capacity(data.len());
    let mut enc = flate2::write::DeflateEncoder::new(buffer, C());
    enc.write_all(data).expect("Failed to encode");
    enc.finish().expect("Failed to finish")
}

fn decompress_flate(data: Vec<u8>) -> Vec<u8> {
    let buffer = Vec::new();
    let mut dec = flate2::write::DeflateDecoder::new(buffer);
    dec.write_all(&data).expect("Failed to decode");
    dec.finish().expect("Failed to finish")
}

fn compress_zlib(data: &[u8]) -> Vec<u8> {
    let buffer = Vec::with_capacity(data.len());
    let mut enc = flate2::write::ZlibEncoder::new(buffer, C());
    enc.write_all(data).expect("Failed to encode");
    enc.finish().expect("Failed to finish")
}

fn decompress_zlib(data: Vec<u8>) -> Vec<u8> {
    let buffer = Vec::new();
    let mut dec = flate2::write::ZlibDecoder::new(buffer);
    dec.write_all(&data).expect("Failed to decode");
    dec.finish().expect("Failed to finish")
}

fn main() {

}
