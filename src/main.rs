
use rustl::collections::LinkedList;


fn main() {

    let mut list = LinkedList::new(0); 

    for r in 1..=10 {
        list.insert(r);
    }
    list.print_all();


    let mut list = LinkedList::new("e");
    list.insert("h");
    list.insert("t");
    list.insert("e");
    list.insert("sham");
    println!("{:?}", list.to_string())
}