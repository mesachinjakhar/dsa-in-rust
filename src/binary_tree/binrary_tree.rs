use std::cmp::min;
use std::{cmp::max, collections::VecDeque, collections::BTreeMap};


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
        *ans = (*ans).max(left_count + right_count);
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
    let mut map = BTreeMap::new();

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

pub fn kth_level(root: &Option<Box<TreeNode>>, k: i32) {
    if root.is_none() {
        return;
    }

    let node = root.as_ref().unwrap();

    if k == 1 {
        println!("{}", node.val);
        return;
    }

    kth_level(&node.left, k - 1); 
    kth_level(&node.right, k - 1);

}

pub fn sum_tree(root: &mut Option<Box<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let left = sum_tree(&mut node.left);
        let right = sum_tree(& mut node.right);
        node.val = node.val + left + right;
        return node.val;
    };

    return 0;
}


pub fn search_helper(in_order: &Vec<i32>, value: i32, left: i32, right: i32) -> usize {
    for i in left..=right {
        if in_order[i as usize] == value {
            return i as usize;
        }
    }
    return 0
}


pub fn build_tree_using_pre_and_in_order(pre_order: &Vec<i32>, in_order: &Vec<i32>, pre_index: &mut usize, left: i32, right: i32) -> Option<Box<TreeNode>> {
    if left > right {
        return None;
    };

    let mut root = TreeNode {
        val: pre_order[*pre_index],
        left: None,
        right: None
    };

    let in_index = search_helper(in_order, pre_order[*pre_index], left, right);

    *pre_index += 1;

    let left = build_tree_using_pre_and_in_order(pre_order, in_order, pre_index, left, in_index as i32 - 1 );
    let right = build_tree_using_pre_and_in_order(pre_order, in_order, pre_index, in_index as i32 + 1, right);

    root.left = left;
    root.right = right;

    return Some(Box::new(root));
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
    pub struct TreeNode2 {
    val: i32,
    left: Option<Rc<RefCell<TreeNode2>>>,
    right: Option<Rc<RefCell<TreeNode2>>>,
}

  impl TreeNode2 {
    // create new leaf node
    pub fn new(val: i32) -> Self {
        TreeNode2 { val: val, left: None, right:None }
    }
}

#[derive(Debug, Clone)]
    pub struct TreeNode3 {
    val: i32,
    left: Option<Rc<RefCell<TreeNode3>>>,
    right: Option<Rc<RefCell<TreeNode3>>>,
    next: Option<Rc<RefCell<TreeNode3>>>,
}

pub fn morris_inorder(root: Option<Rc<RefCell<TreeNode2>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = root;

    while let Some(node_rc) = curr.clone() {
        let left = node_rc.borrow().left.clone();
        if left.is_none() {
            result.push(node_rc.borrow().val);
            curr = node_rc.borrow().right.clone();
        } else {
            let mut pred = left;
            while let Some(pred_rc) = pred.clone() {
                let right = pred_rc.borrow().right.clone();
                if right.is_none() || (right.is_some() && Rc::ptr_eq(right.as_ref().unwrap(), curr.as_ref().unwrap())) {
                    break;
                }
                pred = right;
            }

            let pred_rc = pred.unwrap();

            if pred_rc.borrow().right.is_none() {
                pred_rc.borrow_mut().right = curr.clone();
                curr = node_rc.borrow().left.clone();
            } else {
                pred_rc.borrow_mut().right = None;
                result.push(node_rc.borrow().val);
                curr = node_rc.borrow().right.clone();
            }
        }

    }

    result

}

pub fn delete(root: Option<Box<TreeNode>>, key: i32) -> Option<Box<TreeNode>>  {
    if root.is_none() {
        return None;
    }

    let mut root = root.unwrap();
    
    if key < root.val {
        root.left = delete(root.left, key);
        return Some(root)
    } else if key > root.val {
        root.right = delete(root.right, key);
        return Some(root)
    } else {
        // case 1: left and right  both are none
        if root.left.is_none() && root.right.is_none() {
            // todo delete node
            return None;
        }
        // case 2: 1 side is none; 
        if root.left.is_none() {
            return root.right;
        } else if root.right.is_none() {
            return root.left;
        } else {
            // both side are present
            let root_right = root.right.clone();
            let successor = find_successor(&root_right);
            root.val = successor;
            root.right = delete(root_right, successor);
            return Some(root);
        }

    }
}

fn find_successor(root: &Option<Box<TreeNode>>) -> i32 {
    let mut root = root.as_ref().unwrap();
    let mut successor = root.val;
    while !root.left.is_none() {
        root = root.left.as_ref().unwrap();
        successor = root.val;
    }

    return successor
}

pub fn inorder(root: Option<Box<TreeNode>>, vec1: &mut Vec<i32>) {
    if let Some(node) = root {
        inorder(node.left, vec1);
        vec1.push(node.val);
        inorder(node.right, vec1);
    }
    return;
}

pub fn helper(root1: Option<Box<TreeNode>>, root2: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut  vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let inorder1 = inorder(root1, &mut vec1); 
    let inorder2 = inorder(root2, & mut vec2);

    let mut temp = Vec::new();

    let mut i = 0; 
    let mut j = 0; 

    while i < vec1.len() && j < vec2.len() {
        if vec1[i] < vec2[j] {
            temp.push(vec1[i]);
            i += 1; 
        } else {
            temp.push(vec2[j]);
            j += 1;
        }
    }

    while i < vec1.len() {
        temp.push(vec1[i]);
    }

    while j < vec2.len() {
        temp.push(vec2[j]);
    }
    let  st = 0; 
    let end = temp.len() as i32 - 1 ;

    let root = build_tree(&temp, st, end);
    return root

}

fn build_tree(inorder: &Vec<i32>, st: i32, end: i32) -> Option<Box<TreeNode>> {
    if st > end {
        return None;
    }

    let mid = st + (end - st)/ 2; 
    let mut root = TreeNode::new(inorder[mid as usize]);
    root.left = build_tree(inorder,  st, mid - 1);
    root.right = build_tree(inorder, mid+1, end);

    return Some(Box::new(root))

}

struct largest {
    min: i32,
    max: i32,
    size: i32, 
    is_bst: bool
}

fn largest_bst(root: &Option<Box<TreeNode>>) -> largest {
    if let Some(node) = root { 
        let left = largest_bst(&node.left);
        let right = largest_bst(&node.right);

        if left.is_bst && right.is_bst && node.val > left.max && node.val < right.min {
            largest { min: min(left.min, node.val), max: max(right.max, node.val), size: left.size + right.size + 1, is_bst: true }
        } else {
            return  largest { min: i32::MIN, max: i32::MAX, size: max(left.size, right.size), is_bst: false};
        }

    }
    else {
        return largest { min: i32::MAX, max: i32::MIN, size: 0, is_bst: true };
    }
}

pub fn populate_next_right_pointers(root: Option<Rc<RefCell<TreeNode3>>>) -> Option<Rc<RefCell<TreeNode3>>>{
    if root.is_none() || root.as_ref().unwrap().borrow().left.is_none() {
        return None;
    }

    let mut queue: VecDeque<_> = VecDeque::new();
    let mut prev: Option<Rc<RefCell<TreeNode3>>> = None;
    queue.push_back(root.clone().unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut prev: Option<Rc<RefCell<TreeNode3>>> = None;

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();

            if let Some(p) = prev {
                p.borrow_mut().next = Some(node.clone());
            }

            prev = Some(node.clone());

            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            if let Some(l) = left {
                queue.push_back(l);
            } if let Some(r) = right {
                queue.push_back(r);
            }
        }
    }

    root

}


