
fn main() {

    let nodes :Vec<i32>       = (0..6).collect();
    let edges :Vec<(i32,i32)> = vec![(1,0),(0,3),(1,3),(1,4),(1,5),(2,4)];

    let (w,v) = find_biparit(&nodes, &edges).unwrap();    
        
    println!("{:?}", w);
    println!("{:?}", v);
}


fn find_biparit<A: Eq + Copy>(nodes: &[A], edges: &[(A,A)]) -> Option<(Vec<A>, Vec<A>)>{
    use std::collections::VecDeque;
    
    let mut queue: VecDeque<(i32,A)> = VecDeque::new();
    let head;
    let mut w: Vec<A> = Vec::new();
    let mut u: Vec<A> = Vec::new();
    let mut checked: Vec<A> = Vec::new();
    match nodes.first() {
        None => {
            return None::<(Vec<A>, Vec<A>)>
        },
        Some(node) => {
            head = *node;
        },
    };

    queue.push_back((0,head));
    
    while let Some((in_var, node)) = queue.pop_front() {
        if in_var ==  0 && !find(&node, &checked) {
            push_once(&mut w, node);
            push_once(&mut checked, node);
            for adjecent in find_all(node, &edges) {
                queue.push_back((1,adjecent));
            }
        } else if in_var ==  1 && !find(&node, &checked){
            push_once(&mut u, node);
            push_once(&mut checked, node);
            for adjecent in find_all(node, &edges) {
                queue.push_back((0,adjecent));
            }
        }
        
    }

    for n in &w {
        if find(n, &u){
            return None::<(Vec<A>, Vec<A>)>;
        }
    }


    
            
    
    Some((w, u))

    
    
}

fn find_all<A: Eq + Copy>(i: A, vec: &[(A,A)]) -> Vec<A> {
    let mut v = Vec::new();

    for &(a,b) in vec {
        if a == i {
            push_once(&mut v, b);
        }
        if b == i {
            push_once(&mut v, a);
        }
    }

    v

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
