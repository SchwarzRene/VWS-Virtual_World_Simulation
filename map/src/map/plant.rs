use rand::Rng;
use crate::map::plant_tracker::PlantTracker;

#[derive(Clone)]
pub struct Plant{
    pub position : Vec<i32>,
    pub growth : f64,
    pub num_grown : i32
}

pub struct PlantManager {
    pub plants : Vec<Plant>,
    pub max_width: usize,
    pub max_height: usize,
    setup : bool,
    num_plants : usize,
    tracker: Option<PlantTracker>
}

impl PlantManager{
    pub fn add_plant( &mut self, plant : Plant ){
        // Only add the plant if it's within bounds
        if plant.position[0] >= 0 && plant.position[0] < ( self.max_width as i32 ) &&
           plant.position[1] >= 0 && plant.position[1] < ( self.max_height as i32 ) {
            self.plants.push( plant )
        }
    }

    pub fn new( max_width: usize, max_height: usize, num_plants : usize ) -> Self{
        PlantManager{
            plants : vec![],
            max_width,
            max_height,
            setup : false,
            num_plants : num_plants,
            tracker: None
        }
    }

    pub fn enable_tracking(&mut self, file_path: &str) {
        self.tracker = Some(PlantTracker::new(file_path));
    }

    fn check_new_position( &self, new_position : &Vec<i32>, map_2d : &Vec<Vec<crate::map::Elements>> ) -> bool {
        if new_position[0] >= 0 && ( new_position[0] as usize ) < self.max_width &&
           new_position[1] >= 0 && ( new_position[1] as usize ) < self.max_height {
            if map_2d[ new_position[0] as usize ][ new_position[1] as usize ] == crate::map::Elements::Empty {
                return true;
            }

            return false;
        }

        false
    }

    pub fn update( &mut self, map_2d : &Vec<Vec<crate::map::Elements>> ){
        // Record plant count before update if tracking is enabled
        if let Some(tracker) = &mut self.tracker {
            let _ = tracker.record_count(self.plants.len());
        }

        if !self.setup {
            for _ in 0..self.num_plants {
                let mut max_attempts = 10;
                loop {
                    let mut rng = rand::rng();
                    let x = rng.random_range(0..self.max_width) as i32;
                    let y = rng.random_range(0..self.max_height) as i32;
                    if self.check_new_position( &vec![x, y], map_2d ){
                        self.plants.push(Plant { position: vec![x, y], growth: 0.2, num_grown: 0 });
                        break;
                    }
                    max_attempts -= 1;
                    if max_attempts == 0 {
                        break;
                    }
                }
            }
            self.setup = true;
        }
        let mut rng = rand::rng();
        
        // Create a vector to store new plants to add
        let mut new_plants = Vec::new();
        
        // Iterate through indices to modify plants in place
        for i in 0..self.plants.len() {
            let current_growth = rng.random_range(1.0..1.2);
            self.plants[i].growth = self.plants[i].growth * current_growth;

            if self.plants[i].growth > 1.5 && self.plants[i].num_grown < 3 {
                let mut new_position = self.plants[i].position.clone();
                let direction_x = rng.random_range(-3..=3);
                let direction_y = rng.random_range(-3..=3);

                if direction_x != 0 || direction_y != 0 {
                    new_position[0] = new_position[0] + direction_x;
                    new_position[1] = new_position[1] + direction_y;
                    
                    // Only create new plant if position is within bounds
                    if self.check_new_position( &new_position, map_2d ) {
                        new_plants.push(Plant {
                            position: new_position.clone(),
                            growth: 0.2,
                            num_grown : 0
                        });
                        self.plants[i].growth = 0.2;    
                    }
                }
                self.plants[ i ].num_grown = self.plants[ i ].num_grown + 1; 
            } else if self.plants[i].growth > 1.5 {
                self.plants[i].growth = 1.5;
            }
        }
        
        // Add all new plants to the plants vector
        for plant in new_plants {
            self.plants.push(plant);
        }
    }

    pub fn get_plant_map( &self ) -> Vec<Vec<(crate::map::Elements, f64)>> {
        let mut plant_map = vec![vec![(crate::map::Elements::Empty, 0.0); self.max_height]; self.max_width];
        
        for plant in &self.plants {
            if plant.position[0] >= 0 && (plant.position[0] as usize) < self.max_width &&
               plant.position[1] >= 0 && (plant.position[1] as usize) < self.max_height {
                plant_map[plant.position[0] as usize][plant.position[1] as usize] = (crate::map::Elements::Plant, plant.growth);
            }
        }
        
        plant_map
    }
}