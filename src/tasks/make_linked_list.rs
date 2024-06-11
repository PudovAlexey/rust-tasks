use std::collections::LinkedList;

pub fn make_linked_list(array: Vec<&str>) -> LinkedList<&str> {
    let mut linked_list: LinkedList<&str> = LinkedList::new();

    for a in array {

        linked_list.push_back(a);
    }

    linked_list
}