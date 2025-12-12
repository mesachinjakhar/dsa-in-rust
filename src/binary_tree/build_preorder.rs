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

pub fn build_preorder(preorder: & Vec<i32>, index: &mut usize) -> Option<Box<TreeNode>> {

    if *index == preorder.len() {
        return None;
    }

    let val = preorder[*index];

    if val == -1 {
        return None;
    }

    let mut node = TreeNode::new(val);
    *index += 1;
    node.left = build_preorder(preorder, index);
    *index += 1;
    node.right = build_preorder(preorder, index);

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

        pre_order(&node.left, ans);
        ans.push(node.val);
        pre_order(&node.right, ans);
    }
}