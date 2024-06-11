
use std::collections::{LinkedList};

pub fn Merge_k_sorted_lists() -> LinkedList<i32> {

    let mut linked_list = LinkedList::from([
        LinkedList::from([1,4,5]),
        LinkedList::from([1,3,4]),
        LinkedList::from([2,6, 7]),
    ]);

    let mut result: LinkedList<i32> = LinkedList::new();

    for el in linked_list.iter() {

        for list_el in el.iter() {

            if result.is_empty() {
                result.push_back(list_el.to_owned())
            } else {
      
               let index = result.iter().enumerate()
                .find(|(_, &x)| x >= list_el.to_owned())
                .map(|(i, _)| i)
                .unwrap_or(usize::MAX);

                if index == usize::MAX {
                    result.push_back(list_el.to_owned())
                } else {

                    
                    let mut test = result.split_off(index);

                    result.push_back(list_el.to_owned());
                    result.append(&mut test);
                    
                    
                }

            }
        }
    }

    println!("{:?}", result);

    result
}