//! Leetcode Lists Support
#![allow(clippy::module_name_repetitions)] // the type name is from leetcode, so we cannot modify it

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
    #[must_use]
        Self { next: None, val }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vec_to_linked_list() {
        // Arrange
        let start_vec = vec![1, 2, 3, 4, 5];
        let expected = create_linked_list(1..=5);

        // Act
        let list_head: ListHead = start_vec.into();
        let actual: Option<Box<ListNode>> = list_head.into();

        // Assert
        assert_eq!(actual, expected);
    }

    fn create_linked_list<I: DoubleEndedIterator<Item = i32>>(values: I) -> Option<Box<ListNode>> {
        let mut expected = None;
        for i in values.rev() {
            let mut new_node = Some(Box::new(ListNode::new(i)));
            new_node.as_mut().unwrap().next = expected;
            expected = new_node;
        }
        expected
    }

    #[test]
    fn from_linked_list_to_vec() {
        // Arrange
        let start: ListHead = create_linked_list(1..=5).into();
        let expected = vec![1, 2, 3, 4, 5];

        // Act
        let actual: Vec<i32> = (&start).into();

        // Assert
        assert_eq!(actual, expected);
    }
}
