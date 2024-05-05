use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

pub fn to_weighted_adjacency_list(node_count: usize, edges: Vec<(usize, usize, f64)>) -> Vec<Vec<(usize,f64)>> {
    let mut weighted_adjacency_list: Vec<Vec<(usize,f64)>> = vec![vec![]; node_count];
    for edge in edges { //(node1, node2, weight)
        weighted_adjacency_list[edge.0].push((edge.1, edge.2));
        weighted_adjacency_list[edge.1].push((edge.0, edge.2));
    }
    return weighted_adjacency_list;
}

pub fn extract_weighted_edges(name: String) -> Vec<(usize,usize,f64)> {
    let edges = File::open(name).expect("Could not open file");
    let edge_reader = std::io::BufReader::new(edges).lines();
    let mut result: Vec<(usize, usize, f64)> = vec![];

    for line in edge_reader {

        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();

        let start_node = v[1].parse::<usize>().unwrap();
        let end_node = v[2].parse::<usize>().unwrap();
        let weight = v[3].parse::<f64>().unwrap();

        result.push((start_node, end_node, weight));
    }

    return result;
}

//
pub fn create_node_map() -> HashMap<i32, (f64,f64)> {
    
    let mut node_pos: HashMap<i32, (f64,f64)> = HashMap::new();

    // create key values from node.txt, file reader from lec27
    let nodes = File::open("sf_nodes.txt").expect("Could not open file");
    let node_reader = std::io::BufReader::new(nodes).lines();

    for line in node_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let id = v[0].parse::<i32>().unwrap();
        let x = v[1].parse::<f64>().unwrap();
        let y = v[2].parse::<f64>().unwrap();
        
        node_pos.insert(id, (x,y));
    }
    return node_pos;
}