use crate::structures::linked_list::ListNode;

pub fn int_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for num in nums.iter().rev() {
        let mut node = ListNode::new(*num);
        node.next = head;
        head = Some(Box::new(node));
    }

    head
}
