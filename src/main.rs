//average distances
use rand::Rng;
mod helper;

//given a point, computes bfs distances from selected node to every other, returns (list of distances, average distance)
fn breadth_first_search(node: usize, node_count:usize, adjacency_list: &Vec<Vec<usize>>) ->  (Vec<usize>, f64){

    let mut visited: Vec<(bool, usize, usize)> = vec![(false, 0,0); node_count]; // (visited, previous node, total distance)
    visited[node] = (true, node, 0);
    let mut queue: Vec<usize> = vec![node];

    let mut distances: Vec<usize> = vec![0; node_count];
    let mut distance_sum: usize = 0;
    let mut visited_count = 1;

    while !queue.is_empty() {
        let current_node = queue[0];
        
        for neighbor in &adjacency_list[current_node] {
            if !visited[*neighbor].0 { //if neighbor not visited
                queue.push(*neighbor);
                visited[*neighbor] = (true, current_node, visited[current_node].2 +1);

                distances[*neighbor] = visited[current_node].2 +1;
                distance_sum += distances[*neighbor];
                visited_count +=1;
            } 
        }
        queue.remove(0);

    }
    let average_distance: f64 = distance_sum as f64 / (visited_count -1) as f64; //node_count-1 because should not include distance to self
    return (distances, average_distance);
}

fn main() {
    
    let node_count = 174956;
    let edges = helper::extract_edges(format!("sf_edges.txt"));
    let list = helper::to_adjacency_list(node_count, edges);
    //let edges = vec![(1,2), (1,3), (3,4)];
    //let list = helper::to_adjacency_list(5, edges);

    //println!("{:?}\n", list);

    let bfs = breadth_first_search(1, list.len(), &list);
    //println!("{:?}", bfs);

    //generate random sample of 5000 points 
    //let random_sample: [i32] = [;5000];
    //let x: i32 = rand::thread_rng().gen_range(0..=174956);

    let mut count =0;
    /*
    for node in 0..174956 as usize {
        if rand::thread_rng().gen_range(0..=1000) < 5 {
            println!("{}: {}, count:{}", node, breadth_first_search(node, 174956, &list).1, count);
            count+=1;
        }
        
    }
     */

    let mut sum: f64 = 0.00;
    for _ in 0..5000 {
        let random_node = rand::thread_rng().gen_range(0..node_count);
        sum += breadth_first_search(random_node, node_count, &list).1;
    }
    println!("{}", sum / 5000.0);
    
}