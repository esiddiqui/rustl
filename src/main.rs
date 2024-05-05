
use std::fmt::Pointer;

use rustl::collections::singly_linked_list::LinkedList as InnerLL; // this is using the actual path 
use rustl::collections::{List, LiNode, LinkedList}; // this is the exported path
// use rustl::collections::List; // this is the exported path



fn main() {
    // let mut list = LinkedList::new(0); 
    // let mut list2: InnerLL<String> = LinkedList::new(String::from("ehtesham"));
    // let list3: Option<LinkedList<f32>> =   LinkedList::new_empty(); 

    // // play with list1 
    // for r in 1..=10 {
    //     list.insert(r);
    // }
    // list.print_all();


    // // play with list3
    // list2.insert(String::from("sonia"));
    // println!("{:?}", list2.to_string());


    // let mut list = LinkedList::new("e");
    // list.insert("h");
    // list.insert("t");
    // list.insert("e");
    // list.insert("sham");
    // println!("{:?}", list.to_string());


    // match list3 {
    //     None => println!("list3 was creating as an empty list"), 
    //     Some(li) => println!("no way, the list has {} items", li.size())
    // }

    let mut li: List<i32> = List::new();
    if li.is_empty() {
        println!("oh yeah !!")
    }

    let mut sz = li.append(4); 
    if !li.is_empty() {
        println!("not empty any more, great !!")
    } else {
        println!("yikes, somethings wrong, shouldn't be empty now");
    }

    if sz == 1 {
        println!("hurray the size is also correct: size = {sz}")
    } else {
        println!("yikes, the size should've been 1 at this point")
    }

    sz = li.append(5); 
    if !li.is_empty() {
        println!("not empty any more, great !!")
    } else {
        println!("yikes, somethings wrong, shouldn't be empty now");
    }

    sz = li.size();
    if sz == 2 {
        println!("hurray the size is also correct: size = {sz}")
    } else {
        println!("yikes, the size should've been 2 at this point")
    }


    sz = li.append(6); 
    sz = li.append(7);
    if sz == 4 {
        println!("hurray the size is also correct: size = {sz}")
    } else {
        println!("yikes, the size should've been 4 at this point")
    }

    li.trav(); 



    println!("The size of this shit is {}", li.size());

    li.append(8);
    println!("The size of this shit is {}", li.size());
    li.append(9);
    println!("The size of this shit is {}", li.size());
    li.append(10);
    println!("The size of this shit is {}", li.size());

    li.trav(); 
    let vect = vec![11,12,13]; 
    li.append_from(vect);
    li.trav(); 
    println!("The size of this shit is {}", li.size());



    li.prepend(3);
    li.prepend(2);
    li.prepend(1);

    li.trav();
    li.prepend_from(vec![0,-1,-2,-3,-4]);

    li.trav();

}