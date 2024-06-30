/**
 * ! 3.无重复字符的最长子串
 * ? https://leetcode.cn/problems/longest-substring-without-repeating-characters/
 */
export default function lengthOfLongestSubstring(s: string): number {
    let left = 0, right = 0, maxLength = 0, hashMap = new Map<string, number>();

    while (right < s.length) {
        // * 如果出现重复字符，则移动滑动窗口的左边界
        if (hashMap.has(s[right]) && hashMap.get(s[right])! >= left) {
            left = Math.max(left, hashMap.get(s[right])! + 1);
        }

        // * 更新哈希表，并移动滑动窗口的右边界
        hashMap.set(s[right], right++);
        // * 计算滑动窗口的最大长度
        maxLength = Math.max(maxLength, right - left);
    }

    return maxLength;
}