pub fn get_pred_succ(root: Option<Rc<RefCell<TreeNode2>>>, key: i32) -> (Option<Rc<RefCell<TreeNode2>>>, Option<Rc<RefCell<TreeNode2>>>) {
    let mut curr = root.clone();
    let mut pred = None;
    let mut succ = None;

    while let Some(node) = curr {
        if key < node.borrow().val {
            succ = Some(node.clone());
            curr = node.borrow().left.clone();
        } else if key > node.borrow().val {
            pred = Some(node.clone());
            curr = node.borrow().right.clone();
        } else {
            if node.borrow().left.is_some() {
                pred = find_pred(node.borrow().left.clone());
               
            } if node.borrow().right.is_some() {
                succ = find_succ(node.borrow().right.clone());
            }

            break;
        }
    }

    return (pred, succ)

}

pub fn find_pred(root: Option<Rc<RefCell<TreeNode2>>>) -> Option<Rc<RefCell<TreeNode2>>> {
    let mut ans = root.clone();
    let mut curr = root.clone();
    while let Some(node) = curr {
        ans = Some(node.clone());
        curr = node.borrow().right.clone();
    }
    return ans
}

pub fn find_succ(root: Option<Rc<RefCell<TreeNode2>>>) -> Option<Rc<RefCell<TreeNode2>>> {
    let mut ans = root.clone();
    let mut curr = root.clone();

    while let Some(node) = curr {
        ans = Some(node.clone());
        curr = node.borrow().left.clone();
    }

    return ans;
}




