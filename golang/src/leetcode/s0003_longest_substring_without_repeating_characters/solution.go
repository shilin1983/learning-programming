package s0003_longest_substring_without_repeating_characters

/**
 * ! 3.无重复字符的最长子串
 * ? https://leetcode.cn/problems/longest-substring-without-repeating-characters/
 */
func lengthOfLongestSubstring(s string) int {
	left, right, maxLength, hashMap := 0, 0, 0, make(map[byte]int, len(s))

	for right < len(s) {
		// * 如果出现重复字符，则移动滑动窗口的左边界
		if idx, ok := hashMap[s[right]]; ok && idx >= left {
			left = idx + 1
		}

		// * 更新哈希表，并移动滑动窗口的右边界
		hashMap[s[right]] = right
		right++
		// * 计算滑动窗口的最大长度
		maxLength = max(maxLength, right-left)
	}

	return maxLength
}
