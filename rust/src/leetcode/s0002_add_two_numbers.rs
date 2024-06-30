use crate::structures::linked_list::ListNode;

struct Solution {}

impl Solution {
    /**
     * ! 2.两数相加
     * ? https://leetcode.cn/problems/add-two-numbers/
     */
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut curr, mut l1, mut l2, mut carry) = (&mut head, l1, l2, 0);

        // * 同时遍历两个链表，逐位计算它们的和，并与当前位置的进位值相加
        // * 遍历结束后，如果还有进位值，则在链表末尾添加一个新的节点
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum =
                carry + l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val);

            l1 = l1.as_mut().and_then(|n| n.next.take());
            l2 = l2.as_mut().and_then(|n| n.next.take());

            curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            curr = &mut curr.as_mut().unwrap().next;
            carry = sum / 10;
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use serde::Deserialize;

    use crate::utils::helpers::int_to_list;

    use super::Solution;

    #[derive(Debug, Deserialize)]
    struct Test {
        table2: Owner,
    }

    #[derive(Debug, Deserialize)]
    struct Owner {
        expected: Vec<Vec<i32>>,
        nums1: Vec<Vec<i32>>,
        nums2: Vec<Vec<i32>>,
    }

    struct Case {
        expected: Vec<i32>,
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    }

    fn read_test_file() -> Vec<Case> {
        let (mut file, mut contents, mut cases) = (
            File::open("../test.toml").expect("无法打开文件"),
            String::new(),
            Vec::<Case>::new(),
        );

        file.read_to_string(&mut contents).expect("无法读取文件");

        let test: Test = toml::from_str(&contents).expect("无法解析文件");

        for i in 0..test.table2.expected.len() {
            cases.push(Case {
                expected: test.table2.expected[i].clone(),
                nums1: test.table2.nums1[i].clone(),
                nums2: test.table2.nums2[i].clone(),
            })
        }

        cases
    }

    #[test]
    fn test_add_two_numbers() {
        let cases = read_test_file();

        for c in cases {
            assert_eq!(
                int_to_list(c.expected),
                Solution::add_two_numbers(int_to_list(c.nums1), int_to_list(c.nums2))
            )
        }
    }
}
