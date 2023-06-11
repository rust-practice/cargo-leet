//! Leetcode Lists Support

use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Debug for ListNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.val,
            match self.next.as_ref() {
                Some(next) => format!("{next:?}"),
                None => "None".to_owned(),
            }
        )
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Wrapper class to make handling empty lists easier
#[derive(PartialEq, Eq)]
pub struct ListHead {
    head: Option<Box<ListNode>>,
}

impl Debug for ListHead {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let head: Vec<i32> = self.into();
        head.fmt(f)
    }
}

impl From<ListHead> for Option<Box<ListNode>> {
    fn from(value: ListHead) -> Self {
        value.head
    }
}

impl From<Option<Box<ListNode>>> for ListHead {
    fn from(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }
}

impl From<Vec<i32>> for ListHead {
    fn from(values: Vec<i32>) -> Self {
        // Reverse version before looking at
        // https://github.com/zwhitchcox/leetcode/blob/master/src/0002_add_two_numbers.rs
        // to see how it could be done going forward instead of backward
        //
        // let mut last: Option<Box<ListNode>> = None;
        // for &n in values.iter().rev() {
        //     let mut temp = ListNode::new(n);
        //     temp.next = last;
        //     last = Some(Box::new(temp));
        // }
        // ListHead::new(last)

        let mut result = Self { head: None };
        let mut curr = &mut result.head;
        for &num in &values {
            let node = ListNode::new(num);
            *curr = Some(Box::new(node));
            curr = &mut curr.as_mut().unwrap().next;
        }
        result
    }
}

impl From<&ListHead> for Vec<i32> {
    fn from(list_head: &ListHead) -> Self {
        let mut result = vec![];
        let mut curr = &list_head.head;
        while let Some(node) = &curr {
            result.push(node.val);
            curr = &node.next;
        }
        result
    }
}