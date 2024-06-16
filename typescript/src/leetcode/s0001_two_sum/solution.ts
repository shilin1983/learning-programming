/**
 * ! 1.两数之和
 * ? https://leetcode.cn/problems/two-sum/
 */
export default function twoSum(nums: number[], target: number): number[] {
    // * 创建哈希表，用于存储已遍历元素及其索引
    const hashMap = new Map<number, number>();

    for (const [key, value] of nums.entries()) {
        // * 计算目标值与当前元素的差值
        const diff = target - value;

        // * 如果哈希表中存在差值，直接返回差值与当前元素的索引
        // * 否则，将当前元素及其索引存入哈希表
        if (hashMap.has(diff)) {
            return [hashMap.get(diff)!, key];
        } else {
            hashMap.set(value, key);
        }
    }

    return [];
}

