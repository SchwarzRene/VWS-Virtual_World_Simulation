use rand::Rng;

pub struct Water {
    pub position: Vec<Vec<i32>>
}

impl Water {
    pub fn new( center : Vec<Vec<i32>> ) -> Self {
        Water {
            position: center
        }
    }

    pub fn generate_water(&mut self, max_size : i32, max_width: i32, max_height: i32) {
        // Clone the position to avoid borrow checker issues
        let start_pos = self.position[0].clone();
        self.add_water(&start_pos, 0, max_size, max_width, max_height);

        self.position.sort_unstable();
        self.position.dedup();
    }

    fn add_water(&mut self, pos: &Vec<i32>, mut idx: i32, max_size : i32, max_width: i32, max_height: i32) {
        if idx < max_size {
            let mut rng = rand::thread_rng();
            let direction = rng.gen_range(0..4);

            let mut new_pos = pos.clone();
            match direction {
                0 => { if new_pos[0] > 0 { new_pos[0] -= 1; } },
                1 => { if new_pos[0] < max_width - 1 { new_pos[0] += 1; } },
                2 => { if new_pos[1] > 0 { new_pos[1] -= 1; } },
                3 => { if new_pos[1] < max_height - 1 { new_pos[1] += 1; } },
                _ => (),
            }
            
            // Only add the position if it's within bounds
            if new_pos[0] >= 0 && new_pos[0] < max_width && 
               new_pos[1] >= 0 && new_pos[1] < max_height {
                // Add the new position
                self.position.push(new_pos.clone());
                
                // Increment the index for the next recursive calls
                idx = idx + 1;
                
                // Get the new position for recursive calls
                let new_pos_for_recursion = self.position.last().unwrap().clone();
                
                // Make recursive calls with the new position
                let num = rng.gen_range(2..4);
                let end = rng.gen_range(1..num);
                for _ in 0..end {
                    self.add_water(&new_pos_for_recursion, idx, max_size, max_width, max_height);
                }
            }
        }
    }
}

pub struct WaterManager{
    pub water : Vec<Vec<i32>>
}

impl WaterManager{
    pub fn new() -> Self {
        WaterManager{
            water : Vec::new()
        }
    }

    pub fn generate_water(&mut self, num_waters : i32, max_width : i32, max_height : i32, max_water_size : i32) {
        let mut all_water = Vec::new();

        for _ in 0..num_waters {
            let mut rnd = rand::thread_rng();
            let x_pos = rnd.gen_range(0..max_width);
            let y_pos = rnd.gen_range(0..max_height);

            let center = vec![vec![ x_pos, y_pos ] ];
            let mut new_water = Water::new( center.clone() );
            new_water.generate_water(rnd.gen_range(1..max_water_size), max_width, max_height);

            for i in 0..new_water.position.len() {
                if !all_water.contains( &new_water.position[ i ] ) {
                    all_water.push( new_water.position[ i ].clone() );
                }
            }
        }

        self.water = all_water;
    }
}