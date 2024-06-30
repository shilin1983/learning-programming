<?php

namespace App\LeetCode\s0003_longest_substring_without_repeating_characters;

class Solution
{
    /**
     * ! 3.无重复字符的最长子串
     * ? https://leetcode.cn/problems/longest-substring-without-repeating-characters/
     */
    public function lengthOfLongestSubstring(string $s): int
    {
        $left = $right = $maxLength = 0;
        $hashMap = [];

        while ($right < strlen($s)) {
            // * 如果出现重复字符，则移动滑动窗口的左边界
            if (array_key_exists($s[$right], $hashMap) && $hashMap[$s[$right]] >= $left) {
                $left = max($left, $hashMap[$s[$right]] + 1);
            }

            // * 更新哈希表，并移动滑动窗口的右边界
            $hashMap[$s[$right]] = $right++;
            // * 计算滑动窗口的最大长度
            $maxLength = max($maxLength, $right - $left);
        }

        return $maxLength;
    }
}
