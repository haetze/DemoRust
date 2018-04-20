use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {

    let nodes :Vec<i32>       = (0..6).collect();
    let edges :Vec<(i32,i32)> = vec![(0,3),(1,3),(1,4),(1,5),(2,4)];

    match find_biparit(&nodes, &edges) {
        Some((w,v)) => {
            println!("{:?}", w);
            println!("{:?}", v);
        },
        None => {
            println!("No sets found");
        },
    }
}

fn find_biparit<A: Eq + Copy + Hash>(nodes: &[A], edges: &[(A,A)]) -> Option<(HashSet<A>, HashSet<A>)>{
    use std::collections::VecDeque;
    
    let mut queue: VecDeque<(i32,A)> = VecDeque::new();
    let head;
    let mut w: HashSet<A> = HashSet::new();
    let mut u: HashSet<A> = HashSet::new();
    let mut checked: HashSet<A> = HashSet::new();
    let mut map = create_map(edges);
    match nodes.first() {
        None => {
            return None;
        },
        Some(node) => {
            head = *node;
        },
    };

    queue.push_back((0,head));
    
    while let Some((in_var, node)) = queue.pop_front() {
        let all = match map.remove(&node) {
            None => Vec::new(),
            Some(v) => v,
        };    
        if in_var ==  0 && !checked.contains(&node) {
            w.insert(node);
            checked.insert(node);
            for &adjecent in &all {
                queue.push_back((1,adjecent));
            }
        } else if in_var ==  1 && !checked.contains(&node){
            u.insert(node);
            checked.insert(node);
            for &adjecent in &all {
                queue.push_back((0,adjecent));
            }
        } 
        map.insert(node, all);
      
        
    }

    if ! check(&w, &u, &mut map){
        return None;
    }
        
    Some((w, u))

    
    
}


fn create_map<A: Eq + Copy + Hash>(edges: &[(A,A)]) -> HashMap<A, Vec<A>> {
    let mut map = HashMap::new();
    for &(a,b) in edges {
        match map.remove(&a) {
            None => {
                let mut v: Vec<A> = Vec::new();
                push_once(&mut v, b);
                map.insert(a, v);
            },
            Some(mut v) => {
                push_once(&mut v, b);
                map.insert(a, v);
            }
        }
        match map.remove(&b) {
            None => {
                let mut v: Vec<A> = Vec::new();
                push_once(&mut v, a);
                map.insert(b, v);
            },
            Some(mut v) => {
                push_once(&mut v, a);
                map.insert(b, v);
            }
        }
            
    }

    map

}

fn check<A: Eq + Copy + Hash>(w: &HashSet<A>, u: &HashSet<A>, map: &mut HashMap<A, Vec<A>>) -> bool{
    for &n in w {
        let all = match map.remove(&n) {
            None => Vec::new(),
            Some(v) => v,
        };
        for m in all{
            if w.contains(&m) {
                return false;
            }
        }
    }

    for &n in u {
        let all = match map.remove(&n) {
            None => Vec::new(),
            Some(v) => v,
        };
        for m in all{
            if u.contains(&m){
                return false;
            }
        }
    }

    return true;
}

fn push_once<A:Eq + Copy>(vec: &mut Vec<A>, i: A){
    if ! find(&i, &vec){
        vec.push(i);
    }
}


fn find<A: Eq>(i: &A, vec: &[A]) -> bool {
    for n in vec {
        if n == i {
            return true;
        }
    }
    return false;
}
