use std::collections::VecDeque;

fn bfs(start: usize, graph: &Vec<Vec<usize>>) {
    let n = graph.len();
    let mut visited = vec![false, n];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        println!("{}", node); 

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

}

fn main() {
    let n = 4; // number of nodes
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(), n];

    graph[0].push(1);
    graph[1].push(0);

    graph[0].push(2);
    graph[2].push(0);

    graph[1].push(3);
    graph[3].push(2);

    


}