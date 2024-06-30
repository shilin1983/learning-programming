use std::collections::HashMap;

struct Solution {}

impl Solution {
    /**
     * ! 3.无重复字符的最长子串
     * ? https://leetcode.cn/problems/longest-substring-without-repeating-characters/
     */
    fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right, mut max_length, mut hash_map) =
            (0, 0, 0, HashMap::<char, i32>::with_capacity(s.len()));

        while right < s.len() {
            // * 如果出现重复字符，则移动滑动窗口的左边界
            if let Some(idx) = hash_map.get(&s.chars().nth(right).unwrap()) {
                if *idx >= left {
                    left = *idx + 1;
                }
            }

            // * 更新哈希表，并移动滑动窗口的右边界
            hash_map.insert(s.chars().nth(right).unwrap(), right as i32);
            right += 1;
            // * 计算滑动窗口的最大长度
            max_length = max_length.max(right as i32 - left);
        }

        max_length
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
        table3: Owner,
    }

    #[derive(Debug, Deserialize)]
    struct Owner {
        expected: Vec<i32>,
        s: Vec<String>,
    }

    struct Case {
        expected: i32,
        s: String,
    }

    fn read_test_file() -> Vec<Case> {
        let (mut file, mut contents, mut cases) = (
            File::open("../test.toml").expect("无法打开文件"),
            String::new(),
            Vec::<Case>::new(),
        );

        file.read_to_string(&mut contents).expect("无法读取文件");

        let test: Test = toml::from_str(&contents).expect("无法解析文件");

        for i in 0..test.table3.expected.len() {
            cases.push(Case {
                expected: test.table3.expected[i],
                s: test.table3.s[i].clone(),
            })
        }

        cases
    }

    #[test]
    fn test_length_of_longest_substring() {
        let cases = read_test_file();

        for c in cases {
            assert_eq!(c.expected, Solution::length_of_longest_substring(c.s))
        }
    }
}
