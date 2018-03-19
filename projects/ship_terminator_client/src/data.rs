pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point{
    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct PartOfShip {
    pub point: Point,
    pub hit: bool,
}

impl PartOfShip {
    pub fn overlap(&self, other: &PartOfShip) -> bool{
        self.point.x == other.point.x
            && self.point.y == other.point.y
    }
    pub fn new(p: Point) -> PartOfShip{
        PartOfShip{
            point: p,
            hit: false,
        }
    }
}
        

//5,4,2
pub enum Ship{
    Big(PartOfShip, PartOfShip, PartOfShip, PartOfShip, PartOfShip),
    Medium(PartOfShip, PartOfShip, PartOfShip, PartOfShip),
    Small(PartOfShip, PartOfShip),
}

impl Ship {

    pub fn new(start: Point, direction: Direction, size: u32) -> Option<Ship>{   
        match size {
            1 => Ship::newBig(start, direction),
            2 => Ship::newMedium(start, direction),
            3 => Ship::newSmall(start, direction), 
            _ => None,
        }
    }
        
    
    pub fn newBig(start: Point, direction: Direction) -> Option<Ship>{
        match direction {
            Direction::Left => {
                if start.x - 4 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.left());
                    let q = PartOfShip::new(p.point.left());
                    let r = PartOfShip::new(q.point.left());
                    let s = PartOfShip::new(r.point.left());
                    Some(Ship::Big(PartOfShip::new(start), p, q, r, s))
                }
            },
            Direction::Right => {
                if start.x + 4 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.right());
                    let q = PartOfShip::new(p.point.right());
                    let r = PartOfShip::new(q.point.right());
                    let s = PartOfShip::new(r.point.right());
                    Some(Ship::Big(PartOfShip::new(start), p, q, r, s))
                }
            },
            Direction::Down => {
                if start.y + 4 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.down());
                    let q = PartOfShip::new(p.point.down());
                    let r = PartOfShip::new(q.point.down());
                    let s = PartOfShip::new(r.point.down());
                    Some(Ship::Big(PartOfShip::new(start), p, q, r, s))
                }
            },
            Direction::Up => {
                if start.x - 4 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.up());
                    let q = PartOfShip::new(p.point.up());
                    let r = PartOfShip::new(q.point.up());
                    let s = PartOfShip::new(r.point.up());
                    Some(Ship::Big(PartOfShip::new(start), p, q, r, s))
                }
            },
        }
    }

     pub fn newMedium(start: Point, direction: Direction) -> Option<Ship>{
        match direction {
            Direction::Left => {
                if start.x - 3 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.left());
                    let q = PartOfShip::new(p.point.left());
                    let r = PartOfShip::new(q.point.left());
                    Some(Ship::Medium(PartOfShip::new(start), p, q, r))
                }
            },
            Direction::Right => {
                if start.x + 3 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.right());
                    let q = PartOfShip::new(p.point.right());
                    let r = PartOfShip::new(q.point.right());
                    Some(Ship::Medium(PartOfShip::new(start), p, q, r))
                }
            },
            Direction::Down => {
                if start.y + 3 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.down());
                    let q = PartOfShip::new(p.point.down());
                    let r = PartOfShip::new(q.point.down());
                    Some(Ship::Medium(PartOfShip::new(start), p, q, r))
                }
            },
            Direction::Up => {
                if start.x - 3 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.up());
                    let q = PartOfShip::new(p.point.up());
                    let r = PartOfShip::new(q.point.up());
                    Some(Ship::Medium(PartOfShip::new(start), p, q, r))
                }
            },
        }
    }

    pub fn newSmall(start: Point, direction: Direction) -> Option<Ship>{
        match direction {
            Direction::Left => {
                if start.x - 1 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.left());
                    Some(Ship::Small(PartOfShip::new(start), p))
                }
            },
            Direction::Right => {
                if start.x + 1 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.right());
                    Some(Ship::Small(PartOfShip::new(start), p))
                }
            },
            Direction::Down => {
                if start.y + 1 > 9 {
                    None
                }else{
                    let p = PartOfShip::new(start.down());
                    Some(Ship::Small(PartOfShip::new(start), p))
                }
            },
            Direction::Up => {
                if start.x - 1 < 0 {
                    None
                }else{
                    let p = PartOfShip::new(start.up());
                    Some(Ship::Small(PartOfShip::new(start), p))
                }
            },
        }
    }

    pub fn overlap(&self, other: &Ship) -> bool{
        match *self {
            Ship::Big(ref p, ref q, ref r, ref s, ref t) => {
                match *other {
                    Ship::Big(ref p_, ref q_, ref r_, ref s_, ref t_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_) ||  t.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_) ||  t.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_)  ||  r.overlap(&r_) ||  s.overlap(&r_) ||  t.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_)  ||  r.overlap(&s_) ||  s.overlap(&s_) ||  t.overlap(&s_);
                        let t_check = p.overlap(&t_) ||  q.overlap(&t_)  ||  r.overlap(&t_) ||  s.overlap(&t_) ||  t.overlap(&t_);
                        p_check || q_check || r_check || s_check || t_check
                    },
                    Ship::Medium(ref p_, ref q_, ref r_, ref s_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_) ||  t.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_) ||  t.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_)  ||  r.overlap(&r_) ||  s.overlap(&r_) ||  t.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_)  ||  r.overlap(&s_) ||  s.overlap(&s_) ||  t.overlap(&s_);
                        p_check || q_check || r_check || s_check
                    },
                    Ship::Small(ref p_, ref q_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_) ||  t.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_) ||  t.overlap(&q_);
                        p_check || q_check 
                    },
                }
            },
            Ship::Medium(ref p,ref q,ref r,ref s) => {
                match *other {
                    Ship::Big(ref p_, ref q_, ref r_, ref s_, ref t_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_)  ||  r.overlap(&r_) ||  s.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_)  ||  r.overlap(&s_) ||  s.overlap(&s_);
                        let t_check = p.overlap(&t_) ||  q.overlap(&t_)  ||  r.overlap(&t_) ||  s.overlap(&t_);
                        p_check || q_check || r_check || s_check || t_check
                    },
                    Ship::Medium(ref p_, ref q_, ref r_, ref s_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_)  ||  r.overlap(&r_) ||  s.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_)  ||  r.overlap(&s_) ||  s.overlap(&s_);
                        p_check || q_check || r_check || s_check
                    },
                    Ship::Small(ref p_, ref q_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_)  ||  r.overlap(&p_) ||  s.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_)  ||  r.overlap(&q_) ||  s.overlap(&q_);
                        p_check || q_check 
                    },
                }
            },
            Ship::Small(ref p, ref q) => {
                match *other {
                    Ship::Big(ref p_, ref q_, ref r_, ref s_, ref t_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_);
                        let t_check = p.overlap(&t_) ||  q.overlap(&t_);
                        p_check || q_check || r_check || s_check || t_check
                    },
                    Ship::Medium(ref p_, ref q_, ref r_, ref s_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_);
                        let r_check = p.overlap(&r_) ||  q.overlap(&r_);
                        let s_check = p.overlap(&s_) ||  q.overlap(&s_);
                        p_check || q_check || r_check || s_check
                    },
                    Ship::Small(ref p_, ref q_) => {
                        let p_check = p.overlap(&p_) ||  q.overlap(&p_);
                        let q_check = p.overlap(&q_) ||  q.overlap(&q_);
                        p_check || q_check 
                    },
                }
            },
        }
        
    }
}
                    
                    
                    
pub struct Ships{
    pub ships: Vec<Ship>,
}

impl Ships {
    pub fn new() -> Ships{
        Ships {
            ships: Vec::new(),
        }
    }

    pub fn add(&mut self, ship: Ship) -> bool{
        for s in &self.ships {
            if s.overlap(&ship) {
                return false;
            }
        }
        self.ships.push(ship);
        true
    }
    
}

#[derive(Copy, Clone)]
pub enum Field{
    Hit(),
    Empty(),
    NotSet(),
}


impl Field{
    pub fn new() -> [[Field; 10]; 10] {
        [[Field::NotSet(); 10]; 10]
    }
}

pub struct Board{
    pub fields: [[Field; 10]; 10],
    pub ships: Ships,
}

impl Board{
    pub fn new()-> Board{
        Board{
            fields: Field::new(),
            ships: Ships::new(),
        }
    }

    pub fn add(&mut self, ship: Ship) -> bool{
        self.ships.add(ship)
    }
}
       


        