fn insert_val(root: Option<Rc<RefCell<TreeNode2>>>, value: i32) -> Option<Rc<RefCell<TreeNode2>>>{
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode2::new(value))));
    }

    let node = root.unwrap();
    let mut node_rc = node.borrow_mut();
    if node_rc.val > value {
        node_rc.left = insert_val(node_rc.left.clone(), value);
        return Some(node.clone());
    } else {
        node_rc.right = insert_val(node_rc.right.clone(), value);
        return Some(node.clone())
    }

}

fn floor(root: Option<Rc<RefCell<TreeNode2>>>, ans: &mut i32, key: i32) {
    if root.is_none() {
        return; 
    }

    let node = root.unwrap();
    let node_rc = node.borrow();
    if node_rc.val == key {
        *ans = node_rc.val;
        return;
    }

    else if node_rc.val > key {
        floor(node_rc.left.clone(), ans, key);
    } else {
        *ans = node_rc.val;
        floor(node_rc.right.clone(), ans, key);
    }
}

fn ciel(root: Option<Rc<RefCell<TreeNode2>>>, key: i32, ans: &mut i32) {
    if root.is_none() {
        return;
    }

    let node = root.unwrap();
    let node_rc = node.borrow();
    if node_rc.val == key {
        *ans = node_rc.val;
        return;
    }
    else if node_rc.val < key {
        ciel(node_rc.right.clone(), key, ans);
    } else {
        *ans = node_rc.val;
        ciel(node_rc.left.clone(), key, ans);
    }

}

fn range_sum(root: Option<Rc<RefCell<TreeNode2>>>, low: i32, high: i32, ans: &mut i32) {
    if root.is_none() {
        return; 
    }

    let node = root.unwrap();
    let node_rc = node.borrow();
    if node_rc.val >= low && node_rc.val <= high {
        *ans = *ans + node_rc.val;
        range_sum(node.borrow().left.clone(), low, high, ans);
        range_sum(node.borrow().right.clone(), low, high, ans);

    }
    else if node_rc.val < low {
        range_sum(node_rc.right.clone(), low, high, ans);
    } 
    
    else {
        range_sum(node_rc.left.clone(), low, high, ans);
    }

}

fn helper2(root: Option<Rc<RefCell<TreeNode2>>>, stack: &mut Vec<i32>, ans: &mut Vec<i32>) {
    if root.is_none() {
        stack.push(-1);
    }

    let node = root.unwrap(); 


    helper2(node.borrow().right.clone(), stack, ans);
    helper2(node.borrow().left.clone(), stack, ans);

 

}

struct MinHeap {
    data: Vec<i32>
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1); 
    } 

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = {index - 1}/2; 
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            }
            else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        let min = self.data[0];
        let last = self.data.pop().unwrap();

        if !self.data.is_empty() {
            self.data[0] = last; 
            self.heapify_down(0); 
        }

        Some(min)
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len(); 

        loop {
            let left = 2 * index + 1; 
            let right = 2 * index + 2; 
            let mut smallest = index; 

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }

            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.data.swap(index, smallest);
            index = smallest;
        }

    }
}


fn create_heap() {
    let mut heap = MinHeap::new(); 

    heap.push(5);
    heap.push(3);
    heap.push(8);
    heap.push(1);

    print!("{:?}", heap.pop());
    println!("{:?}", heap.pop())
}