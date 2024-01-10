use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TreeNode {
    pub(crate) val: Option<char>,
    pub(crate) frequency: usize,
    pub(crate) left: Option<TreeNodeRef>,
    pub(crate) right: Option<TreeNodeRef>,
}

pub(crate) type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TreeNode {
    pub(crate) fn build_huffman_tree(freq_char: &HashMap<char, usize>) -> Option<TreeNodeRef> {
        let mut heap: BinaryHeap<TreeNodeRef> = BinaryHeap::new();

        for (&char, &freq) in freq_char {
            let node = Rc::new(RefCell::new(TreeNode {
                val: Some(char),
                frequency: freq,
                left: None,
                right: None,
            }));

            heap.push(node);
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();

            let combined_node = Rc::new(RefCell::new(TreeNode {
                val: None,
                frequency: left.borrow().frequency + right.borrow().frequency,
                left: Some(left.clone()),
                right: Some(right.clone()),
            }));

            heap.push(combined_node);
        }

        heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_huffman_tree() {
        let freq_map: HashMap<char, usize> = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ]
        .iter()
        .cloned()
        .collect();

        if let Some(root) = TreeNode::build_huffman_tree(&freq_map) {
            assert_eq!(root.borrow().frequency, 100);
        } else {
            panic!("Failed to build Huffman Tree");
        }
    }
}
