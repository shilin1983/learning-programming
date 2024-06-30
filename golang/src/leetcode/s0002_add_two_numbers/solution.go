package s0002_add_two_numbers

import "golang/src/structures"

type ListNode = structures.ListNode

/**
 * ! 2.两数相加
 * ? https://leetcode.cn/problems/add-two-numbers/
 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	head := &ListNode{Val: 0}
	curr, carry := head, 0

	// * 同时遍历两个链表，逐位计算它们的和，并与当前位置的进位值相加
	// * 遍历结束后，如果还有进位值，则在链表末尾添加一个新的节点
	for l1 != nil || l2 != nil || carry != 0 {
		sum := carry

		if l1 != nil {
			sum += l1.Val
			l1 = l1.Next
		}

		if l2 != nil {
			sum += l2.Val
			l2 = l2.Next
		}

		curr.Next = &ListNode{Val: sum % 10}
		curr = curr.Next
		carry = sum / 10
	}

	return head.Next
}
