use std::{cmp::max, collections::VecDeque};


#[derive(Debug, Clone)]
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

pub fn binrary_tree(preorder: &Vec<i32>, index: &mut usize) -> Option<Box<TreeNode>> {
    // handles both if current index exceed total length and if preorder length is 0 
    if *index >= preorder.len() {
        return None
    }

    if preorder[*index] == -1 {
        // consume by icrementing it
        *index = *index + 1;
        return None
    }

    let mut root = TreeNode {
        val: preorder[*index],
        left: None,
        right: None,
    };

    *index = *index + 1;

    root.left = binrary_tree(preorder, index);
    root.right = binrary_tree(preorder, index);
    Some(Box::new(root))
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

        in_order(&node.left, ans); // go left 
        ans.push(node.val); // add root
        in_order(&node.right, ans); // got right
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
        return
    }

    let mut queue: VecDeque<&Box<TreeNode>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap());

    while let Some(node) = queue.pop_front() {
        ans.push(node.val);

        if let Some(node) = &node.left {
            queue.push_back(node);
        }

        if let Some(node) = &node.right {
            queue.push_back(node);
        }
    }
}

pub fn height(root: &Option<Box<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let left_count = height(&node.left);
        let right_count = height(&node.right);
        return max(left_count, right_count) + 1
    }

    return 0;
}

pub fn height_helper(root: &Option<Box<TreeNode>>, ans: &mut i32) -> i32 {
    if let Some(node) = root {
        let left_count = height(&node.left);
        let right_count = height(&node.right);
        *ans = (*ans).max(left + height);
        return max(left_count, right_count) + 1
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

pub fn is_identical(p: &Option<Box<TreeNode>>, q: &Option<Box<TreeNode>>) -> bool {

    if p.is_none() || q.is_none() {
        return p.is_none() == q.is_none();
    };

    let is_left = is_identical(&p.as_ref().unwrap().left, &q.as_ref().unwrap().left);
    let is_right = is_identical(&p.as_ref().unwrap().right, &q.as_ref().unwrap().right);

    return is_left && is_right && p.as_ref().unwrap().val == q.as_ref().unwrap().val ;

}

pub fn diameter(root: &Option<Box<TreeNode>>) -> i32 {
    let mut ans = 0;
    height_helper(root, &mut ans);
    return ans;
}


pub fn top_view(root: &Option<Box<TreeNode>>) {
    if root.is_none() {
        return;
    }

    let mut queue: VecDeque<(&Box<TreeNode>, i32)> = VecDeque::new();
    queue.push_back((root.as_ref().unwrap(), 0));
    let mut map = HashMap::new();

    while let Some((node, horizontal_distance)) = queue.pop_front() {
        if !map.contains_key(&horizontal_distance) {
            map.insert(horizontal_distance, node.val);
        }

        if let Some(left) = node.left.as_ref() {
            queue.push_back((left, horizontal_distance-1)); 
        }

        if let Some(right) = node.right.as_ref() {
            queue.push_back((right, horizontal_distance + 1));
        }
    }

    let mut ans = Vec::new();

    for (_, val) in map.iter() {
        ans.push(val);
    }

    println!("Ans: {:?}", ans);

}