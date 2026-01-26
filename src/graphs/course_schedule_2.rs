pub fn is_cycle(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, path: &mut Vec<bool>) -> bool {
        visited[node] = true; 
        path[node] = true; 

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                if Self::is_cycle(neighbor, graph, visited, path) {
                    return true; 
                }
            } else if path[neighbor] {
                return true; 
            }
        }

        path[node] = false;
        return false;
    }

pub fn topo_sort(node: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<i32>) {
        visited[node] = true; 

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                Self::topo_sort(neighbor, graph, visited, stack);
            }
        }

        stack.push(node as i32);
    }


    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize; 
        let mut graph = vec![vec![]; n];
        let mut visited = vec![false; n];
        let mut path = vec![false; n]; 

        // create graph 
        for p in prerequisites {
            let course = p[0];
            let prereq = p[1]; 

            graph[prereq as usize].push(course as usize);
        }

        for i in 0..n {
            if !visited[i] {
                if Self::is_cycle(i, &graph, &mut visited, &mut path) {
            return vec![];
        } 
            }
        }

        let mut stack = Vec::new();
        let mut visited = vec![false; n];
        for i in 0..n {
    if !visited[i] {
        Self::topo_sort(i, &graph, &mut visited, &mut stack);
    }
}
        stack.reverse();
        return stack;
}
