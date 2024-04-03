
pub mod collections; 


#[cfg(test)]
mod tests {
    use super::collections::LinkedList;

    #[test]
    fn list_size() {

        let mut list = LinkedList::new(0); 
        for v in 1..=9 {
            list.insert(v)
        }

        assert_eq!(list.size(), 10)
    }
}
