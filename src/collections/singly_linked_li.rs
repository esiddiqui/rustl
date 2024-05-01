use std::fmt::Debug;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

type Pointer<T> = Option<Rc<RefCell<LiNode<T>>>>;

#[derive(Debug)]
pub struct LiNode<T:Debug> {
    pub value: T, 
    pub next :Pointer<T>
}

#[derive(Debug)]
pub struct List<T>  
  where T:Debug
{
    head: Pointer<T>, // points to head
    curr: Pointer<T>, // points to curr (last node)
    size: i32
}


impl<T:Debug> List<T> {

    pub fn new() -> List<T>{
        return List { head: None, curr: None, size: 0 }
    }

    /// is_empty returns a boolean value, `true` is the 
    /// head points to None, else `false` is returned
    pub fn is_empty(&self) -> bool {
        if let None = self.head {
            return true
        }
        false
    }

    /// append add the supplied `value` at the end of the 
    /// list, if the list is_empty() then T becomes the head
    /// node; Returns a value `true` if the operation was
    /// successful, or `false` if an error occurred
    pub fn append(&mut self, value: T) -> i32 {

        let node = LiNode{value: value, next: None};
        let ele = Rc::new(RefCell::new(node)); 

        if self.is_empty() {
            // setting node.next to 
            //node.next = std::mem::replace(&mut self.head, None);
            //self.head = node; 
            
            println!("strong count of ele is {}", Rc::strong_count(&ele));
            self.curr = Some(Rc::clone(&ele));
            println!("strong count of ele is {}", Rc::strong_count(&ele));
            self.head = Some(ele);
            self.size = self.size + 1; 
            return self.size
        } else {

            // create a rc::clone 
            // let mut ele_ref = Some(Rc::clone(&ele));
            println!("strong count of ele is {}", Rc::strong_count(&ele));
            let mut new_ele_clone = Some(Rc::clone(&ele)); // rc_ref (clone) of new ele
            println!("strong count of ele is {}", Rc::strong_count(&ele));
            let mut new_ele = Some(ele); // rc_ref of new ele
            mem::swap(&mut self.curr, &mut new_ele_clone );  // swap curr & new element
   
            if let Some(ref old_last_node) = new_ele_clone {
                mem::swap( &mut old_last_node.borrow_mut().next, &mut new_ele);  // swap new & new_clone, so 
                // &mut new.borrow_mut().next = mem::replace(self.curr)
                // self.size = self.size + 1; 
                // return tru
            }

           match new_ele {
                None => println!("yay, new ele is none we ca delete it"), 
                _ => println!("new ele still pointing to something shit !!")
            }

            match new_ele_clone {
                None => println!("yay, new ele clone is none we ca delete it"), 
                _ => println!("new ele clone still pointing to something shit !!")
            }


            // get rid of new_ele & new_ele_clone 
            self.size = self.size + 1; 

            return self.size
            
            // swap the self.curr & self.new element first
            // mem::swap( &mut self.curr.as_mut().next, &mut some_ele);
            // mem::swap(&mut self.curr, &mut some_ele );
            // mem::swap(&mut self.curr, &mut some_ele );

            // if let Some(ref mut new) = some_ele {
            //     //mem::swap( &mut new.borrow_mut().next, &mut None);
            //     &mut new.borrow_mut().next = mem::replace(self.curr)
            //     self.size = self.size + 1; 
            //     return true
            // }
        }
    }


    //// returns the current size of the list
    pub fn size(&self) ->i32 {
        return self.size;
    }


    /// internal helper to clone a pointers internal RefCell ref
    pub fn ptr_copy(mut ptr :Pointer<T>) -> Pointer<T> {
        let ele = mem::take(&mut ptr).unwrap();  //TODO: deal with panic
        let ptr_cpy = Some(Rc::clone(&ele));
        ptr = Some(ele);
        ptr_cpy
    }


    /// a temporary testing method 
    /// to walk the list & print each item
    pub fn trav(&mut self) {
        println!("travering list");
        // let walker : Pointer<T>;
        if !self.is_empty() {

            let head_ele = self.head.take().unwrap(); // take head 
            self.head =Some(Rc::clone(&head_ele));// head restored
            let mut walker = Some(head_ele);
            let mut done = false; 

            while !done {
                let ele = walker.unwrap(); 
                println!("{:?}",&ele.borrow().value);
                let next = &ele.borrow().next; 
                if let Some(next_ele) = &next {
                    walker = Some(Rc::clone(&next_ele));
                } else {
                    walker = None;
                    done = true; 
                }
            }
        }

        println!("-----------");
    }

}


