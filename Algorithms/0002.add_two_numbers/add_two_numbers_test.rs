mod add_two_numbers;
use add_two_numbers::Solution;
use add_two_numbers::ListNode;

fn main() {
    let arr1 = [9, 4, 5];
    let arr2 = [3, 4, 5];

    let arr1_last_index = arr1.len() - 1;
    let arr2_last_index = arr2.len() - 1;

    let mut list1 = Some(Box::new(ListNode::new(arr1[arr1_last_index])));
    let mut list2 = Some(Box::new(ListNode::new(arr2[arr2_last_index])));

    for i in (0..arr1_last_index).rev() {
        let mut ln = ListNode::new(arr1[i]);
        ln.next = list1;
        list1 = Some(Box::new(ln));
    }
    for i in (0..arr2_last_index).rev() {
        let mut ln = ListNode::new(arr2[i]);
        ln.next = list2;
        list2 = Some(Box::new(ln));
    }

    let results = Solution::add_two_numbers(list1, list2);
    println!("{:?}", results);
}
