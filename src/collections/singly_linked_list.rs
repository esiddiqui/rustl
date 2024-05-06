use std::fmt::Debug;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;


/// Node is the internal struct that represents 
/// a linked-list node. It wraps the data 
/// and a pointer to the next node in the list
#[derive(Debug)]
pub struct Node<T:Debug> {
    pub value: T, 
    pub next :Pointer<T>
}


/// SinglyLinkedList struct is the composite data structure
/// that represents a single linked list. The struct holds 
/// two `Pointer`s, head & curr. 
/// 
/// head: points to the first node in the list, or None when 
/// the list is empty
/// 
/// curr: points to the list node in the list, or None when 
/// the list is empty. When the list has only 1 node, it 
/// points to the `head` of the list. Ths `curr` pointer is
/// an optimization so we dont have to traverse from `head`
/// to the last node when append-inn an item to the list.
/// 
/// The list data structure also holds the current size of
/// this linked list. The size is computed after every mutable
/// oeration, like append(), prepend, delete() operation & 
/// stored
/// 
/// To create a new, empty list, use the `new` associated 
/// function. 
/// 
/// Example 
/// ```
///   use rustl::collections::SinglyLinkedList; 
/// 
///   let mut list = rustl::collections::SinglyLinkedList::new();
///   list.append(1); 
///   list.prepend(0); 
///   assert_eq!(list.size(),2);   
/// 
/// ```
#[derive(Debug)]
pub struct SinglyLinkedList<T>  
  where T:Debug
{
    head: Pointer<T>, // points to head
    curr: Pointer<T>, // points to curr (last node)
    size: i32
}


impl<T:Debug> SinglyLinkedList<T> {


    /// creaes a new List with the head & curr pointers
    /// pointing to None (nil) & the size initialized to 0
    pub fn new() -> SinglyLinkedList<T>{
        return SinglyLinkedList { head: None, curr: None, size: 0 }
    }

    /// is_empty returns a boolean value, `true` is the 
    /// head points to None, else `false` is returned
    pub fn is_empty(&self) -> bool {
        if let None = self.head {
            return true
        }
        false
    }


    /// append adds the supplied value at the end of the 
    /// list. If the list is_empty(), i.e head is None, then 
    /// the new node becomes the head of the list. The curr also 
    /// points to this same node as it will also be the last node 
    /// in the list. 
    /// 
    /// if the list was not empty, the new node is added at the
    /// end of the list & curr is moved to point to this node 
    /// instead.
    /// 
    /// Returns the new size of the list
    /// 
    pub fn append(&mut self, value: T) -> i32 {

        // we make a new node and put the value in it, next is None; 
        let node = Node{value: value, next: None};
        // put the node in the RefCel & 
        let ele = Rc::new(RefCell::new(node)); 

        // if list is
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

        }
    }

    

    /// prepend adds the new value to the front of the queue.
    ///  
    /// if prepend is called on an empty list, it behaves the same
    /// as `append`, the head & curr both point to the new node.
    /// 
    /// if the list was not empty, the new node with the value is 
    /// added to the front of the queue, front points to it, the 
    /// previous head is then pointed by front->next.
    /// 
    pub fn prepend(&mut self, value:T) -> i32 {

         // we make a new node and put the value in it, next is None; 
         let node = Node{value: value, next: None};
         // wrap it up inside an Rc/RefCell
         let ele = Rc::new(RefCell::new(node)); 
 
         if self.is_empty() {
             self.curr = Some(Rc::clone(&ele)); // curr ->rc clone 
             self.head = Some(ele); //head -> rc
             self.size += 1; // size++
             return self.size
         } else {

            // new_ele & new_ele_clone now
            let mut new_ele_clone = Some(Rc::clone(&ele)); 
            let new_ele = Some(ele);

            // take head 
            mem::swap(&mut self.head, &mut new_ele_clone); // head points to new node, new_ele_clone points to old head
            if let Some(now_head) = new_ele {
                mem::swap(&mut  now_head.borrow_mut().next, &mut new_ele_clone) // now_head->next points to old head
            }
            self.size += 1; // size++
            return self.size // return size
         }
    }




    //// returns the current size of the list
    pub fn size(&self) ->i32 {
        return self.size;
    }


    /// append_from appends all items from the supplied Vec<T>
    /// the vec<t> is moved here; 
    pub fn append_from(&mut self, values:Vec<T>) -> i32{
        let mut sz: i32 = 0;
        for v in values {
            sz = self.append(v);
        }
        sz
    }


    /// prepend_from appends all items from the supplied Vec<T>
    /// the vec<t> is moved here; 
    pub fn prepend_from(&mut self, values:Vec<T>) -> i32{
        let mut sz: i32 = 0;
        for v in values {
            sz = self.prepend(v);
        }
        sz
    }


    /// internal helper to clone a pointers internal RefCell ref
    // pub fn ptr_copy(mut ptr :Pointer<T>) -> Pointer<T> {
    //     let ele = mem::take(&mut ptr).unwrap();  //TODO: deal with panic
    //     let ptr_cpy = Some(Rc::clone(&ele));
    //     ptr = Some(ele);
    //     ptr_cpy
    // }


    /// a temporary testing method 
    /// to walk the list & print each item
    pub fn trav(&mut self) {
        println!("----------- travering list -----------");
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

        println!("----- --------- ------");
    }

}


#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn test_list_size() {

        let mut list = SinglyLinkedList::new(); 
        for v in 1..=9 {
            list.append(v);
        }

        assert_eq!(list.size(), 9)
    }


    // #[test]
    // fn test_list_to_string() {  
    //     let mut list = SinglyLinkedList::new('r'); 
    //     list.insert('u');
    //     list.insert('s');
    //     list.insert('t');
    //     list.insert('l');

    //     assert_eq!(list.to_string().as_str(),"'r''u''s''t''l'");
    // }
}