
pub struct Player {
    pub name: String,
    pub ip: String,
}

pub struct Playerlist {
    pub list: Vec<Player>,
}

impl Playerlist {
    pub fn new() -> Playerlist{
        Playerlist {
            list: Vec::new(),
        }
    }
    pub fn exists(&self, name: &String) -> bool{
        for player in &self.list {
            if player.name == *name {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, name: &String, ip: &String){
        if !self.exists(name){
            let p = Player {
                name: name.clone(),
                ip: ip.clone(),
            };
            self.list.push(p);
        }
    }

    pub fn find(&mut self, name: &String) -> Option<Player>{
        let index = self.find_index(name);
        if index == -1 {
            None
        }else{
            Some(self.list.swap_remove(index as usize))
        }
    }

    pub fn find_index(&self, name: &String) -> i32{
        let mut u = 0;
        for player in &self.list {
            if player.name == *name {
                return u;
            }
            u = u + 1;
        }
        -1
    }
        
}
