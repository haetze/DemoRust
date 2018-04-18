


fn main() {

    let nodes :Vec<i32>       = (0..6).collect();
    let edges :Vec<(i32,i32)> = vec![(0,1),(0,3),(1,3),(1,4),(1,5),(2,4)];

    let (w,v) = find_biparit(&nodes, &edges).unwrap();    
        
    println!("{:?}", w);
    println!("{:?}", v);
}


fn find_biparit(nodes: &[i32], edges: &[(i32,i32)]) -> Option<(Vec<i32>, Vec<i32>)>{

    let mut w :Vec<i32> = Vec::new();
    let mut v :Vec<i32> = Vec::new();
    
    for &node in nodes {
        if find(node, &w) && find(node, &v) {
            println!("Konnte keine Teilung finden");
            return None::<(Vec<i32>, Vec<i32>)>;
        }else if find(node, &w) {
            for &(a, b) in edges {
                if node == a {
                    push_once(&mut v,b);
                }else if node == b {
                    push_once(&mut v,a);
                }
            }
        }else if find(node, &v) {
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
                if node == a && find(b, &w) {
                    choice = 2;
                    break;
                }
                if node == a && find(b, &v) {
                    choice = 1;
                    break;
                }
                if node == b && find(a, &w) {
                    choice = 2;
                    break;
                }
                if node == b && find(a, &v) {
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

fn push_once(vec: &mut Vec<i32>, i: i32){
    if ! find(i, &vec){
        vec.push(i);
    }
}


fn find(i: i32, vec: &Vec<i32>) -> bool {
    for n in vec {
        if *n == i {
            return true;
        }
    }
    return false;
}
