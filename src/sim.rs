use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Simulation {
    width: usize,
    height: usize,
    field: Vec<bool>,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            width,
            height,
            field: vec![false; width * height],
        }
    }

    pub fn get_field(&self, x: usize, y: usize) -> Option<bool> {
        self.field.get(y * self.width + x).copied()
    }

    pub fn get_field_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        self.field.get_mut(y * self.width + x)
    }

    pub fn activate(&mut self, x: usize, y: usize) {
        let field = self.get_field_mut(x, y);
        if let Some(field) = field {
            *field = true;
        }
    }

    #[allow(dead_code)]
    pub fn deactivate(&mut self, x: usize, y: usize) {
        let field = self.get_field_mut(x, y);
        if let Some(field) = field {
            *field = false;
        }
    }

    pub fn enforce_rules(&mut self, x: usize, y: usize, old_field: &Simulation) {
        let ncount = old_field.get_neighbor_count(x, y);
        let field = match old_field.get_field(x, y) {
            Some(field) => field,
            _ => return,
        };
        let new_field = match self.get_field_mut(x, y) {
            Some(field) => field,
            _ => return,
        };

        if field && ncount < 2 {
            *new_field = false;
        } else if field && (ncount == 2 || ncount == 3) {
            *new_field = true;
        } else if field && ncount > 3 {
            *new_field = false;
        } else if !field && ncount == 3 {
            *new_field = true;
        }
    }

    pub fn get_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;
        for oy in y.saturating_sub(1)..=y.saturating_add(1).min(self.height) {
            for ox in x.saturating_sub(1)..=x.saturating_add(1).min(self.width) {
                if !(ox == x && oy == y) && self.get_field(ox, oy) == Some(true) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn step(&mut self) {
        let old_field = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                self.enforce_rules(x, y, &old_field);
            }
        }
    }

    pub fn get_symbol(&self, x: usize, y: usize, alive: &'static str, dead: &'static str) -> &'static str {
        match self.get_field(x, y) {
            Some(true) => alive,
            Some(false) => dead,
            _ => dead,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_symbol(x, y, "##", "  "));
            }
            println!();
        }
    }
}
