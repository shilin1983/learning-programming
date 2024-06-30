import { ListNode } from "@/structures/linked_list";

/**
 * ! 2.两数相加
 * ? https://leetcode.cn/problems/add-two-numbers/
 */
export default function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
    const head = new ListNode();
    let curr = head, carry = 0;

    // * 同时遍历两个链表，逐位计算它们的和，并与当前位置的进位值相加
    // * 遍历结束后，如果还有进位值，则在链表末尾添加一个新的节点
    while (l1 !== null || l2 !== null || carry !== 0) {
        let sum = carry;

        if (l1 !== null) {
            sum += l1.val;
            l1 = l1.next;
        }

        if (l2 !== null) {
            sum += l2.val;
            l2 = l2.next;
        }

        curr.next = new ListNode(sum % 10);
        curr = curr.next;
        carry = Math.floor(sum / 10);
    }

    return head.next;
}
