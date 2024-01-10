use std::{
    collections::{HashMap, VecDeque},
    env,
    fmt::Write as FmtWrite,
    fs::File,
    io::Read,
    io::{self, Write},
};

mod huffman;

use huffman::{TreeNode, TreeNodeRef};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file.bin>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let frequency_map = character_frequencies(&content);

    let tree = TreeNode::build_huffman_tree(&frequency_map);

    let prefix_table = build_prefix_table(tree);

    let header = build_header(frequency_map);

    let file = &mut File::create(format!("{}.bin", &args[2]))?;
    write_header(header.as_bytes(), file)?;

    let bytes = prefix_table_to_bytes(prefix_table);
    file.write_all(&bytes)?;

    Ok(())
}

fn prefix_table_to_bytes(bit_strings: HashMap<char, String>) -> Vec<u8> {
    let mut packed_bits = String::new();

    for (_, bit_string) in bit_strings {
        packed_bits.push_str(bit_string.as_str());
    }

    while packed_bits.len() % 8 != 0 {
        packed_bits.push_str("0");
    }

    let mut result = vec![];
    for i in (0..packed_bits.len()).step_by(8) {
        let part = &packed_bits[i..i + 8];

        let byte = u8::from_str_radix(part, 2).unwrap();
        result.push(byte);
    }

    result
}

fn write_header<W: Write>(header_data: &[u8], writer: &mut W) -> io::Result<()> {
    writer.write_all(header_data)?;
    writer.write_all(&[0xFF])?;

    Ok(())
}

fn build_header(frequency_map: HashMap<char, usize>) -> String {
    frequency_map
        .iter()
        .fold(String::new(), |mut output, (ch, freq)| {
            let _ = write!(output, "{ch}:{freq},");
            output
        })
}

fn character_frequencies(text: &str) -> HashMap<char, usize> {
    let mut char_frequencies = HashMap::new();

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            *char_frequencies.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    char_frequencies
}

fn build_prefix_table(root: Option<TreeNodeRef>) -> HashMap<char, String> {
    let mut prefix_table = HashMap::new();
    let mut stack: VecDeque<(TreeNodeRef, String)> = VecDeque::new();

    if let Some(root_node) = root {
        stack.push_back((root_node.clone(), String::new()));

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

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn count_charachter_freq() {
        let input = "aaabcdd xxx boo";

        let result = character_frequencies(&input);

        assert_eq!(Some(&3), result.get(&'a'));
        assert_eq!(Some(&2), result.get(&'b'));
        assert_eq!(Some(&1), result.get(&'c'));
        assert_eq!(Some(&2), result.get(&'d'));
        assert_eq!(Some(&3), result.get(&'x'));
        assert_eq!(Some(&2), result.get(&'o'));
        assert_eq!(None, result.get(&' '));

        // Negative test
        assert_eq!(None, result.get(&'!'));
    }

    #[test]
    fn test_build_prefix_table() {
        let leaf1 = Rc::new(RefCell::new(TreeNode {
            val: Some('a'),
            frequency: 5,
            left: None,
            right: None,
        }));

        let leaf2 = Rc::new(RefCell::new(TreeNode {
            val: Some('b'),
            frequency: 9,
            left: None,
            right: None,
        }));

        let combined_node = Rc::new(RefCell::new(TreeNode {
            val: None,
            frequency: 14,
            left: Some(leaf1.clone()),
            right: Some(leaf2.clone()),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: None,
            frequency: 23,
            left: Some(combined_node.clone()),
            right: None,
        }));

        let prefix_table = build_prefix_table(Some(root));

        assert_eq!(prefix_table.get(&'a'), Some(&"00".to_string()));
        assert_eq!(prefix_table.get(&'b'), Some(&"01".to_string()));
    }
}
