use std::collections::{HashSet, HashMap};

type Lane = u32;
type Elements = HashSet<Lane>;
type Order = Vec<u32>;
type Orders = HashSet<Order>;
type Weight = u32;
type Graph = HashMap<(Lane, Lane), Weight>;
type UsedSet = HashSet<Lane>;
type Rank = u32;

fn main() {
    let orders : Orders = vec![vec![1,2,3,4],
                               vec![2,1,3,4]].into_iter().collect();
    let graph = orders_to_graph(orders.clone());
    let elements = orders_to_elements(orders);
    
    println!("Graph: {:?}", graph);
    println!("Elements: {:?}", elements);

    let mut final_paths = HashSet::new();
    let mut final_rank = 0;
    for e in elements {
        let mut used = HashSet::new();
        used.insert(e);
        let (paths, rank) = try_path(e, &graph, &mut used);
        if rank == final_rank {
            for path in paths {
                final_paths.insert(path);
            }
        } else if rank > final_rank {
            final_rank = rank;
            final_paths = paths;
        }
    }

    println!("Paths: {:?} with Rank: {:?}", final_paths, final_rank);
}

fn try_path(start : u32, graph : &Graph, used : &mut UsedSet) -> (Orders, Rank) {
    let mut orders = HashSet::new();
    let mut value = 0;

    for (edge, weight) in graph {
        let beginning = edge.0;
        let end = edge.1;

        if start == beginning && !used.contains(&end) {
            used.insert(end);
            let (mut paths, rank) = try_path(end, graph, used);
            if rank + weight == value {
                for mut path in paths {
                    path.push(start);
                    orders.insert(path);
                }
                
            } else if rank + weight > value {
                orders = HashSet::new();
                value = rank + weight;
                for mut path in paths {
                    path.push(start);
                    orders.insert(path);
                }   
            }
            used.remove(&end);
        }
        
    }


    if value == 0 {
        orders = HashSet::new();
        orders.insert(vec![start]);
    }
    return (orders, value);
    
}

fn orders_to_elements(orders : Orders) -> Elements {
    let mut elements = HashSet::new();
    for order in orders {
        for lane in order{
            elements.insert(lane);
        }
    }
    return elements;
}

fn orders_to_graph(orders : Orders) -> Graph {
    let mut graph = HashMap::new();
    for order in orders {
        for i in 0..order.len() {
            for j in i+1..order.len(){
                let key = (order[i],order[j]);
                let count = graph.get(&key).unwrap_or(&0).clone();
                graph.insert(key, count+1);
            }
        }
    }

    return graph;
}
