use std::collections::LinkedList;
use std::cmp;

pub fn add_two_numbers (list1: LinkedList<u32>, list2: LinkedList<u32>) -> LinkedList<u32> {
    let mut results: LinkedList<u32> = LinkedList::new();

    let length = cmp::max(list1.len(), list2.len()) + 1;
    let mut iter1 = list1.iter();
    let mut iter2 = list2.iter();

    let mut overflow_value: u32 = 0;

    for _i in 0..length {
        let i1 = iter1.next();
        let i2 = iter2.next();
        
        let mut v1 = 0;
        let mut v2 = 0;

        if i1.is_some() {
            v1 = *i1.unwrap();
        }
        if i2.is_some() {
            v2 = *i2.unwrap();
        }

        let mut v = v1 + v2 + overflow_value;

        overflow_value = v / 10;

        v = v % 10;

        results.push_back(v);
    }

    let last_value = results.pop_back();
    if last_value.is_some() && last_value.unwrap() != 0 {
        results.push_back(last_value.unwrap());
    }

    return results;
}

fn main() {
    let mut list1: LinkedList<u32> = LinkedList::new();
    let mut list2: LinkedList<u32> = LinkedList::new();

    list1.push_back(9);
    list1.push_back(4);
    list1.push_back(5);

    list2.push_back(3);
    list2.push_back(4);
    list2.push_back(5);
    list2.push_back(5);

    let results = add_two_numbers(list1, list2);
    println!("{:?}", results);
}
