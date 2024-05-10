use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

/// alias for poniter type
type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

/// Node is the internal struct that represents
/// a linked-list node. It wraps the data
/// and a pointer to the next node in the list
#[derive(Debug)]
struct Node<T: Debug + Clone + std::fmt::Display> {
    pub value: T,
    pub next: Pointer<T>,
}

/// SinglyLinkedList struct is the composite data structure
/// and represents a single linked list. The struct holds
/// two `Pointer`s, named head & curr. `head` is the classic
///
/// head: is the classic `head` & points to the first node
/// in the list, or None when the list is empty
///
/// curr: points to the last node in the list, or None when
/// the list is empty. When the list has only 1 node, it
/// points to the `head` of the list. Ths `curr` pointer is
/// an optimization so we dont have to traverse from `head`
/// to the last node when appending to the list.
///
/// The list composite data structure also holds the current
/// size of this linked list. The size is computed after every
/// mutable oeration, like append(), prepend(), delete() operation
/// & stored
///
/// To create a new, empty list, use the `new` associated
/// function.
///
/// Example
/// ```
///   use rustl::collections::SinglyLinkedList as List;
///
///   let mut list = List::new();
///   list.append(1);
///   list.prepend(0);
///   list.patch(1); // update the first element
///   assert_eq!(list.size(),2);   
///
/// ```
#[derive(Debug)]
pub struct SinglyLinkedList<T>
where
    T: Debug + Clone + std::fmt::Display,
{
    head: Pointer<T>, // points to head
    curr: Pointer<T>, // points to curr (last node)
    size: i32,
}

impl<T: Debug + Clone + std::fmt::Display> SinglyLinkedList<T> {
    
    /// creaes a new List with the head & curr pointers
    /// pointing to None (nil) & the size initialized to 0
    pub fn new() -> SinglyLinkedList<T> {
        return SinglyLinkedList {
            head: None,
            curr: None,
            size: 0,
        };
    }

    /// is_empty returns a boolean value, `true` is the
    /// head points to None, else `false` is returned
    pub fn is_empty(&self) -> bool {
        if let None = self.head {
            return true;
        }
        false
    }

    //// returns the current size of the list
    pub fn size(&self) -> i32 {
        return self.size;
    }

