use rand::Rng;

#[derive(Clone)]
pub struct Plant{
    pub position : Vec<i32>,
    pub growth : f64,
    pub num_grown : i32
}

pub struct PlantManager {
    pub plants : Vec<Plant>,
    pub max_width: i32,
    pub max_height: i32
}

impl PlantManager{
    pub fn add_plant( &mut self, plant : Plant ){
        // Only add the plant if it's within bounds
        if plant.position[0] >= 0 && plant.position[0] < self.max_width &&
           plant.position[1] >= 0 && plant.position[1] < self.max_height {
            self.plants.push( plant )
        }
    }

    pub fn new( max_width: i32, max_height: i32 ) -> Self{
        PlantManager{
            plants : vec![],
            max_width,
            max_height
        }
    }

    pub fn update( &mut self ){
        let mut rng = rand::thread_rng();
        
        // Create a vector to store new plants to add
        let mut new_plants = Vec::new();
        
        // Iterate through indices to modify plants in place
        for i in 0..self.plants.len() {
            let current_growth = rng.gen_range(1.0..1.2);
            self.plants[i].growth = self.plants[i].growth * current_growth;

            if self.plants[i].growth > 1.5 && self.plants[i].num_grown < 3 {
                let mut new_position = self.plants[i].position.clone();
                let direction_x = rng.gen_range(-3..=3);
                let direction_y = rng.gen_range(-3..=3);

                if direction_x != 0 || direction_y != 0 {
                    new_position[0] = new_position[0] + direction_x;
                    new_position[1] = new_position[1] + direction_y;
                    
                    // Only create new plant if position is within bounds
                    if new_position[0] >= 0 && new_position[0] < self.max_width &&
                       new_position[1] >= 0 && new_position[1] < self.max_height {
                        new_plants.push(Plant {
                            position: new_position.clone(),
                            growth: 0.2,
                            num_grown : 0
                        });
                        self.plants[ i ].num_grown = self.plants[ i ].num_grown + 1;
                    }
                }

                self.plants[i].growth = 0.2;
            }
        }
        
        // Add all new plants at once
        for plant in new_plants {
            self.add_plant(plant);
        }
    }
}