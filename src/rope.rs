type Weight = usize;

#[derive(Debug, Clone)]
pub enum Rope {
    Sentinel,
    Leaf(String, Weight),
    Node(Box<Rope>, Weight, Box<Rope>),
}

impl Rope {

    pub fn new(s: &str) -> Rope {
        Rope::Leaf(s.to_string(), s.len()).balance()
    }

    fn weight(&self) -> Weight {
        match self {
            Rope::Sentinel => 0,
            Rope::Leaf(_, w) => *w,
            Rope::Node(_, w, _) => *w,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Rope::Sentinel => 0,
            Rope::Leaf(s, _) => s.len(),
            Rope::Node(l, w, r) => w + r.len(),
        }
    }

    pub fn index(&self, i: usize) -> char {
        match self {
            Rope::Sentinel => panic!("Index out of bounds"),
            Rope::Leaf(s, _) => s.chars().nth(i).unwrap(),
            Rope::Node(l, w, r) => {
                if i < *w {
                    l.index(i)
                } else {
                    r.index(i - w)
                }
            }
        }
    }

    pub fn concat(l: Rope, r: Rope) -> Rope {
        let w = l.len();
        Rope::Node(Box::new(l), w, Box::new(r))
    }

    pub fn split(self, i: usize) -> (Rope, Rope) {
        match self {
            Rope::Sentinel => panic!("Index out of bounds"),
            Rope::Leaf(s, _) => {
                let (l, r) = s.split_at(i);
                (Rope::Leaf(l.to_string(), l.len()), Rope::Leaf(r.to_string(), r.len()))
            }
            Rope::Node(l, w, r) => {
                if i < w {
                    let (ll, lr) = l.split(i);
                    (ll, Rope::concat(lr, *r))
                } else {
                    let (rl, rr) = r.split(i - w);
                    (Rope::concat(*l, rl), rr)
                }
            }
        }
    }

    pub fn insert(self, i: usize, s: &str) -> Rope {
        let (l, r) = self.split(i);
        Rope::concat(l, Rope::concat(Rope::Leaf(s.to_string(), s.len()), r))
    }

    pub fn delete(self, i: usize, j: usize) -> Rope {
        let (l, r) = self.split(i);
        let (_, r) = r.split(j - i);
        Rope::concat(l, r)
    }

    pub fn balance(self) -> Rope {
        let s = self.collect();
        if s.len() <= 8 {
            let l = s.len();
            Rope::Leaf(s, l)
        } else {
            let (l, r) = s.split_at(s.len() / 2);
            Rope::concat(Rope::Leaf(l.to_string(), l.len()).balance(), Rope::Leaf(r.to_string(), r.len()).balance())
        }
    }

    pub fn collect(&self) -> String {
        match self {
            Rope::Sentinel => String::new(),
            Rope::Leaf(s, _) => s.clone(),
            Rope::Node(l, _, r) => format!("{}{}", l.collect(), r.collect())
        }
    }


}
