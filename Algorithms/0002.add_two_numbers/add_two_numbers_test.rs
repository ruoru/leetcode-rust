mod add_two_numbers;
use add_two_numbers::Solution;

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
