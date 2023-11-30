use std::vec;

#[derive(Debug, Clone)]
pub enum BinaryTree<K, V> {
    Leaf,
    Node(Box<BinaryTree<K, V>>, (K, V), Box<BinaryTree<K, V>>),
}

impl<K, V> BinaryTree<K, V> {
    pub fn new() -> Self {
        BinaryTree::Leaf
    }
}

pub enum Traversal {
    Inorder,
    Preorder,
    Postorder,
}

impl<K: Ord + Eq, V: Clone> BinaryTree<K, V> {
    pub fn insert(self, k: K, v: V) -> Self {
        match self {
            BinaryTree::Leaf => BinaryTree::Node(
                Box::new(BinaryTree::Leaf),
                (k, v),
                Box::new(BinaryTree::Leaf),
            ),
            BinaryTree::Node(l, (key, value), r) => {
                if k == key {
                    BinaryTree::Node(l, (k, v), r)
                } else if k < key {
                    BinaryTree::Node(Box::new(l.insert(k, v)), (key, value), r)
                } else {
                    BinaryTree::Node(l, (key, value), Box::new(r.insert(k, v)))
                }
            }
        }
    }

    pub fn find(&self, k: &K) -> Option<V> {
        match self {
            BinaryTree::Leaf => None,
            BinaryTree::Node(l, (key, value), r) => {
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
        match self {
            BinaryTree::Leaf => BinaryTree::Leaf,
            BinaryTree::Node(l, (key, _), r) => {
                if *k == key {
                    if let BinaryTree::Node(ll, (lk, lv), lr) = *l {
                        BinaryTree::Node(Box::new(ll.delete(&lk)), (lk, lv), lr)
                    } else if let BinaryTree::Node(rl, (rk, rv), rr) = *r {
                        BinaryTree::Node(rl, (rk, rv), Box::new(rr.delete(k)))
                    } else {
                        BinaryTree::Leaf
                    }
                } else if *k < key {
                    l.delete(k)
                } else {
                    r.delete(k)
                }
            }
        }
    }

    pub fn traverse(&self, t: &Traversal) -> Vec<V> {
        let (ltrav, v, rtrav) = if let BinaryTree::Node(l, (_, v), r) = self {
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
        let mut bst: BinaryTree<u32, u32> = BinaryTree::new();
        bst = bst.insert(3, 5);
        bst = bst.insert(10, 1);
        assert_eq!(bst.find(&3), Some(5));
        assert_eq!(bst.find(&4), None);
    }

    #[test]
    fn test_delete() {
        let mut bst: BinaryTree<u32, u32> = BinaryTree::new();
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
        let mut bst: BinaryTree<u32, u32> = BinaryTree::new();
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