
// use std::marker::Copy;

use std::fmt::Debug;

// an un-orderded linked list 
// doc to go here in future
#[derive(Debug)]
pub struct LinkedList<T: Debug> {
   pub val: T, 
   pub next: Option<Box<LinkedList<T>>>
}


impl <T: Debug> LinkedList<T> {

    // constructor
    pub fn new (head: T) -> LinkedList<T> {
        LinkedList{val: head, next: None}
    }

    // returns the current count of items in the list
    pub fn size(&self) -> u32 {
        match &self.next {
            Some(t) => 1 + t.size(), 
            _ => 1
        }
    }

    // converts & returns the contents of the list with the supplied
    // delim as the separator
    pub fn to_string_with_delim(&self, delim: char) -> String {
        match &self.next {
            Some(t) => {
                let sub = t.to_string_with_delim(delim);
                let sub = sub.as_str(); // shadowed 
                let mut s = String::from(format!("{:?}",self.val)); 
                s.push(delim);
                s.push_str(sub); 
                s
            } 
            _ =>  String::from(format!("{:?}",self.val))
        }
    }

        // converts & returns the contents of the list with the supplied
    // delim as the separator
    pub fn to_string(&self) -> String {
        match &self.next {
            Some(t) => {
                let sub = t.to_string();
                let sub = sub.as_str(); // shadowed 
                let mut s = String::from(format!("{:?}",self.val)); 
                s.push_str(sub); 
                s
            } 
            _ =>  String::from(format!("{:?}",self.val))
        }
    }
 
    // print_all prints all contents to std::out
    pub fn print_all(&self) {
       println!("{:?}",self.val);
       match self.next{
         Some(ref t) => t.print_all(),
         _ => println!("end")
       }
    }


    // insert the supplied node at the end of list
    // the list takes mutable ownership of the node
    pub fn insert(&mut self, val: T) {
        match self.next {
            Some(ref mut t) => t.insert(val),
            _ => { 
                let node = LinkedList{
                    val: val, 
                    next: None
                };        
                self.next = Some(Box::new(node)) 
            }
        }
    }

}

