//average distances
use rand::Rng;
use std::collections::HashSet;
mod helper;

//given a point, computes bfs distances from selected node to every other, returns list of distanced from node
fn breadth_first_search(node: usize, node_count:usize, adjacency_list: &Vec<Vec<usize>>) ->  Vec<i32> {

    let mut visited: Vec<(bool, usize, usize)> = vec![(false, 0,0); node_count]; // (visited, previous node, total distance)
    visited[node] = (true, node, 0);
    let mut queue: Vec<usize> = vec![node];

    while !queue.is_empty() {
        let current_node = queue[0];
        
        for neighbor in &adjacency_list[current_node] {
            if !visited[*neighbor].0 { //if neighbor not visited
                queue.push(*neighbor);
                visited[*neighbor] = (true, current_node, visited[current_node].2 +1);

            } 
        }
        queue.remove(0);

    }

    let mut distances: Vec<i32> = vec![0; node_count];
    for i in 0..visited.len() {
        if visited[i].0 {
            distances[i] = visited[i].2 as i32;
        }
        else {
            distances[i] = -1; //using -1 to represnet unvisited points
        }
    }
    
    return distances;
}

/* 
fn dijkstras() {
    let mut visited: Vec<()>
}
 */

fn average_distances(node: usize, distances: Vec<i32>) -> f64 { //given a node and list of distances from node, compute average distance
    let mut sum: i32 = 0;
    let mut visited_count: f64 = 0.0;
    for i in 0..distances.len() {
        if i != node && distances[i] != -1 { //should not account for distance to self, and must be visited
            sum += distances[i];
            visited_count += 1.0;
        }
    }
    let average: f64 = sum as f64 / visited_count;
    if visited_count == 0.0 {return 0.0;}
    return average;
}

fn main() {
    
    // initialize adjacency list from txt file
    
    let node_count = 174956;
    let edges = helper::extract_edges(format!("sf_edges.txt"));
    let list = helper::to_adjacency_list(node_count, edges);

    let mut count =0;
    let mut sum: f64 = 0.00; // track sum
    let mut bank:HashSet<usize> = HashSet::new(); // hashset to ensure no repeated points

    let sampled = 3000;
    // find average of AVERAGE distance from 3000 random sampled points to rest of points in set
    while count < sampled {// 3000 random sampled points

        let random_node = rand::thread_rng().gen_range(0..node_count);
        if !bank.contains(&random_node) {
            let bfs = breadth_first_search(random_node, node_count, &list);
            sum += average_distances(random_node, bfs);

            bank.insert(random_node);
            count += 1;
        }   
    }

    println!("{}", sum / sampled as f64);

}

#[test]
fn test_bfs() {
    let edges = vec![(0,3), (1,2), (1,3), (3,4)];
    let node_count = 5;
    let list = helper::to_adjacency_list(node_count, edges);
    let bfs = breadth_first_search(1, node_count, &list);
    //     1   
    //    /  \
    //   2    3
    //      /  \
    //     0    4
    // from node 1, node 2 distance 1, node 3 distance 1, node 0 distance 2, node 4 distance 2
    // average distance should be (1 + 1 + 2 + 2) / 4 = 1.5
    assert_eq!(bfs, vec![2, 0, 1, 1, 2], "MISTAKE MADE");
    assert_eq!(average_distances(1, bfs), 1.5);
}

#[test]
fn test_bfs_loop() {
    let edges = vec![(0,1), (0,2), (1,3), (2,3)];
    let node_count = 4;
    let list = helper::to_adjacency_list(node_count, edges);
    let bfs = breadth_first_search(0, node_count, &list);
    //  0  -  1
    //  |     |
    //  2  -  3
    // from node 0, node 1 distance 1, node 2 distance 1, node 3 distance 2
    // average distance should be (1 + 1 + 2) / 3 = 1.333
    assert_eq!(bfs, vec![0, 1, 1, 2], "ERROR");
    assert_eq!(average_distances(0, bfs), 4.0 / 3.0, "ERROR");
}

#[test]
fn test_bfs_lonely() {
    let edges = vec![(1,3), (2,3)];
    let node_count = 4;
    let list = helper::to_adjacency_list(node_count, edges);
    let bfs = breadth_first_search(0, node_count, &list);
    //  0     1
    //        |
    //  2  -  3
    // from node 0, no points connected
    // average distance should be 0
    assert_eq!(bfs, vec![0, -1, -1, -1], "ERROR");
    assert_eq!(average_distances(0, bfs), 0.0, "ERROR");
}

#[test]
fn test_adjacency_list() {
    let edges = vec![(0,3), (1,2), (1,3), (3,4), (4,5), (6,2)];
    let node_count = 7;
    let list = helper::to_adjacency_list(node_count, edges);
    assert_eq!(list, vec![vec![3], vec![2,3], vec![1,6], vec![0, 1, 4], vec![3, 5], vec![4], vec![2]], "BIG ERROR");
}