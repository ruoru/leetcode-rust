use super::list_node::ListNode;
use std::cmp;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut results: Option<Box<ListNode>>;
        let mut i1 = l1;
        let mut i2 = l2;
        let mut overflow_value = 0;

        while i1.is_some() || i2.is_some() {
            let v1 = i1.unwrap_or( Box::new(ListNode::new(0)) );
            let v2 = i2.unwrap_or( Box::new(ListNode::new(0)) );

            let mut v = v1.val + v2.val + overflow_value;
            overflow_value = v / 10;
            v = v % 10;

            results.next.get_or_insert(Box::new(ListNode::new(v)));
            curr = &mut results.next;

            i1 = v1.next;
            i2 = v2.next;
            println!( "{:?}",  v);
        }

        

        // println!( "{:?}",  i1.val);


        // let i2 = i1.next.unwrap_or( Box::new(ListNode::new(0)));

        // println!( "{:?}",  i2.val);

        // let i3 = i2.next.unwrap_or( Box::new(ListNode::new(0)));

        // println!( "{:?}",  i3.val);

        // let i4 = i3.next.unwrap_or( Box::new(ListNode::new(0)));

        // println!( "{:?}",  i4.val);
        
        // let mut results: Some;
        // let mut i1 = l1;
        // let mut i2 = l2;

        // let mut overfalow_value: i32 = 0;

        // while i1.is_some() || i2.is_some() {
        //     let mut v1 = i1.unwrap_or(0);
        //     let mut v2 = i2.unwrap_or(0);

        //     let mut v = v1 + v2 + overflow_value;
        //     overflow_value = v / 10;
        //     v = v % 10;

        //     results.next.push_back(Some(Box::new(ListNode::new(v))));

        //     i1 = iter1.next;
        //     i2 = iter2.next;
        // }
        // return results;

        return None;
    }
}

pub struct Solution {}