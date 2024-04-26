mod helper;

fn breadth_first_search(node: usize, node_count:usize, adjacency_list: Vec<Vec<usize>>) ->  Vec<usize>{

    let mut visited: Vec<(usize, usize)> = vec![]; // (node, distance)
    let mut queue: Vec<usize> = vec![node];

    while !queue.is_empty() && visited.len() != node_count {
        let current_node = queue[0];
        visited.push((current_node, visited[current_node].1 +1));

    }
    return(vec![]);
}

fn main() {
    
    //let edges = helper::extract_edges(format!("sf_edges.txt"));
    let edges = vec![(1,2), (1,3)];
    let list = helper::to_adjacency_list(4, edges);
    println!("{:?}", list);
    println!("{}", true && true);
    
}
