
use std::collections::LinkedList;
use std::cmp;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut results: ListNode::new(0);
        let mut i1 = l1;
        let mut i2 = l2;

        let mut overfalow_value: i32 = 0;

        while i1.is_some() || i2.is_some() {
            let mut v1 = i1.unwrap_or(0);
            let mut v2 = i2.unwrap_or(0);

            let mut v = v1 + v2 + overflow_value;
            overflow_value = v / 10;
            v = v % 10;

            results.next.push_back(Some(Box::new(ListNode::new(v))));

            i1 = iter1.next;
            i2 = iter2.next;
        }
        return results;
    }
}

pub struct Solution {}