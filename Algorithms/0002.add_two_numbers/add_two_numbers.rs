use std::cmp;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self { // ?? leetcode is private
        ListNode { next: None, val }
    }
}

pub struct Solution {
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut n1 = l1;
        let mut n2 = l2;
        let mut overflow_value = 0;
        let mut vec = Vec::new();

        while n1.is_some() || n2.is_some() || overflow_value > 0 {
            let v1 = n1.unwrap_or(Box::new(ListNode::new(0)));
            let v2 = n2.unwrap_or(Box::new(ListNode::new(0)));

            let mut v = v1.val + v2.val + overflow_value;
            overflow_value = v / 10;
            v = v % 10;

            vec.push(v);

            n1 = v1.next;
            n2 = v2.next;
        }

        let mut vec_last_index = vec.len() - 1;
        let mut results = Some(Box::new(ListNode::new(vec[vec_last_index])));

        for i in (0..vec_last_index).rev() {
            let mut ln = ListNode::new(vec[i]);
            ln.next = results;
            results = Some(Box::new(ln));
        }

        return results;
    }
}
