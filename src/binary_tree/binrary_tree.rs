use std::{cmp::max, collections::VecDeque};


#[derive(Debug)]
    pub struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

    impl TreeNode {
    // create new leaf node
    pub fn new(val: i32) -> Self {
        TreeNode { val: val, left: None, right:None }
    }
}

pub fn binrary_tree(preorder: & Vec<i32>, index: &mut usize) -> Option<Box<TreeNode>> {

    if *index == preorder.len() {
        return None;
    }

    let val = preorder[*index];

    if val == -1 {
        return None;
    }

    let mut node = TreeNode::new(val);
    *index += 1;
    node.left = binrary_tree(preorder, index);
    *index += 1;
    node.right = binrary_tree(preorder, index);

    Some(Box::new(node))

}


pub fn pre_order(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {

        ans.push(node.val);

        pre_order(&node.left, ans);

        pre_order(&node.right, ans);
    }
}

pub fn in_order(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {

        in_order(&node.left, ans);
        ans.push(node.val);
        in_order(&node.right, ans);
    }
}

pub fn post_order(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {

        post_order(&node.left, ans);
        post_order(&node.right, ans);
        ans.push(node.val);
    }
}

pub fn level_order(root: &Option<Box<TreeNode>>, ans: &mut Vec<i32>) {
    if root.is_none() {
        return; 
    }

    let mut queue: VecDeque<&Box<TreeNode>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap()); // extract as reference -> then unrwap the option so we get Box , without unwrap we will get Some

    while let Some(node) = queue.pop_front() {
        ans.push(node.val);

        if let Some(left) = &node.left {
            queue.push_back(left);
        }

        if let Some(right) = &node.right {
            queue.push_back(right);
        }

    }

}

pub fn height(root: &Option<Box<TreeNode>>) -> i32 {

    if let Some(node) = root {
         let left = height(&node.left);
         let right = height(&node.right);

         return max(left, right) + 1;
    
    }

    return 0;
}

pub fn count(root: &Option<Box<TreeNode>>) -> i32 {

    if let Some(node) = root {
         let left = count(&node.left);
         let right = count(&node.right);

         return left + right + 1;
    
    }

    return 0;
}

pub fn sum(root: &Option<Box<TreeNode>>) -> i32 {

    if let Some(node) = root {
         let val = node.val;
         let left = sum(&node.left);
         let right = sum(&node.right);

         return val + left + right;

    }

    return 0;
}