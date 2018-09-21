use std::collections::HashMap;

pub trait Show {
    fn show(&self) -> String;
}

impl<A: Show> Show for HashMap<String, A> {
    fn show(&self) -> String {
        let mut s = String::new();
        for (k, v) in  self.iter() {
            let s_s = format!("{} : {}\n", k, v.show());
            s.push_str(&s_s);
        }
        return s;
    }
}
