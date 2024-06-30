<?php

class ListNode
{
    public function __construct(public int $val = 0, public ?self $next = null)
    {
    }
}
