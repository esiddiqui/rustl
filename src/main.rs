
use rustl::collections::singly_linked_list::LinkedList as InnerLL; // this is using the actual path 
use rustl::collections::LinkedList; // this is the exported path



fn main() {
    let mut list = LinkedList::new(0); 
    let mut list2: InnerLL<String> = LinkedList::new(String::from("ehtesham"));
    let list3: Option<LinkedList<f32>> =   LinkedList::new_empty(); 

    // play with list1 
    for r in 1..=10 {
        list.insert(r);
    }
    list.print_all();


    // play with list3
    list2.insert(String::from("sonia"));
    println!("{:?}", list2.to_string());


    let mut list = LinkedList::new("e");
    list.insert("h");
    list.insert("t");
    list.insert("e");
    list.insert("sham");
    println!("{:?}", list.to_string());


    match list3 {
        None => println!("list3 was creating as an empty list"), 
        Some(li) => println!("no way, the list has {} items", li.size())
    }

}