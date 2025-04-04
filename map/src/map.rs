pub mod plant;
pub mod water;
pub mod plant_tracker;

#[derive(Clone, PartialEq)]
pub enum Elements{
    Plant,
    Water,
    Empty
}

pub struct Map{
    pub plant_manager : plant::PlantManager,
    pub water_manager : water::WaterManager,
    pub map_2d : Vec<Vec<Elements>>,
    pub plant_growth : Vec<Vec<f64>>
}

impl Map{
    pub fn new( water_manager : water::WaterManager, max_width : usize, max_height : usize, num_plants : usize ) -> Self{
        Map{
            plant_manager : crate::map::plant::PlantManager::new(max_width, max_height, num_plants),
            water_manager : water_manager,
            map_2d : vec![vec![Elements::Empty; max_height]; max_width],
            plant_growth : vec![vec![0.0; max_height]; max_width]
        }
    }

    pub fn enable_plant_tracking(&mut self, file_path: &str) {
        self.plant_manager.enable_tracking(file_path);
    }

    pub fn update( &mut self ){
        let water_map = self.water_manager.get_water_map();
        
        
        // Get the plant map with growth information
        let plant_map = self.plant_manager.get_plant_map();
        
        // Create a new map_2d with both water and plant information
        let mut new_map_2d = vec![vec![Elements::Empty; self.map_2d[0].len()]; self.map_2d.len()];
        let mut new_plant_growth = vec![vec![0.0; self.map_2d[0].len()]; self.map_2d.len()];
        
        // First, copy water information
        for x in 0..self.map_2d.len() {
            for y in 0..self.map_2d[0].len() {
                if water_map[x][y] == Elements::Water {
                    new_map_2d[x][y] = Elements::Water;
                }
            }
        }
        
        // Then, add plant information (plants take precedence over water)
        for x in 0..self.map_2d.len() {
            for y in 0..self.map_2d[0].len() {
                if plant_map[x][y].0 == Elements::Plant {
                    new_map_2d[x][y] = Elements::Plant;
                    new_plant_growth[x][y] = plant_map[x][y].1;
                }
            }
        }
        self.map_2d = new_map_2d;
        self.plant_growth = new_plant_growth;

        self.plant_manager.update( &self.map_2d );
    }
}