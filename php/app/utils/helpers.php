<?php

function intToList(array $nums): ?ListNode
{
    if (empty($nums)) return null;

    $head = null;

    foreach (array_reverse($nums) as $num) {
        $node = new ListNode($num);
        $node->next = $head;
        $head = $node;
    }

    return $head;
}
