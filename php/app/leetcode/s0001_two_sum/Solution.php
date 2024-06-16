<?php

namespace App\LeetCode\s0001_two_sum;

class Solution
{
    /**
     * ! 1.两数之和
     * ? https://leetcode.cn/problems/two-sum/
     */
    public function twoSum(array $nums, int $target): array
    {
        // * 创建哈希表，用于存储已遍历元素及其索引
        $hashMap = [];

        foreach ($nums as $key => $value) {
            // * 计算目标值与当前元素的差值
            $diff = $target - $value;

            // * 如果哈希表中存在差值，直接返回差值与当前元素的索引
            // * 否则，将当前元素及其索引存入哈希表
            if (array_key_exists($diff, $hashMap)) {
                return [$hashMap[$diff], $key];
            } else {
                $hashMap[$value] = $key;
            }
        }

        return [];
    }
}
