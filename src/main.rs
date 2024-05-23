// use std::fmt::Pointer;

// use rustl::collections::singly_linked_list_old::LinkedList as InnerLL; // this is using the actual path
use rustl::collections::SinglyLinkedList as List; // this is the exported path
                                                  // use rustl::collections::List; // this is the exported path

//

fn main() {


    let mut lilly = List::new(); 
    lilly.append(101);
    lilly.append(102);
    for s in 0..=1000 {
        lilly.append(s+103);
    }


    for l in lilly {
        print!("{l},");
    }

   // println!("the size of the thing is {}", lilly.size());

    // println!("list size={}", lilly.size());

    // let a =  lilly.take();
    // println!("list now after take {:?} {}",a, lilly.size());

    // let a =  lilly.take();
    // println!("list now after take {:?} {}",a, lilly.size());

    // lilly.append_from(vec![1,2,3,4,5,6,7,8,9,10]);
    // for num in lilly{
    //     print!("{num}");
    // }

    // println!("list now after iteration {:?} {}",a, lilly.size());

    if true {
        return;
    }

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

    // let mut li: List<i32> = List::new();
    // if li.is_empty() {
    //     println!("oh yeah !!")
    // }

    // let mut sz = li.append(4);
    // if !li.is_empty() {
    //     println!("not empty any more, great !!")
    // } else {
    //     println!("yikes, somethings wrong, shouldn't be empty now");
    // }

    // if sz == 1 {
    //     println!("hurray the size is also correct: size = {sz}")
    // } else {
    //     println!("yikes, the size should've been 1 at this point")
    // }

    // sz = li.append(5);
    // if !li.is_empty() {
    //     println!("not empty any more, great !! the size is {sz}")
    // } else {
    //     println!("yikes, somethings wrong, shouldn't be empty now");
    // }

    // sz = li.size();
    // if sz == 2 {
    //     println!("hurray the size is also correct: size = {sz}")
    // } else {
    //     println!("yikes, the size should've been 2 at this point")
    // }

    // li.append(6);
    // sz = li.append(7);
    // if sz == 4 {
    //     println!("hurray the size is also correct: size = {sz}")
    // } else {
    //     println!("yikes, the size should've been 4 at this point")
    // }

    // li.trav();

    // println!("The size of this shit is {}", li.size());

    // li.append(8);
    // println!("The size of this shit is {}", li.size());
    // li.append(9);
    // println!("The size of this shit is {}", li.size());
    // li.append(10);
    // println!("The size of this shit is {}", li.size());

    // li.trav();
    // let vect = vec![11,12,13];
    // li.append_from(vect);
    // li.trav();
    // println!("The size of this shit is {}", li.size());

    // li.prepend(3);
    // li.prepend(2);
    // li.prepend(1);

    // li.trav();
    // li.prepend_from(vec![0,-1,-2,-3,-4]);

    // li.trav();

    // let name = String::from("Ehtesham Siddiqui");
    // let mut buff = List::new();
    // for chr in name.chars() {
    //     buff.append(chr);
    // }
    // buff.trav2();

    let mut bff2 = List::new();

    bff2.prepend('z');
    bff2.trav();
    println!("{}", bff2.size());
    bff2.delete();
    bff2.trav();
    println!("{}", bff2.size());

    bff2.append_from(vec!['k', 'z', 'e', 'h', 't', 'e', 's', 'h', 'a', 'm']);
    bff2.trav();
    println!("{}", bff2.size());

    bff2.delete(); // delete k
    bff2.delete(); // delete z

    bff2.trav(); // should be ehtesham again
    println!("{}", bff2.size()); // size = 8

    bff2.prepend(' ');
    bff2.prepend('.');
    bff2.prepend('r');
    bff2.prepend('m'); // should be "mr. ehtesham" now

    bff2.append(' ');
    bff2.append_from(vec!['s', 'i', 'd', 'd', 'i', 'q', 'u', 'i']);
    bff2.trav();
    println!("{}", bff2.size()); // size = 8

    bff2.delete();
    bff2.delete();
    bff2.delete();
    bff2.delete();

    while let Some(c) = bff2.take() {
        print!("{:?}", c);
    }
    println!("");
    bff2.trav();

    bff2.append('A');
    bff2.append('B');
    bff2.trav();

    bff2.take();
    bff2.trav();
    bff2.prepend('H');
    bff2.append('C');
    bff2.trav();

    bff2.map(|v| println!("tatastic {}", v));

    let mut inties = List::new(); 
    inties.append_from(vec![1,2,3,4,5]);
    inties.map(|v| print!("{},", v*2));
    println!("\n");


    let mut  bi = List::new();
    bi.append_from(vec![0,0,0,0,0,1]);
    let base :i32 = 2;
    let mut  pow =0;
    let mut  val :i32 =0;
    bi.map(|e| { 
        let powrr = base.pow(pow);
        println!("{} * {} pow {}", e, 2,powrr);
        val += e * powrr;
        pow = pow + 1;
    });

    println!("the conversion to decimal is {val}");

    // once we impl iterator, you can do this... 
    for ele in bi {
        println!("{}", ele);
    }
    

}