/* 

            // get head 
            // let mut head = mem::take(&mut self.head);
            // if let Some(i) = head { 
            //     let head_clone = Rc::clone(&i);  
            //     // return header
            // }

            // let head_ele = mem::take(&mut self.head).unwrap();  //TODO: deal with panic
            // // let h = self.head.unwrap(); 
            // let walker = Some(Rc::clone(&head_ele));
            // self.head = Some(head_ele);
            
            // let head = mem::take(&mut self.head); // take head
            // let walker = head.


/// A single linked-list node definitions
pub enum LiNode<'a, T>
where T: Debug
{
    // here List Node holds a value & the next ptr
    Value{val: T, next: &'a LiNode<'a,T>},
    // here List Node is a nil; pointed by the LinNode from the previous node, or the head 
    // when empty list
    Nil
}

/// A singly Linked-list of nodes 
pub struct List<'a, T:Debug>  {
    pub head: &'a LiNode<'a, T>
}

impl <'a, T: Debug> List<'a, T> {

    // constructors
    pub fn new () -> List<'a, T> {
        return List{head: &LiNode::Nil}
    }

    pub fn new_with_head (head: &'a LiNode<'a, T>) -> List<'a, T> {
        return List{head: head}
    }

   // returns true is list is empty (head points to Nil node, else false)
    pub fn is_empty(&self) -> bool {
        match self.head {
            LiNode::Nil => true, 
            _ => false
        }
    }

    // prepend puts the new node in the front
    // pub fn prepend(&mut self, node: &'a LiNode<'a,T>) -> bool {

    //     if let LiNode::Nil = node { // cannot prepend a Nil node
    //         return false
    //     } 

    //     if self.is_empty() {
    //         self.head = node;
    //         return true 
    //     } 
        
    //     if let &LiNode::Value{ref val, mut next} = node {
    //        // incoming nodes, next points to current head  
    //        next = self.head;
    //        self.head = node; // head points to node
    //     } 
    //     return false
    // } 


    // pushes an element to the end of the list
    // pub fn push(&mut self, node: &'a LiNode<'a, T> ) -> bool {

    //     // if empty set node as head
    //     if self.is_empty() {
    //         self.head = node;
    //         return true 
    //     } 

    //     // when not empty, traverse the list till the 
    //     let mut ptr = self.head; 
        
    //     match ptr {
    //         LiNode::Value{val, ref mut next} => {
    //             if let mut next_ele = &LiNode::Nil {
    //                 next_ele = node; 
    //             } else {
    //                 ptr = next; 
    //             }
    //         }
    //         _ => return false // should not get here...
    //     }

    //     match self.head {
    //         LiNode::Nil => self.head = node, 
    //         _ => {
    //             let found = false;
    //             let mut ptr = self.head;
    //             while !found {
    //                  match ptr {
    //                     &LiNode::Value{next, ..} => {
    //                         if let mut next =  &LiNode::Nil {
    //                             next = node;
    //                         } else {
    //                             ptr = next;
    //                         }
    //                     }
    //                 };
    //             }// while
    //         }// match _
    //     }// match


    //     return true
    // }// push
}


    // // return true if the shit is empty
    // pub fn is_empty(&self) -> bool {
    //     self.size() == 0 
    // }

    // // returns the current count of items in the list
    // pub fn size(&self) -> u32 {
    //     match &self.next {
    //         Some(t) => 1 + t.size(), 
    //         _ => 1
    //     }
    // }
        

    //     return &arr 
    // }

        // returns an Option of &T to item at index 
    // pub fn item_at(&self, index: u32) -> Option<&T> {

    //     if self.is_empty() {
    //         return None
    //     }

    //     match index {
    //         0 => return Some(&self.val), 
    //         _ => return self.item_at(index - 1)
    //     }
    // }


    /*** 
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
 
    // TODO: remove this 
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

    // alias for insert(T)
    pub fn append(&mut self, val: T) {
        self.insert(val)
    }


    // // inserts the suppllied list at the end of the 
    // // list
    // pub fn insert_list(&mut self, other: LinkedList<T>) {
    //     for t in other {
    //         self.insert(t)
    //     }
    // }

    // // alias for insert(LinkedList<T>)
    // pub fn append_list(&mut self, other: LinkedList<T>) {
    //     self.insert(other)
    // }
    

}



#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_list_size() {

        let mut list = LinkedList::new(0); 
        for v in 1..=9 {
            list.insert(v)
        }

        assert_eq!(list.size(), 10)
    }


    #[test]
    fn test_list_to_string() {  
        let mut list = LinkedList::new('r'); 
        list.insert('u');
        list.insert('s');
        list.insert('t');
        list.insert('l');

        assert_eq!(list.to_string().as_str(),"'r''u''s''t''l'");
    }

    ***/


*/