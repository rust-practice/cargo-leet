//! Leetcode Lists Support

use std::fmt::{Debug, Formatter};

/// Definition for singly-linked list.
#[derive(PartialEq, Eq)]
pub struct ListNode {
    /// The value stored at this node
    pub val: i32,
    /// Links to the next node if it exists
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
    /// Creates a new unlinked [ListNode] with the value passed
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

// TODO: Test the happy path of getting a linked list from a vec
impl From<Vec<i32>> for ListHead {
    fn from(values: Vec<i32>) -> Self {
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

// TODO: Test the happy path of going from a linked list to a vec
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
