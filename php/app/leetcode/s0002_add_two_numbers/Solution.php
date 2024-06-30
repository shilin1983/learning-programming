<?php

namespace App\LeetCode\s0002_add_two_numbers;

use ListNode;

class Solution
{
    /**
     * ! 2.两数相加
     * ? https://leetcode.cn/problems/add-two-numbers/
     */
    public function addTwoNumbers(ListNode $l1, ListNode $l2): ListNode
    {
        $head = $curr = new ListNode();
        $carry = 0;

        // * 同时遍历两个链表，逐位计算它们的和，并与当前位置的进位值相加
        // * 遍历结束后，如果还有进位值，则在链表末尾添加一个新的节点
        while (!is_null($l1) || !is_null($l2) || $carry !== 0) {
            $sum = $carry + ($l1->val ?? 0) + ($l2->val ?? 0);

            $l1 = $l1->next ?? null;
            $l2 = $l2->next ?? null;

            $curr->next = new ListNode($sum % 10);
            $curr = $curr->next;
            $carry = $sum >= 10 ? 1 : 0;
        }

        return $head->next;
    }
}
