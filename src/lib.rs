pub mod collections;

pub use crate::collections::singly_linked_list; // re-export here

// #[cfg(test)]
// mod tests {
//     use super::collections::singly_linked_list::LinkedList;

//     #[test]
//     fn test_list_size() {

//         let mut list = LinkedList::new(0);
//         for v in 1..=9 {
//             list.insert(v)
//         }

//         assert_eq!(list.size(), 10)
//     }
// }
