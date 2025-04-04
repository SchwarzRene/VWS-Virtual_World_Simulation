pub mod map;
mod display;

#[macroquad::main("Water and Plant Visualization")]
async fn main() {
    let max_width = 100;
    let max_height = 50;
    
    let mut water_manager = map::water::WaterManager::new();
    water_manager.generate_water(8, max_width, max_height, 14);

    // Place the initial plant near the center of the water area
    let p1 = map::plant::Plant{
        position : vec![ max_width / 2, max_height / 2 ],  // Center of the area
        growth : 1.0,
        num_grown : 0
    };

    let mut plant_manager = map::plant::PlantManager::new(max_width, max_height);
    plant_manager.add_plant(p1);

    // Display the water and plant visualization
    display::display_water_and_plants(&water_manager, &mut plant_manager).await;
}
