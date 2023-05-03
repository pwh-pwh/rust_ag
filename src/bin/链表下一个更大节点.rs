#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
        let mut cur = &head;
        let mut idx = -1;
        while let Some(node) = cur {
            idx += 1;
            ans.push(0);
            while !stack.is_empty() && node.val > stack.get(stack.len() - 1).unwrap().0 {
                ans[stack.pop_back().unwrap().1 as usize] = node.val;
            }
            stack.push_back((node.val, idx));
            cur = &node.next;
        }
        ans
    }
}

fn main() {}
