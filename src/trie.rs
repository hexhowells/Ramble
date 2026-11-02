use wincode::{SchemaWrite, SchemaRead, len::BincodeLen, containers};

type MaxBytes = BincodeLen<260000000>;

#[derive(Default)]
#[derive(SchemaWrite, SchemaRead)]
struct TrieNode {
    children: [u32; 26],
    is_end: bool,
}

#[derive(Default)]
#[derive(SchemaWrite, SchemaRead)]
pub struct Trie {
    #[wincode(with = "containers::Vec<TrieNode, MaxBytes>")]
    nodes: Vec<TrieNode>,
    root: usize,
}


impl Trie {
    pub fn new() -> Self {
        let mut trie = Self::default();
        trie.nodes.push(TrieNode::default());
        trie.root = 0;
        trie
    }


    pub fn insert(&mut self, word: Vec<u8>) {
        let mut root = self.root;

        for ch in word {
            let child_idx = self.nodes[root].children[ch as usize] as usize;
            
            if child_idx == 0 {
                let new_root = self.nodes.len();
                self.nodes.push(TrieNode::default());
                self.nodes[root].children[ch as usize] = new_root as u32;
                root = new_root;
            } else {
                root = child_idx;
            }
        }
        self.nodes[root].is_end = true;
    }


    pub fn contains(&self, word: &[u8]) -> bool {
        let mut root = self.root;

        for &ch in word {
            let child_idx = self.nodes[root].children[ch as usize] as usize;

            if child_idx == 0 {
                return false;
            } else {
                root = child_idx;
            }
        }

        self.nodes[root].is_end
    }
}