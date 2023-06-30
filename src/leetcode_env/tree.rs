//! Leetcode Tree Support

use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{Debug, Formatter},
    rc::Rc,
};

#[derive(PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn wrapped_node_maybe(val: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        val.map(|x| Rc::new(RefCell::new(Self::new(x))))
    }
}

// Wrapper class to make handling empty trees easier
#[derive(PartialEq, Eq)]
pub struct TreeRoot {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl Debug for TreeRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec: Vec<Option<i32>> = self.into();

        // Remove trailing None's
        while !vec.is_empty() && vec[vec.len() - 1].is_none() {
            let _ = vec.remove(vec.len() - 1);
        }

        let vec: Vec<String> = vec
            .iter()
            .map(|x| {
                if let Some(x) = x {
                    format!("{x}")
                } else {
                    "None".to_string()
                }
            })
            .collect();
        write!(f, "{vec:?}")
    }
}

impl From<&TreeRoot> for Vec<Option<i32>> {
    fn from(value: &TreeRoot) -> Self {
        let mut result = vec![];
        let mut deque = VecDeque::new();
        if value.root.is_some() {
            deque.push_back(value.root.clone());
        }
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            match &node {
                Some(node) => {
                    let node = node.as_ref().borrow();
                    result.push(Some(node.val));
                    deque.push_back(node.left.clone());
                    deque.push_back(node.right.clone());
                }
                None => {
                    result.push(None);
                }
            }
        }
        result
    }
}

impl From<Option<Rc<RefCell<TreeNode>>>> for TreeRoot {
    fn from(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }
}

impl From<&str> for TreeRoot {
    /// Expects the "[]" around the values, separated by comma "," and only
    /// integers and "null" (which is the format you'll get on LeetCode)
    ///
    /// # Panics
    ///
    /// This function panics if it doesn't match the expected format
    fn from(value: &str) -> Self {
        let mut result: Vec<Option<i32>>;
        // Check and remove "[]"
        assert!(value.len() >= 2);
        assert_eq!('[', value.chars().next().unwrap());
        assert_eq!(']', value.chars().nth(value.len() - 1).unwrap());
        if value.len() == 2 {
            // Empty array return empty tree
            return Self { root: None };
        }
        let value = &value[1..value.len() - 1];

        // Separate by comma
        let values: Vec<&str> = value.split(',').map(|v| v.trim()).collect();

        // Convert into values
        result = vec![];
        for value in values {
            result.push(if value == "null" {
                None
            } else {
                Some(value.parse().unwrap())
            })
        }

        result.into()
    }
}

impl Debug for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let left = if let Some(left) = &self.left {
            format!("{:?}", left.as_ref().borrow())
        } else {
            "None".to_string()
        };
        let right = if let Some(right) = &self.right {
            format!("{:?}", right.as_ref().borrow())
        } else {
            "None".to_string()
        };
        write!(f, "{{val:{} left:{} right:{}}}", self.val, left, right)
    }
}

impl From<Vec<i32>> for TreeRoot {
    fn from(value: Vec<i32>) -> Self {
        value
            .iter()
            .map(|x| Some(*x))
            .collect::<Vec<Option<i32>>>()
            .into()
    }
}

impl From<Vec<Option<i32>>> for TreeRoot {
    /// Converts the incoming vec into a tree
    fn from(list: Vec<Option<i32>>) -> Self {
        // Based on https://leetcode.com/problems/recover-binary-search-tree/solutions/32539/Tree-Deserializer-and-Visualizer-for-Python/

        if list.is_empty() {
            return TreeRoot { root: None };
        }

        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = list
            .iter()
            .map(|x| TreeNode::wrapped_node_maybe(*x))
            .collect();

        let mut kids: Vec<Option<Rc<RefCell<TreeNode>>>> = nodes
            .iter()
            .rev()
            .map(|x| x.as_ref().map(Rc::clone))
            .collect();

        let root = kids.pop().expect("Check for empty done at top");

        for node in nodes.into_iter().flatten() {
            if let Some(left_child) = kids.pop() {
                node.borrow_mut().left = left_child;
            }
            if let Some(right_child) = kids.pop() {
                node.borrow_mut().right = right_child;
            }
        }

        TreeRoot { root }
    }
}

impl From<TreeRoot> for Option<Rc<RefCell<TreeNode>>> {
    fn from(value: TreeRoot) -> Self {
        value.root
    }
}
