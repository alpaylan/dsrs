
use dsrs::Rope;

fn main() {
    let r = Rope::new("Hello, world!");
    println!("{}", r.collect());
    let r = r.insert(3, " cruel");
    println!("{}", r.collect());
    let r = r.delete(5, 11);
    println!("{}", r.collect());
    println!("{}", r.index(5));
    let (l, r) = r.split(5);
    println!("{}", l.collect());
    println!("{}", r.collect());
    let r = Rope::concat(l, r);
    println!("{}", r.collect());
    
}
