import { ListNode } from "@/structures/linked_list";

const intToList = (nums: number[]): ListNode | null => {
    if (nums.length === 0) return null;

    let head = null;

    for (const num of nums.reverse()) {
        const node = new ListNode(num);
        node.next = head;
        head = node;
    }

    return head;
};

export { intToList };
