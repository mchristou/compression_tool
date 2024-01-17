use std::fs::{self, File};

use compression_tool::{decoder, encoder, DecodeOpt, EncodeOpt};
#[test]
fn check_compressed_size() {
    let input_file = "tests/input.txt".to_string();
    let encode_opt = EncodeOpt::new(input_file.clone());

    encoder::encode(encode_opt).unwrap();

    let compressed_reader = File::open("compressed.bin").unwrap();
    let compressed_len = compressed_reader.metadata().unwrap().len() as f64;

    let original_reader = File::open(input_file).unwrap();
    let original_len = original_reader.metadata().unwrap().len() as f64;

    assert!(original_len > compressed_len);
}

#[test]
fn decode() {
    let input_file = "tests/input_2.txt".to_string();
    let encode_opt = EncodeOpt::new(input_file.clone());

    encoder::encode(encode_opt).unwrap();

    let compressed_file = "compressed.bin".to_string();
    let decode_opt = DecodeOpt::new(compressed_file);

    decoder::decode(decode_opt).unwrap();

    let input_str = fs::read_to_string(input_file).unwrap();
    let decoded_str = fs::read_to_string("output.txt").unwrap();
    assert_eq!(input_str, decoded_str);
}
