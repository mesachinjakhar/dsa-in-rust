pub fn optimize_approach(head: Option<Box<Node>>) -> bool {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let (Some(s), Some(f)) = (slow, fast) {
        slow = slow.next.as_ref();

        if let Some(f_next) = f.next.as_ref() {
            fast = f_next.next.as_ref();
        } else {
            return false;
        }

        // compare
        if let (Some(s_node), Some(f_node)) = (slow, fast) {
            if std::ptr::eq(s_node, f_node) {
                return true;
            }
        }

    }

    false
}