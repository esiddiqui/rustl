
pub mod collections; 


#[cfg(test)]
mod tests {
    use super::collections::LinkedList;

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

        assert_eq!(list.to_string().as_str(),"r u s t l");
    }
}
