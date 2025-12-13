pub struct Node {
    val: i32,
    next: Option<Box<Node>>
}


pub fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    // create a prev None 
    let mut prev: Option<Box<Node>> = None;

    while let Some(mut curr) = head {

        // next pointer 
        head = curr.next.take();

        // reverse pointer
        curr.next = prev;

        // forward prev
        prev = Some(curr)

    }
    
    prev

}