    /// append adds the supplied value at the end of the
    /// list. If the list is_empty(), i.e head is None, then
    /// the new node becomes the head of the list. The `curr`` also
    /// points to this node as it will also be the last node
    /// in the list.
    ///
    /// if the list was not empty, the new node is added at the
    /// end of the list & curr is moved to point to this node
    /// instead.
    ///
    /// Returns the new size of the list
    /// 
    /// Example: 
    /// ```
    /// ```   
    pub fn append(&mut self, value: T) -> i32 {
        let ele = Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }));

        if let None = self.head {
            self.curr = Some(Rc::clone(&ele)); // point curr to ele
            self.head = Some(ele); // point head to ele
            self.size += 1 // incr size
        } else {
            let mut copy = Some(Rc::clone(&ele));
            mem::swap(&mut self.curr, &mut copy); // swap curr & copy (old curr)
            mem::swap(&mut copy.unwrap().borrow_mut().next, &mut Some(ele)); // copy (old curr).next & new, new points
            self.size += 1;
        }
        return self.size; // new size
    }

    /// prepend adds the new value to the front of the list.
    ///  
    /// if prepend is called on an empty list, it behaves the same
    /// as `append`, the `head`` & `curr`` both point to the new node.
    ///
    /// if the list was not empty, the new node with the value is
    /// added to the front of the queue, front points to it, the
    /// previous head is then pointed by front->next.
    /// 
    /// Example: 
    /// ```
    /// ```
    pub fn prepend(&mut self, value: T) -> i32 {
        if let None = self.head {
            return self.append(value); // when list is empty, prepend is the same as append
        }

        // create the new element, wraps the value & take head_copy
        let mut ele = Some(Rc::new(RefCell::new(Node {
            value: value,
            next: mem::take(&mut self.head), // next takes head
        })));
        self.head = mem::take(&mut ele); // head take new_ele
        self.size += 1;
        self.size
    }

    /// patch replaces the `head` with the supplied value.
    ///
    /// - if the list is empty, then patch behaves the same
    ///   as append() or prepend()
    ///
    /// - if the list is not empty, patch updates the first
    ///   element with the supplied value.
    ///
    /// Example:
    /// ```
    ///   use rustl::collections::SinglyLinkedList as List;
    ///
    ///   let mut list = List::new();
    ///   list.append_from(vec![0,2,3]);  // list is 0,2,3
    ///   assert_eq!(list.size(),3);
    ///   list.patch(1);                  // list is 1,2,3
    ///   assert_eq!(list.size(),3);
    /// ```
    pub fn patch(&mut self, value: T) -> i32 {
        if let None = self.head {
            return self.append(value); // when list is empty, prepend is the same as append
        }

        if let Some(ref head) = self.head {
            // create the new element, wraps the value & take head_copy
            let mut ele = Some(Rc::new(RefCell::new(Node {
                value: value,
                next: head.borrow_mut().next.take(), // point to head.next
            })));
            self.head = mem::take(&mut ele);
        }
        self.size
    }

    /// append_from appends all items from the supplied Vec<T>
    /// the vec<t> is moved here;
    pub fn append_from(&mut self, values: Vec<T>) -> i32 {
        let mut sz: i32 = 0;
        for v in values {
            sz = self.append(v);
        }
        sz
    }

    /// prepend_from appends all items from the supplied Vec<T>
    /// the vec<t> is moved here;
    pub fn prepend_from(&mut self, values: Vec<T>) -> i32 {
        let mut sz: i32 = 0;
        for v in values {
            sz = self.prepend(v);
        }
        sz
    }

    /// delete always removes the item at the head of the list.
    ///
    /// - if the list is empty, this is a no-op.
    ///
    /// - if the list has 1 element, both head & curr will point to None
    ///   & the head will be removed.
    ///
    /// - if the list has more than 1 element, the head  is removed & heads
    ///   points to the next item in the node
    ///
    ///  Example:
    ///  ```
    ///   use rustl::collections::SinglyLinkedList as List;
    ///
    ///   let mut list = List::new();
    ///     list.append_from(vec![-1,0,1,2,3,4,5,6,7,8]);
    ///     list.delete();  // delete node -1
    ///     list.delete();  // delete node 0
    ///     assert_eq!(list.size(),8);
    ///  ```
    pub fn delete(&mut self) -> i32 {
        drop(self.take()); // take head drop it...
        self.size
    }

    /// take deletes the head & returns a clone (owned)
    /// value from the head of the list.
    /// Example:
    ///
    /// ```
    /// use rustl::collections::SinglyLinkedList as List;
    ///
    ///   let mut list = List::new();
    ///     list.append_from(vec![-1,0,1]);
    ///     list.delete();  // delete -1
    ///     list.delete();  // delete 0
    ///     assert_eq!(list.take().unwrap(),1); // take returns 2
    ///     assert_eq!(list.take(),None);       // take must return none
    ///     
    /// ```
    pub fn take(&mut self) -> Option<T> {
        if let None = self.head {
            return None;
        }

        // when list has elements
        // take curr when only 1 element in the list...
        if self.size == 1 {
            let _ = mem::take(&mut self.curr);
        }

        let detached_head = mem::take(&mut self.head); // detach head
        if let Some(ref detached_head_rc) = detached_head {
            mem::swap(&mut detached_head_rc.borrow_mut().next, &mut self.head); // point head to detachec head->next
            let v = detached_head_rc.borrow();
            self.size -= 1;
            let rc = Ref::map(v, |n| &n.value);
            return Some(rc.clone());
        }
        None
    }

    /// runs the supplied closure on the value of every node
    pub fn map<F>(&self, f: F)
    where
        F: Fn(&T),
    {
        if let Some(ref h) = self.head {
            let mut walker = Some(Rc::clone(h));
            while let Some(ele) = walker {
                let v = &ele.borrow().value;
                f(v);
                if let Some(next) = &ele.borrow().next {
                    walker = Some(Rc::clone(&next))
                } else {
                    walker = None;
                }
            }
        }

    }

    /// a temporary testing method
    /// prints the list size, some ---- markers at the begin & end 
    /// prints if Head or Curr are Nil 
    /// and finally walk the list & print each item with a print! macor
    pub fn trav(&self) {
        println!("-list({})-----------", self.size());
        if let None = self.head {
            println!(" Head: Nil")
        }
        if let None = self.curr {
            println!(" Curr: Nil")
        }
        self.map(|v| print!("{v}"));
        println!("\n-------------------");
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
