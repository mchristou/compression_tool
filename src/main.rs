use std::collections::HashMap;
use std::{env, fs::File, io::Read};

mod huffman;

use huffman::TreeNode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let frequencies = character_frequencies(&content);

    let tree = TreeNode::build_huffman_tree(&frequencies);

    Ok(())
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

#[cfg(test)]
mod test {
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
}
