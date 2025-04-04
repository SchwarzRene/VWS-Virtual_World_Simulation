pub mod map;
mod display;

#[macroquad::main("Water and Plant Visualization")]
async fn main() {
    let max_width : usize = 130;
    let max_height : usize = 80;
    
    let mut water_manager = map::water::WaterManager::new();
    water_manager.generate_water(10, max_width, max_height, 25 );

    let mut map = map::Map::new( water_manager, max_width, max_height, 10 );
    
    // Enable plant tracking to record plant counts over time
    map.enable_plant_tracking("plant_history.json");

    // Display the water and plant visualization
    display::display_water_and_plants(&mut map, max_width, max_height).await;
}
