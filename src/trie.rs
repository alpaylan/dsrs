
#[derive(Debug)]
pub enum Trie {
    Leaf,
    Node(Vec<(char, Trie)>),
}

impl Trie {
    pub fn new() -> Self {
        Trie::Leaf
    }

    pub fn insert(&mut self, s: &str) {
        let trie = self;
        if let Some(c) = s.chars().next() {
            match trie {
                Trie::Leaf => {
                    let mut leaf = Trie::new();
                    leaf.insert(&s[1..]);
                    let children = vec![(c, leaf)];
                    *trie = Trie::Node(children);
                }
                Trie::Node(children) => {
                    if let Some((i, _)) = children
                    .iter()
                    .enumerate()
                    .find(|(_, (cp, _))| c == *cp) {
                        children[i].1.insert(&s[1..]);
                    } else {
                        let mut t = Trie::new();
                        t.insert(&s[1..]);
                        children.push((c, t));
                    }
                }
            }
        }
    }

    pub fn search(&self, s: &str) -> bool {
        if let Some(c) = s.chars().next() {
            match self {
                Trie::Leaf => false,
                Trie::Node(children) => {
                    if let Some((_, t)) = children.iter().find(|(cp, _)| c == *cp) {
                        t.search(&s[1..])
                    } else {
                        false
                    }
                }
            }
        } else {
            match self {
                Trie::Leaf => true,
                Trie::Node(_) => false,
            }
        }

    }
}

mod tests {
    use core::time;

    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(trie.search("world"));
        assert!(trie.search("hello"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("helto"));
    }
}
