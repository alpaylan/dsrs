use std::vec;

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub enum RedBlackTree<K, V> {
    Leaf,
    Node(
        Box<RedBlackTree<K, V>>,
        (K, V, Color),
        Box<RedBlackTree<K, V>>,
    ),
}

// - Every node is either red or black.
// - All NIL nodes (figure 1) are considered black.
// - A red node does not have a red child.
// - Every path from a given node to any of its descendant
//   NIL nodes goes through the same number of black nodes.
// - (Conclusion) If a node N has exactly one child,
//   it must be a red child, because if it were black,
//   its NIL descendants would sit at a different black
//   depth than N's NIL child, violating requirement 4.

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        RedBlackTree::Leaf
    }

    pub fn color(&self) -> Color {
        match self {
            RedBlackTree::Leaf => Color::Black,
            RedBlackTree::Node(_, (_, _, c), _) => *c,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            RedBlackTree::Leaf => 0,
            RedBlackTree::Node(l, _, r) => l.height().max(r.height()),
        }
    }
}

pub enum Traversal {
    Inorder,
    Preorder,
    Postorder,
}

impl<K: Ord + Eq, V: Clone> RedBlackTree<K, V> {
    pub fn insert(self, k: K, v: V) -> Self {
        match self {
            RedBlackTree::Leaf => todo!(),
            RedBlackTree::Node(_, _, _) => todo!(),
        }
    }

    pub fn balance(self) -> Self {
        match self {
            RedBlackTree::Leaf => todo!(),
            RedBlackTree::Node(_, _, _) => todo!(),
        }
    }



    pub fn find(&self, k: &K) -> Option<V> {
        match self {
            RedBlackTree::Leaf => None,
            RedBlackTree::Node(l, (key, value, _), r) => {
                if k == key {
                    Some(value.clone())
                } else if k < key {
                    l.find(k)
                } else {
                    r.find(k)
                }
            }
        }
    }

    pub fn delete(self, k: &K) -> Self {
        todo!()
    }

    pub fn traverse(&self, t: &Traversal) -> Vec<V> {
        let (ltrav, v, rtrav) = if let RedBlackTree::Node(l, k, v, c, r) = self {
            (l.traverse(t), v, r.traverse(t))
        } else {
            return vec![];
        };

        match t {
            Traversal::Inorder => [ltrav, vec![v.clone()], rtrav].concat(),
            Traversal::Preorder => [vec![v.clone()], ltrav, rtrav].concat(),
            Traversal::Postorder => [ltrav, rtrav, vec![v.clone()]].concat(),
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert() {
        let mut bst: RedBlackTree<u32, u32> = RedBlackTree::new();
        bst = bst.insert(3, 5);
        bst = bst.insert(10, 1);
        assert_eq!(bst.find(&3), Some(5));
        assert_eq!(bst.find(&4), None);
    }

    #[test]
    fn test_delete() {
        let mut bst: RedBlackTree<u32, u32> = RedBlackTree::new();
        bst = bst.insert(3, 5);
        bst = bst.insert(10, 1);
        bst = bst.delete(&3);
        bst = bst.insert(5, 8);
        bst = bst.insert(5, 15);
        bst = bst.insert(25, 2);
        assert_eq!(bst.find(&3), None);
        assert_eq!(bst.find(&5), Some(15));
        assert_eq!(bst.find(&25), Some(2));
    }

    #[test]
    fn test_traverse() {
        let mut bst: RedBlackTree<u32, u32> = RedBlackTree::new();
        bst = bst.insert(3, 5);
        bst = bst.insert(10, 1);
        bst = bst.delete(&3);
        bst = bst.insert(5, 8);
        bst = bst.insert(5, 15);
        bst = bst.insert(25, 2);
        assert_eq!(bst.traverse(&Traversal::Inorder), vec![15, 1, 2]);
        assert_eq!(bst.traverse(&Traversal::Preorder), vec![1, 15, 2]);
        assert_eq!(bst.traverse(&Traversal::Postorder), vec![15, 2, 1]);
    }
}
