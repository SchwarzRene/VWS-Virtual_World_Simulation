use rand::Rng;

pub struct Water {
    pub position: Vec<Vec<usize>>,
    direction : i32
}

impl Water {
    pub fn new( center : Vec<Vec<usize>> ) -> Self {
        let mut rng = rand::rng();
        let rand_direction = rng.random_range(0..=3);
        Water {
            position: center, 
            direction : rand_direction
        }
    }

    pub fn generate_water(&mut self, max_size : i32, max_width: usize, max_height: usize) {
        // Clone the position to avoid borrow checker issues
        let start_pos = self.position[0].clone();
        self.add_water(&start_pos, 0, max_size, max_width, max_height);

        self.position.sort_unstable();
        self.position.dedup();
    }

    fn add_water(&mut self, pos: &Vec<usize>, mut idx: i32, max_size : i32, max_width: usize, max_height: usize ) {
        if idx < max_size {
            let mut rng = rand::rng();
            
            // Generate a valid direction that will move within bounds
            let mut valid_direction = false;
            let mut new_pos = pos.clone();
            let mut attempts = 0;
            const MAX_ATTEMPTS: i32 = 5; // Safety limit to prevent infinite loops
            
            while !valid_direction && attempts < MAX_ATTEMPTS {
                let direction = rng.random_range(0..=7);
                attempts += 1;
                
                match direction {
                    0 => { 
                        if new_pos[0] > 0 { 
                            new_pos[0] -= 1; 
                            valid_direction = true; 
                        }
                    },
                    1 => { 
                        if new_pos[0] < max_width - 1 { 
                            new_pos[0] += 1; 
                            valid_direction = true; 
                        }
                    },
                    2 => { 
                        if new_pos[1] > 0 { 
                            new_pos[1] -= 1; 
                            valid_direction = true; 
                        }
                    },
                    3 => { 
                        if new_pos[1] < max_height - 1 { 
                            new_pos[1] += 1; 
                            valid_direction = true; 
                        }
                    },
                    4 => { 
                        // Try a random direction from 0-3
                        let rand_dir = rng.random_range(0..4);
                        match rand_dir {
                            0 => { if new_pos[0] > 0 { new_pos[0] -= 1; valid_direction = true; } },
                            1 => { if new_pos[0] < max_width - 1 { new_pos[0] += 1; valid_direction = true; } },
                            2 => { if new_pos[1] > 0 { new_pos[1] -= 1; valid_direction = true; } },
                            3 => { if new_pos[1] < max_height - 1 { new_pos[1] += 1; valid_direction = true; } },
                            _ => {}
                        }
                    },
                    5 | 6 | 7 => { 
                        // Use the stored direction
                        match self.direction {
                            0 => { if new_pos[0] > 0 { new_pos[0] -= 1; valid_direction = true; } },
                            1 => { if new_pos[0] < max_width - 1 { new_pos[0] += 1; valid_direction = true; } },
                            2 => { if new_pos[1] > 0 { new_pos[1] -= 1; valid_direction = true; } },
                            3 => { if new_pos[1] < max_height - 1 { new_pos[1] += 1; valid_direction = true; } },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            
            // If we couldn't find a valid direction after multiple attempts, just use the current position
            if !valid_direction {
                new_pos = pos.clone();
            }
            
            // Only add the position if it's within bounds
            if new_pos[0] < max_width && new_pos[1] < max_height {
                // Add the new position
                self.position.push(new_pos.clone());
                
                // Increment the index for the next recursive calls
                idx = idx + 1;
                
                // Get the new position for recursive calls
                let new_pos_for_recursion = self.position.last().unwrap().clone();
                
                // Make recursive calls with the new position
                let num = rng.random_range(2..5);
                let end = rng.random_range(1..num);
                for _ in 0..end {
                    self.add_water(&new_pos_for_recursion, idx, max_size, max_width, max_height);
                }
            }
        }
    }
}

pub struct WaterManager{
    pub water : Vec<Vec<crate::map::Elements>>
}

impl WaterManager{
    pub fn new() -> Self {
        WaterManager{
            water : Vec::new()
        }
    }

    pub fn generate_water(&mut self, num_waters : i32, max_width : usize, max_height : usize, max_water_size : i32) {
        let mut all_water = vec![vec![crate::map::Elements::Empty; max_height as usize ]; max_width as usize ];

        for _ in 0..num_waters {
            let mut rnd = rand::rng();
            let x_pos = rnd.random_range(0..max_width);
            let y_pos = rnd.random_range(0..max_height);

            let center = vec![vec![ x_pos, y_pos ] ];
            let mut new_water = Water::new( center.clone() );
            new_water.generate_water( max_water_size, max_width, max_height);

            for i in 0..new_water.position.len() {
                if all_water[ new_water.position[ i ][ 0 ] as usize ][ new_water.position[ i ][ 1 ] as usize ] == crate::map::Elements::Empty {
                    all_water[ new_water.position[ i ][ 0 ] as usize ][ new_water.position[ i ][ 1 ] as usize ] = crate::map::Elements::Water;
                }
            }
        }

        self.water = all_water;
    }

    pub fn get_water_map(&self) -> Vec<Vec<crate::map::Elements>> {
        self.water.clone()
    }
}