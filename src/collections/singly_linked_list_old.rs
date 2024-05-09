use std::fmt::Debug;

/// A singly linked-list
///
/// #Examples
/// ```
///   use rustl::collections::LinkedList;
///
///   let mut list = LinkedList::new('r');
///           
///    list.insert('u');
///    list.insert('s');
///    list.insert('t');
///    list.insert('l');
///
///   assert_eq!(list.to_string().as_str(),"'r''u''s''t''l'");
/// ```
#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    pub val: T,
    pub next: Option<Box<LinkedList<T>>>,
}

impl<T: Debug> LinkedList<T> {
    // constructors

    // returns an empty new linked list.
    pub fn new_empty() -> Option<LinkedList<T>> {
        return None;
    }

    pub fn new(head: T) -> LinkedList<T> {
        LinkedList {
            val: head,
            next: None,
        }
    }

    // return true if the shit is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    // returns the current count of items in the list
    pub fn size(&self) -> u32 {
        match &self.next {
            Some(t) => 1 + t.size(),
            _ => 1,
        }
    }

    // pub fn to_array(&self) -> Vec<T> {

    //     if self.size() == 0 {
    //         return vec![];
    //     }

    //     let v = vec![self.size()];
    //     for i in 0..=self.size()-1 {
    //         match self.item_at(i) {
    //             Some(t) => v.push(t.to_owned()),
    //             _ => panic!("something went terribly wrong, looking at index ")
    //         }
    //         v.push(self.item_at(i))
    //     }

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

    // converts & returns the contents of the list with the supplied
    // delim as the separator
    pub fn to_string_with_delim(&self, delim: char) -> String {
        match &self.next {
            Some(t) => {
                let sub = t.to_string_with_delim(delim);
                let sub = sub.as_str(); // shadowed
                let mut s = String::from(format!("{:?}", self.val));
                s.push(delim);
                s.push_str(sub);
                s
            }
            _ => String::from(format!("{:?}", self.val)),
        }
    }

    // converts & returns the contents of the list with the supplied
    // delim as the separator
    pub fn to_string(&self) -> String {
        match &self.next {
            Some(t) => {
                let sub = t.to_string();
                let sub = sub.as_str(); // shadowed
                let mut s = String::from(format!("{:?}", self.val));
                s.push_str(sub);
                s
            }
            _ => String::from(format!("{:?}", self.val)),
        }
    }

    // TODO: remove this
    // print_all prints all contents to std::out
    pub fn print_all(&self) {
        println!("{:?}", self.val);
        match self.next {
            Some(ref t) => t.print_all(),
            _ => println!("end"),
        }
    }

    // insert the supplied node at the end of list
    // the list takes mutable ownership of the node
    pub fn insert(&mut self, val: T) {
        match self.next {
            Some(ref mut t) => t.insert(val),
            _ => {
                let node = LinkedList {
                    val: val,
                    next: None,
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

        assert_eq!(list.to_string().as_str(), "'r''u''s''t''l'");
    }
}
