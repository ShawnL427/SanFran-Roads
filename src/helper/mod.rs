use std::fs::File;
use std::io::prelude::*;

pub fn extract_edges(name: String) -> Vec<(usize,usize)> {
    let edges = File::open(name).expect("Could not open file");
    let edge_reader = std::io::BufReader::new(edges).lines();
    let mut result: Vec<(usize, usize)> = vec![];

    for line in edge_reader {

        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();

        let start_node = v[1].parse::<usize>().unwrap();
        let end_node = v[2].parse::<usize>().unwrap();

        result.push((start_node, end_node));
    }

    return result;
}

pub fn to_adjacency_list(node_count: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {

    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; node_count];

    for edge in edges {
        adjacency_list[edge.0].push(edge.1);
        adjacency_list[edge.1].push(edge.0);
    }
    
    return adjacency_list;

}

pub fn to_weighted_adjacency_list(node_count: usize, edges: Vec<(usize, usize, usize)>) -> Vec<Vec<(usize,usize)>> {
    let mut weighted_adjacency_list: Vec<Vec<(usize,usize)>> = vec![vec![]; node_count];
    for edge in edges { //(node1, node2, weight)
        weighted_adjacency_list[edge.0].push((edge.1, edge.2));
        weighted_adjacency_list[edge.1].push((edge.0, edge.2));
    }
    return weighted_adjacency_list;
}
//pub fn to csv()