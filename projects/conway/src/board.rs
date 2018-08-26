#[derive(Clone, Debug)]
pub struct Board {
    lines: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(v: Vec<Vec<bool>>) -> Board {
        Board {
            lines: v,
        }
    }
    fn field(&self,
                 x_position: usize,
                 y_position: usize) -> u32 {
        let mut sum = 0;
        self.lines.get(y_position).map(|line| {
            if x_position > 0 {
                line.get(x_position - 1).map(|a| if *a {sum += 1;});
            }
            line.get(x_position + 1).map(|a| if *a {sum += 1;});
        });
        if y_position > 0 {
            self.lines.get(y_position - 1).map(|line| {
                if x_position > 0 {
                    line.get(x_position - 1).map(|a| if *a {sum += 1;});
                }
                line.get(x_position).map(|a| if *a {sum += 1;});
                line.get(x_position + 1).map(|a| if *a {sum += 1;});
            });
        }
        self.lines.get(y_position + 1).map(|line| {
            if x_position > 0 {
                line.get(x_position - 1).map(|a| if *a {sum += 1;});
            }
            line.get(x_position).map(|a| if *a {sum += 1;});
            line.get(x_position  + 1).map(|a| if *a {sum += 1;});
        });
        sum
    }
    pub fn step(&self) -> Board {
        let mut lines = Vec::new();
        for (y, line) in self.lines.iter().enumerate() {
            let mut new_line = Vec::new();
            for (x, v) in line.iter().enumerate() {
                let new_v;
                let around = self.field(x, y);
                new_v = around == 3 || (*v && around == 2);
                new_line.push(new_v);
            }
            lines.push(new_line);
        }
        Board {
            lines: lines,
        }
    }
    
    pub fn show(&self) -> String {
        let mut string = String::new();
        for line in &self.lines {
            string.push_str("| ");
            for v in line {
                if *v {
                    string.push_str(&format!("X | "));
                } else {
                    string.push_str(&format!("O | "));
                }
            }
            string.push_str("\n");
        }
        string
    }
}
