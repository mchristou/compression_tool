use clap::Args;

#[derive(Args, Debug)]
pub struct DecodeOpt {
    #[clap(long)]
    input: String,
}

impl DecodeOpt {
    pub fn new(input: String) -> DecodeOpt {
        Self { input }
    }
}

pub mod decoder {
    use crate::{
        huffman::{TreeNode, TreeNodeRef},
        DecodeOpt,
    };

    use std::{
        borrow::BorrowMut,
        collections::BTreeMap,
        fs::File,
        io::{self, Read, Write},
    };

    pub fn decode(args: DecodeOpt) -> Result<(), Box<dyn std::error::Error>> {
        let reader = &mut File::open(args.input)?;
        let header_len = read_header_len(reader)?;

        let header = read_header(reader, header_len)?;
        let freq_map = parse_header(&header);

        let root = TreeNode::build_huffman_tree(&freq_map);
        let encoded_data = read_compressed_data(reader)?;

        let data = decompress_data(encoded_data, root);

        let file = &mut File::create("output.txt")?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    fn byte_to_bit_str(bytes: Vec<u8>) -> String {
        let result = bytes
            .iter()
            .flat_map(|byte| (0..8).rev().map(move |i| ((byte >> i) & 1).to_string()))
            .collect();

        result
    }

    fn decompress_data(encoded_data: Vec<u8>, root: Option<TreeNodeRef>) -> String {
        let mut result = String::new();
        let mut current_node = root.clone();

        // Assuming root is not empty so .unwrap is ok
        let original_len = root.clone().unwrap().borrow().frequency;
        let encoded_data = byte_to_bit_str(encoded_data);

        for bit in encoded_data.chars() {
            match bit {
                '0' => {
                    if let Some(node) = current_node.clone() {
                        current_node = node.borrow().left.clone();
                    }
                }
                '1' => {
                    if let Some(node) = current_node.clone() {
                        current_node = node.borrow().right.clone();
                    }
                }
                _ => {}
            }

            if let Some(node) = current_node.clone() {
                if let Some(ch) = node.borrow().val {
                    result.push(char::from(ch));
                    *current_node.borrow_mut() = root.clone();
                }
            }

            if result.len() == original_len {
                break;
            }
        }

        result
    }

    fn read_compressed_data<R: Read>(reader: &mut R) -> Result<Vec<u8>, io::Error> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn parse_header(header: &str) -> BTreeMap<u8, usize> {
        let mut freq_map = BTreeMap::new();

        for item in header.split(',') {
            if let Some((frequency, byte_str)) = item.split_once(':') {
                match byte_str.chars().next() {
                    Some('\n') => {
                        freq_map.insert(b'\n', frequency.parse::<usize>().unwrap());
                    }
                    Some(' ') => {
                        freq_map.insert(b' ', frequency.parse::<usize>().unwrap());
                    }
                    Some(ch) => {
                        freq_map.insert(ch as u8, frequency.parse::<usize>().unwrap());
                    }
                    _ => {}
                };
            }
        }

        freq_map
    }

    fn read_header_len<R: Read>(reader: &mut R) -> Result<usize, io::Error> {
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;

        let header_len = u32::from_le_bytes(len_buf) as usize;

        Ok(header_len)
    }

    fn read_header<R: Read>(reader: &mut R, header_len: usize) -> Result<String, io::Error> {
        let mut header_bytes = vec![0; header_len];
        reader.read_exact(&mut header_bytes)?;
        let header = String::from_utf8_lossy(&header_bytes);

        Ok(header.to_string())
    }

    #[cfg(test)]
    mod test {
        use std::collections::BTreeMap;

        use crate::decoder::*;
        #[test]
        fn test_parse_header() {
            let header1 = "61458:\n";
            let expected_result1: BTreeMap<u8, usize> = BTreeMap::from([(b'\n', 61458)]);
            assert_eq!(parse_header(header1), expected_result1);

            let header2 = "12:3,45: ,678:\n";
            let expected_result2: BTreeMap<u8, usize> =
                BTreeMap::from([(b'3', 12), (b' ', 45), (b'\n', 678)]);
            assert_eq!(parse_header(header2), expected_result2);

            let header3 = "1:2,3:a,45: ,55:\n";
            let expected_result3: BTreeMap<u8, usize> =
                BTreeMap::from([(b'2', 1), (b'a', 3), (b' ', 45), (b'\n', 55)]);
            assert_eq!(parse_header(header3), expected_result3);

            let header5 = "55:  ,49:.,125678:~";
            let expected_result5: BTreeMap<u8, usize> =
                BTreeMap::from([(b' ', 55), (b'.', 49), (b'~', 125678)]);

            assert_eq!(parse_header(header5), expected_result5);

            let header6 = "2:'";
            let expected_result6: BTreeMap<u8, usize> = BTreeMap::from([(b'\'', 2)]);

            assert_eq!(parse_header(header6), expected_result6);
        }
    }
}
