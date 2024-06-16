use std::collections::HashMap;

struct Solution {}

impl Solution {
    /**
     * ! 1.两数之和
     * ? https://leetcode.cn/problems/two-sum/
     */
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // * 创建哈希表，用于存储已遍历元素及其索引
        let mut hash_map = HashMap::<i32, i32>::with_capacity(nums.len());

        for (key, value) in nums.iter().enumerate() {
            // * 计算目标值与当前元素的差值
            let diff = target - value;

            // * 如果哈希表中存在差值，直接返回差值与当前元素的索引
            // * 否则，将当前元素及其索引存入哈希表
            if let Some(index) = hash_map.get(&diff) {
                return vec![*index, key as i32];
            } else {
                hash_map.insert(*value, key as i32);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use serde::Deserialize;

    use super::Solution;

    #[derive(Debug, Deserialize)]
    struct Test {
        table1: Owner,
    }

    #[derive(Debug, Deserialize)]
    struct Owner {
        expected: Vec<Vec<i32>>,
        nums: Vec<Vec<i32>>,
        target: Vec<i32>,
    }

    struct Case {
        expected: Vec<i32>,
        nums: Vec<i32>,
        target: i32,
    }

    fn read_test_file() -> Vec<Case> {
        let (mut file, mut contents, mut cases) = (
            File::open("../test.toml").expect("无法打开文件"),
            String::new(),
            Vec::<Case>::new(),
        );

        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let test: Test = toml::from_str(&contents).expect("无法解析文件");

        for i in 0..test.table1.expected.len() {
            cases.push(Case {
                expected: test.table1.expected[i].clone(),
                nums: test.table1.nums[i].clone(),
                target: test.table1.target[i],
            })
        }

        cases
    }

    #[test]
    fn test_two_sum() {
        let cases = read_test_file();

        for c in cases {
            assert_eq!(c.expected, Solution::two_sum(c.nums.clone(), c.target))
        }
    }
}
