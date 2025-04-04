use rand::Rng;

pub struct Water {
    pub position: Vec<Vec<i32>>
}

impl Water {
    pub fn new() -> Self {
        Water {
            position: vec![vec![0, 0]] // Initialize with a starting position
        }
    }

    pub fn generate_water(&mut self, max_size : i32 ) {
        // Clone the position to avoid borrow checker issues
        let start_pos = self.position[0].clone();
        self.add_water(&start_pos, 0, max_size);
    }

    fn add_water(&mut self, pos: &Vec<i32>, mut idx: i32, max_size : i32 ) {
        if idx < max_size {
            let mut rng = rand::rng();
            let direction = rng.random_range(0..4);

            let mut new_pos = pos.clone();
            match direction {
                0 => { new_pos[0] = new_pos[0] - 1; },
                1 => { new_pos[0] = new_pos[0] + 1; },
                2 => { new_pos[1] = new_pos[1] - 1; },
                3 => { new_pos[1] = new_pos[1] + 1; },
                _ => (),
            }
            
            // Add the new position
            self.position.push(new_pos.clone());
            
            // Increment the index for the next recursive calls
            idx = idx + 1;
            
            // Get the new position for recursive calls
            let new_pos_for_recursion = self.position.last().unwrap().clone();
            
            // Make recursive calls with the new position

            for _ in ..max_size - idx {
                self.add_water(&new_pos_for_recursion, idx, max_size );
            }
        }
    }
}