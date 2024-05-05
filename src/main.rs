//average distances
use rand::Rng;
use std::{collections::HashSet, cmp::Ordering};
mod helper;

use std::collections::BinaryHeap;

//given a point, computes bfs distances from selected node to every other, returns list of distances from selected node
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
            distances[i] = -1; //using -1 to represent unvisited points
        }
    }
    
    return distances;
}

// given a point, computes dijkstra's shortest path from selected point to all others, returns list of distances
fn dijkstras(node: usize, node_count:usize, weighted_adjacency_list: &Vec<Vec<(usize, f64)>>) -> Vec<f64>{
    
    /*
    //assuming all points can be visited
    let mut visited_count: usize =0;

    //list where (visited, shortest dist) and index represents node
    let mut visited: Vec<(bool, f64)> = vec![(false, f64::MAX); node_count]; 

    fn next_shortest(list: &Vec<(bool,f64)>) -> usize { //given list, return next unvisited node with shortest dist
        let mut min: f64 = f64::MAX;
        let mut min_index: usize = 0;

        for i in 0..list.len() {
            if !list[i].0 && list[i].1 < min { //if unvisited and shortest dist
                min = list[i].1;
                min_index = i;
            }
        }
        return min_index;
    }

    let mut current_node: usize = node;
    visited[current_node] = (true, 0.0);
    visited_count += 1;

    while visited_count != node_count { //assuming every node can be visited

        for (neighbor,weight) in &weighted_adjacency_list[current_node] { //(node, weight)
            if !visited[*neighbor].0 && *weight + visited[current_node].1 < visited[*neighbor].1 { 
                // if neighbor unvisited and distance shorter, update distance
                visited[*neighbor].1 = *weight + visited[current_node].1;
            }
        }
        current_node = next_shortest(&visited);
        visited[current_node].0 = true;
        visited_count += 1;
    }

    let mut distances: Vec<f64> = vec![0.0; node_count];
    for i in 0..visited.len() {
        distances[i] = visited[i].1 as f64;
    }
    
    return distances;
     */

    // min binary heap for f64 from stackoverflow 
    // https://stackoverflow.com/questions/39949939/how-can-i-implement-a-min-heap-of-f64-with-rusts-binaryheap

    #[derive(PartialEq)]
    struct Rev(f64, usize);

    impl Eq for Rev {}

    impl PartialOrd for Rev {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.0.partial_cmp(&self.0)
        }
    }

    impl Ord for Rev {
        fn cmp(&self, other: &Rev) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    let mut pqueue = BinaryHeap::new(); //holds (dist, node)
    let mut dist: Vec<f64> = vec![f64::MAX; node_count];

    dist[node] = 0.0;
    pqueue.push(Rev(0.0, node));

    while !pqueue.peek().is_none() {
        let current_node = pqueue.pop().unwrap().1;

        for (neighbor, weight) in &weighted_adjacency_list[current_node] {
            if dist[current_node] + *weight < dist[*neighbor] {
                dist[*neighbor] = dist[current_node] + *weight;
                pqueue.push(Rev(dist[current_node] + *weight, *neighbor));
            }
        }
    }
    return dist;

}

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

fn average_weighted_distances(node: usize, distances: Vec<f64>) -> f64 { //given node and list of distances, compute average assuming all points visited
    let mut sum: f64 = 0.0;
    let mut visited_count: f64 = 0.0;
    for i in 0..distances.len() {
        if i != node { //should not account for distance to self
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

    println!("Breadth First Search Average: {}", sum / sampled as f64);
    
    let node_count = 174956;
    let edges = helper::extract_weighted_edges(format!("sf_edges.txt"));
    let weighted_adjacency_list = helper::to_weighted_adjacency_list(node_count, edges);

    let mut count =0;
    let mut sum: f64 = 0.00; // track sum
    let mut bank:HashSet<usize> = HashSet::new(); // hashset to ensure no repeated points

    let sampled = 1000;
    
    // find average of AVERAGE distance from 3000 random sampled points to rest of points in set
    while count < sampled {// 3000 random sampled points

        let random_node = rand::thread_rng().gen_range(0..node_count);
        if !bank.contains(&random_node) {
            let djk = dijkstras(random_node, node_count, &weighted_adjacency_list);
            sum += average_weighted_distances(random_node, djk);

            bank.insert(random_node);
            count += 1;
        }   
    }
    println!("Dijkstra's Algorithm Average: {}", sum / sampled as f64);
        
}

#[test]
fn test_dijkstras() {
        // [0] -1-   [1]   -2-   [2]
        // 3 nodes, connections between 0-1 with weight 1 and 1-2 weight 2
        // distances from point 0 should return vec [0, 1, 3]
        // average distance should be 2
        let edges: Vec<(usize, usize, f64)> = vec![(0,1,1.0), (1,2,2.0)];
        let node_count = 3;
        let weighted_adjacency_list = helper::to_weighted_adjacency_list(node_count, edges);
        let djk = dijkstras(0, node_count, &weighted_adjacency_list);
        assert_eq!(djk, vec![0.0,1.0,3.0], "ERROR");
        assert_eq!(average_weighted_distances(0, djk), 2.0, "ERROR");
}

#[test] 
fn test_dijkstras_multiple() {
    // [0]   -3- [2]  -1- [3]
    //   2\               /
    //        [1]     -5/
    let edges: Vec<(usize, usize, f64)> = vec![(0,1, 2.0), (0,2,3.0), (1,3,5.0), (2,3,1.0)];
    let node_count = 4;
    let weighted_adjacency_list = helper::to_weighted_adjacency_list(node_count, edges);
    let djk = dijkstras(0, node_count, &weighted_adjacency_list);
    assert_eq!(djk, vec![0.0, 2.0, 3.0, 4.0], "ERROR");
    assert_eq!(average_weighted_distances(0, djk), 3.0, "ERROR");
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