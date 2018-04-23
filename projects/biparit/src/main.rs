use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {

    let nodes :HashSet<_>       = vec!['a', 'b','c','d','e','f'].into_iter().collect();
    let edges :HashSet<_> = vec![('a','d'),('b','d'),('b','e'),('b','f'),('c','e')].into_iter().collect();


    match find_biparit(&nodes, &edges) {
        Some((w,v)) => {
            println!("{:?}", w);
            println!("{:?}", v);
        },
        None => {
            println!("No sets found");
        },
    }

    println!("Input: {:?}, {:?}", nodes, edges);
}

fn find_biparit<I, J, A: Eq + Copy + Hash>(nodes: &I, edges: &J) -> Option<(HashSet<A>, HashSet<A>)>
    where
    I: IntoIterator<Item = A> + Clone,
    J: IntoIterator<Item = (A,A)> + Clone,
{
    use std::collections::VecDeque;
    let head;
    let nodes = nodes.clone().into_iter().collect::<Vec<_>>();
    let edges = edges.clone().into_iter().collect::<Vec<_>>();
    let empty                            = HashSet::new();
    let mut w                            = HashSet::new();
    let mut u                            = HashSet::new();
    let mut checked                      = HashSet::new();
    let mut queue                        = VecDeque::new();
    let mut map                          = create_map(&edges);
    if nodes.is_empty() { return None; }else{ head = nodes[0]; } 
    queue.push_back((0 as usize,head));

    {
        let mut sets = vec![&mut w, &mut u];
        while let Some((in_var, node)) = queue.pop_front() {
            let all = map.get(&node).unwrap_or(&empty);
            let out_var = (in_var + 1) % 2;
            if !checked.contains(&node) {
                sets[in_var].insert(node);
                checked.insert(node);
                all.iter().for_each(|adj| queue.push_back((out_var,*adj)));
            } 
        }
    }
    if !check(&w, &u, &mut map){
        return None;
    }
    
    Some((w, u))
}


fn create_map<I, A: Eq + Copy + Hash>(edges: &I) -> HashMap<A, HashSet<A>>
    where
    I: IntoIterator<Item = (A,A)> + Clone,
{
    let edges = edges.clone();
    let mut map : HashMap<A, HashSet<A>> = HashMap::new();
    for (a,b) in edges {
        if let Some(mut v) = map.remove(&a) {
            v.insert(b);
            map.insert(a, v);
        } else {
            let mut v: HashSet<A> = HashSet::new();
            v.insert(b);
            map.insert(a, v);
        }
        if let Some(mut v) = map.remove(&b) {
            v.insert(a);
            map.insert(b, v);
        } else {
            let mut v: HashSet<A> = HashSet::new();
            v.insert(a);
            map.insert(b, v);
        }
       
            
    }

    map

}

fn check<A: Eq + Copy + Hash>(w: &HashSet<A>, u: &HashSet<A>, map: &mut HashMap<A, HashSet<A>>) -> bool{
    for &n in w {
        if let Some(all) = map.remove(&n) {
            for m in all{
                if w.contains(&m) {
                    return false;
                }
            }
        }
    }

    for &n in u {
        if let Some(all) = map.remove(&n) {
            for m in all{
                if u.contains(&m){
                    return false;
                }
            }
        }
    }

    return true;
}

