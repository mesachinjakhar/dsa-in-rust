pub fn dijkstra_algo(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<i32> {
    let n = graph.len(); // graph len 
    let mut dist = vec![i32::MAX; n]; // distance vector

    dist[start] = 0;

    let mut queue: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new(); // priority queue
    queue.push(Reverse((0, start)));

    while let Some(Reverse((wt, node))) = queue.pop() {
        if wt > dist[node] {
            continue;
        }
        for &(neighbor, weight) in &graph[node] {
            let new_dist = wt + weight;
            if new_dist < dist[neighbor] {
                dist[neighbor] = new_dist;
                queue.push(Reverse((new_dist, neighbor)));
            }
        }
    }

    return dist;

}