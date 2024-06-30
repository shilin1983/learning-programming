package utils

import "golang/src/structures"

func IntToList(nums []int) *structures.ListNode {
	if len(nums) == 0 {
		return nil
	}

	var head *structures.ListNode

	for i := len(nums) - 1; i >= 0; i-- {
		node := structures.ListNode{Val: nums[i]}
		node.Next = head
		head = &node
	}

	return head
}
