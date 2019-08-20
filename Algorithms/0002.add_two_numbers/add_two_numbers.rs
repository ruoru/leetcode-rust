
use std::collections::LinkedList;
use std::cmp;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut results: LinkedList<u32> = LinkedList::new();

//         let length = cmp::max(list1.len(), list2.len()) + 1;
//         let mut iter1 = list1.iter();
//         let mut iter2 = list2.iter();

//         let mut overflow_value: u32 = 0;

//         for _i in 0..length {
//             let i1 = iter1.next();
//             let i2 = iter2.next();
            
//             let mut v1 = 0;
//             let mut v2 = 0;

//             if i1.is_some() {
//                 v1 = *i1.unwrap();
//             }
//             if i2.is_some() {
//                 v2 = *i2.unwrap();
//             }

//             let mut v = v1 + v2 + overflow_value;

//             overflow_value = v / 10;

//             v = v % 10;

//             results.push_back(v);
//         }

//         let last_value = results.pop_back();
//         if last_value.is_some() && last_value.unwrap() != 0 {
//             results.push_back(last_value.unwrap());
//         }

//         return results;
//         }
// }
impl Solution {
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut one = l1.unwrap();
        let mut two = l2.unwrap();
        let mut root = ListNode::new(0);

        let mut res = Solution::make_node(one.val + two.val);
        root.next.get_or_insert(Box::new(res.0));

        let mut curr = &mut root.next;

        while one.next.is_some() || two.next.is_some() {
            match curr {
                None => break,
                Some(now) => {
                    one = one.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    two = two.next.or(Some(Box::new(ListNode::new(0)))).unwrap();

                    res = Solution::make_node(one.val + two.val + res.1);

                    now.next.get_or_insert(Box::new(res.0));
                    curr = &mut now.next;
                }
            }
        }

        if res.1 > 0 {
            if let Some(now) = curr {
                now.next.get_or_insert(Box::new(ListNode::new(res.1)));
            }
        }

        root.next
    }

    fn make_node(mut result: i32) -> (ListNode, i32) {
        let single;
        if result > 9 {
            single = 1;
            result = result - 10;
        } else {
            single = 0;
        }
        (ListNode::new(result), single)
    }
}

pub struct Solution {}