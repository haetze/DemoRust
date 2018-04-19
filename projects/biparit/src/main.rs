


fn main() {

    let nodes :Vec<i32>       = (0..6).collect();
    let edges :Vec<(i32,i32)> = vec![(0,3),(1,3),(1,4),(1,5),(2,4)];

    let (w,v) = find_biparit(&nodes, &edges).unwrap();    
        
    println!("{:?}", w);
    println!("{:?}", v);
}


fn find_biparit<A: Eq + Copy>(nodes: &[A], edges: &[(A,A)]) -> Option<(Vec<A>, Vec<A>)>{

    let mut w :Vec<A> = Vec::new();
    let mut v :Vec<A> = Vec::new();
    
    for &node in nodes {
        if find(&node, &w) && find(&node, &v) {
            println!("Konnte keine Teilung finden");
            return None::<(Vec<A>, Vec<A>)>;
        }else if find(&node, &w) {
            for &(a, b) in edges {
                if node == a {
                    push_once(&mut v,b);
                }else if node == b {
                    push_once(&mut v,a);
                }
            }
        }else if find(&node, &v) {
            for &(a, b) in edges{
                if node == a {
                    push_once(&mut w,b);
                }else if node == b {
                    push_once(&mut w,a);
                }
            }

        }else{
            let mut choice = 0;
            for &(a, b) in edges{
                if node == a && find(&b, &w) {
                    choice = 2;
                    break;
                }
                if node == a && find(&b, &v) {
                    choice = 1;
                    break;
                }
                if node == b && find(&a, &w) {
                    choice = 2;
                    break;
                }
                if node == b && find(&a, &v) {
                    choice = 1;
                    break;
                }
            }
            if choice == 1 || choice == 0 {
                w.push(node);
                for &(a, b) in edges{
                    if node == a {
                        push_once(&mut v,b);
                    }else if node == b {
                        push_once(&mut v,a);
                    }
                }
            }
            if choice == 2 {
                v.push(node);
                for &(a, b) in edges{
                    if node == a {
                        push_once(&mut w,b);
                    }else if node == b {
                        push_once(&mut w,a);
                    }
                }
            }
                
        }
    }

    Some((w,v))

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
