use clap::Args;

#[derive(Args, Debug)]
pub struct EncodeOpt {
    #[clap(long)]
    input: String,
}

impl EncodeOpt {
    pub fn new(input: String) -> EncodeOpt {
        Self { input }
    }
}

pub mod encoder {
    use crate::huffman::{TreeNode, TreeNodeRef};
    use crate::EncodeOpt;
    use std::collections::{BTreeMap, VecDeque};
    use std::fmt::Write as FmtWrite;
    use std::fs::File;
    use std::io::{self, Read, Seek, SeekFrom, Write};

    pub fn encode(args: EncodeOpt) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(args.input)?;
        let mut reader = io::BufReader::new(file);

        let frequency_map = character_frequencies(&mut reader)?;
        let tree = TreeNode::build_huffman_tree(&frequency_map);

        let prefix_table = build_prefix_table(tree);

        let file = &mut File::create("compressed.bin")?;
        let mut writer = io::BufWriter::new(file);
        write_header(frequency_map, &mut writer)?;

        encode_msg(reader, writer, prefix_table)?;

        Ok(())
    }

    fn encode_msg(
        mut reader: impl Read,
        mut writer: impl Write,
        bit_strings: BTreeMap<u8, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer: [u8; 65536] = [0; 65536];

        let mut compressed = String::new();
        loop {
            let bytes = reader.read(&mut buffer).unwrap();

            if bytes == 0 {
                break;
            }

            for byte in buffer.iter().take(bytes) {
                compressed.push_str(bit_strings.get(byte).unwrap());
            }
        }

        writer.write_all(&bit_str_to_bytes(&compressed))?;

        Ok(())
    }

    fn bit_str_to_bytes(bits: &str) -> Vec<u8> {
        bits.as_bytes()
            .chunks(8)
            .map(|chunk| {
                let bin_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(bin_str, 2).unwrap()
            })
            .collect()
    }

    fn character_frequencies(
        mut reader: impl Read + Seek,
    ) -> Result<BTreeMap<u8, usize>, Box<dyn std::error::Error>> {
        let mut char_frequencies = BTreeMap::new();

        let mut buffer: [u8; 65536] = [0; 65536];

        loop {
            let bytes = reader.read(&mut buffer)?;
            if bytes == 0 {
                break;
            }

            for byte in buffer.iter().take(bytes) {
                let count = char_frequencies.entry(*byte).or_insert(0);
                *count += 1;
            }
        }

        reader.seek(SeekFrom::Start(0))?;

        Ok(char_frequencies)
    }

    fn build_prefix_table(root: Option<TreeNodeRef>) -> BTreeMap<u8, String> {
        let mut prefix_table = BTreeMap::new();
        let mut stack: VecDeque<(TreeNodeRef, String)> = VecDeque::new();

        if let Some(root_node) = root {
            stack.push_back((root_node, String::new()));

            while let Some((node, path)) = stack.pop_back() {
                if let Some(char_value) = node.borrow().val {
                    prefix_table.insert(char_value, path.clone());
                }

                if let Some(right_child) = node.borrow().right.clone() {
                    stack.push_back((right_child, path.clone() + "1"));
                }

                if let Some(left_child) = node.borrow().left.clone() {
                    stack.push_back((left_child, path + "0"));
                }
            }
        }

        prefix_table
    }

    fn build_header(frequency_map: BTreeMap<u8, usize>) -> String {
        frequency_map
            .iter()
            .fold(String::new(), |mut output, (ch, freq)| {
                let _ = write!(output, "{freq}:{ch},", freq = freq, ch = *ch as char);
                output
            })
    }

    fn write_header<W: Write>(
        frequency_map: BTreeMap<u8, usize>,
        writer: &mut W,
    ) -> io::Result<()> {
        let header = build_header(frequency_map);
        let header_len = header.len() as u32;

        writer.write_all(&header_len.to_le_bytes())?;
        writer.write_all(header.as_bytes())?;

        Ok(())
    }

    #[cfg(test)]
    mod test {
        use std::io::Cursor;
        use std::{cell::RefCell, rc::Rc};

        use crate::encoder::*;

        #[test]
        fn test_character_frequencies() {
            let data: &[u8] = b"abccba"; // Example data
            let cursor = Cursor::new(data);

            let frequencies = character_frequencies(cursor).unwrap();

            assert_eq!(frequencies.get(&b'a'), Some(&2));
            assert_eq!(frequencies.get(&b'b'), Some(&2));
            assert_eq!(frequencies.get(&b'c'), Some(&2));

            let data2: &[u8] = b"xyz";
            let cursor2 = Cursor::new(data2);

            let frequencies2 = character_frequencies(cursor2).unwrap();

            assert_eq!(frequencies2.get(&b'x'), Some(&1));
            assert_eq!(frequencies2.get(&b'y'), Some(&1));
            assert_eq!(frequencies2.get(&b'z'), Some(&1));
        }

        #[test]
        fn test_build_prefix_table() {
            // Example tree structure:   a:5
            //                         /   \
            //                        b:2     c:3
            //                       / \   / \
            //                      d:1 e:1 f:2 g:1

            let leaf_d = Rc::new(RefCell::new(TreeNode {
                val: Some(b'd'),
                frequency: 1,
                left: None,
                right: None,
            }));

            let leaf_e = Rc::new(RefCell::new(TreeNode {
                val: Some(b'e'),
                frequency: 1,
                left: None,
                right: None,
            }));

            let leaf_f = Rc::new(RefCell::new(TreeNode {
                val: Some(b'f'),
                frequency: 2,
                left: None,
                right: None,
            }));

            let leaf_g = Rc::new(RefCell::new(TreeNode {
                val: Some(b'g'),
                frequency: 1,
                left: None,
                right: None,
            }));

            let node_b = Rc::new(RefCell::new(TreeNode {
                val: Some(b'b'),
                frequency: 2,
                left: Some(leaf_d),
                right: Some(leaf_e),
            }));

            let node_c = Rc::new(RefCell::new(TreeNode {
                val: Some(b'c'),
                frequency: 3,
                left: Some(leaf_f),
                right: Some(leaf_g),
            }));

            let root = Rc::new(RefCell::new(TreeNode {
                val: Some(b'a'),
                frequency: 5,
                left: Some(node_b),
                right: Some(node_c),
            }));

            let prefix_table = build_prefix_table(Some(root));

            assert_eq!(prefix_table.get(&b'a'), Some(&"".to_string()));
            assert_eq!(prefix_table.get(&b'b'), Some(&"0".to_string()));
            assert_eq!(prefix_table.get(&b'c'), Some(&"1".to_string()));
            assert_eq!(prefix_table.get(&b'd'), Some(&"00".to_string()));
            assert_eq!(prefix_table.get(&b'e'), Some(&"01".to_string()));
            assert_eq!(prefix_table.get(&b'f'), Some(&"10".to_string()));
            assert_eq!(prefix_table.get(&b'g'), Some(&"11".to_string()));
        }
    }
}
