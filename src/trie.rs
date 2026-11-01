#[derive(Default)]
struct TrieNode {
    children: [Option<usize>; 26],
    is_end: bool,
}

#[derive(Default)]
struct Trie {
    nodes: Vec<TrieNode>,
    root: usize,
}


impl Trie {
    pub fn new() -> Self {
        let mut trie = Self::default()
        trie.nodes.push(TrieNode::default());
        trie.root = 0;
        trie
    }


    pub fn insert(&mut self, word: Vec<u8>) {
        let mut root = self.root;

        for ch in word {
            let child_idx = self.nodes[root].children[ch as usize];
            
            if child_idx == None {
                let new_root = self.nodes.len();
                self.nodes.push(TrieNode::default());
                self.nodes[root].children[ch as usize] = Some(new_root);
                root = new_root;
            } else {
                root = child_idx.unwrap();
            }
        }
        self.nodes[root].is_end = true;
    }


    pub fn contains(&self, word: &[u8]) -> bool {
        let mut root = self.root;

        for &ch in word {
            let child_idx = self.nodes[root].children[ch as usize];

            if child_idx == None {
                return false;
            } else {
                root = child_idx.unwrap();
            }
        }

        self.nodes[root].is_end
    }
}