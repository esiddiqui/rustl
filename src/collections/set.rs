use std::fmt::Debug;

// a set with distinct values 
// implemented using https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
// doc to go here in future

#[derive(Debug)]
pub struct Set<T: Debug> {
   pub val: T, 
   pub next: Option<Box<Set<T>>>
}


impl <T: Debug> Set<T> {

    // constructor
    pub fn new (head: T) -> Set<T> {
        Set{val: head, next: None}
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
                let node = Set{
                    val: val, 
                    next: None
                };        
                self.next = Some(Box::new(node)) 
            }
        }
    }

}



    #[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn test_list_size() {

        let mut list = Set::new(0); 
        for v in 1..=9 {
            list.insert(v)
        }

        assert_eq!(list.size(), 10)
    }


    #[test]
    fn test_list_to_string() {  
        let mut list = Set::new('r'); 
        list.insert('u');
        list.insert('s');
        list.insert('t');
        list.insert('l');

        assert_eq!(list.to_string().as_str(),"'r''u''s''t''l'");
    }
}


