package s0001_two_sum

/**
 * ! 1.两数之和
 * ? https://leetcode.cn/problems/two-sum/
 */
func twoSum(nums []int, target int) []int {
	// * 创建哈希表，用于存储已遍历元素及其索引
	hashMap := make(map[int]int, len(nums))

	for key, value := range nums {
		// * 计算目标值与当前元素的差值
		diff := target - value

		// * 如果哈希表中存在差值，直接返回差值与当前元素的索引
		// * 否则，将当前元素及其索引存入哈希表
		if idx, ok := hashMap[diff]; ok {
			return []int{idx, key}
		} else {
			hashMap[value] = key
		}
	}

	return []int{}
}
