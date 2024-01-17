use std::{
    cell::RefCell,
    collections::{BTreeMap, BinaryHeap},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TreeNode {
    pub(crate) val: Option<u8>,
    pub(crate) frequency: usize,
    pub(crate) left: Option<TreeNodeRef>,
    pub(crate) right: Option<TreeNodeRef>,
}

pub(crate) type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TreeNode {
    pub(crate) fn build_huffman_tree(freq_char: &BTreeMap<u8, usize>) -> Option<TreeNodeRef> {
        let mut heap: BinaryHeap<TreeNodeRef> = BinaryHeap::new();

        for (&byte, &freq) in freq_char {
            let node = Rc::new(RefCell::new(TreeNode {
                val: Some(byte),
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
        let freq_map: BTreeMap<u8, usize> = BTreeMap::from([
            (b'a', 5),
            (b'b', 9),
            (b'c', 12),
            (b'd', 13),
            (b'e', 16),
            (b'f', 45),
        ]);

        if let Some(root) = TreeNode::build_huffman_tree(&freq_map) {
            assert_eq!(root.borrow().frequency, 100);
        } else {
            panic!("Failed to build Huffman Tree");
        }
    }
}
