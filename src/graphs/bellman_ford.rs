pub fn bellman_ford(edges: &Vec<(usize, usize, i32)>, start: usize, n: usize) -> Option<Vec<i32>> {
    let mut dist = vec![i32::MAX; n];
    dist[start] = 0;


    // cal dist
    for _ in 0..n - 1{
        for &(u, v, weight) in edges {
            if dist[u] != i32::MAX && dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
            }
        }
    }

    // detect negative cycle
    // if a dist is still small then cal final dist list then there is a cycle 
    for &(u, v, weight) in edges {
        if dist[u] != i32::MAX && dist[u] + weight < dist[v] {
            return None;
        }
    }

    return Some(dist); 

}