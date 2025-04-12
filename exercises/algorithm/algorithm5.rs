/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,   // 使用邻接矩阵来表示图：adj_ij = 顶点vi与vj之间边的情况
}

impl Graph {
    // Create a new graph with n vertices
    // 创建有n个顶点的图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    // 看来这是一个无向图：src <=> dst，边是相互的
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        let mut visit_order = vec![];
        let mut visited = vec![false; self.adj.len()];  // 标记各个节点是否被遍历过了
        let mut queue = VecDeque::new();

        // BFS的思路：使用队列存储从来没有访问过的，当前节点的邻居，然后依次出队遍历所有的节点
        // 首先标记起始节点为“访问过了”
        visited[start] = true;
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // node出队才算是访问
            // 将该node的所有从未访问过的邻居节点加入到队列中
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

