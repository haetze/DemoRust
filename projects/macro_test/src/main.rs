use std::collections::HashMap;

macro_rules! map {
    ($ (($x:expr,$y:expr)),* ) => {
        {
            let mut map = HashMap::new();
            $ (map.insert($x, $y);)*
            map
        }
    };
}

fn main() {
    let map: HashMap<i32, i32> = map![(1, 2)];
    println!("{:?}", map);
}
