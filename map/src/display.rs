use macroquad::prelude::*;
use crate::map::water::WaterManager;
use crate::map::plant::{Plant, PlantManager};
use std::time::{Duration, Instant};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const CELL_SIZE: f32 = 20.0;
const WATER_COLOR: Color = Color::new(0.0, 0.5, 1.0, 1.0);
const PLANT_COLOR: Color = Color::new(0.0, 0.8, 0.0, 1.0);
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

pub async fn display_water_and_plants(water_manager: &WaterManager, plant_manager: &mut PlantManager) {
    // Calculate the offset to center the visualization
    let min_x = if !water_manager.water.is_empty() {
        water_manager.water.iter().map(|pos| pos[0]).min().unwrap_or(0) as f32
    } else {
        0.0
    };
    let max_x = if !water_manager.water.is_empty() {
        water_manager.water.iter().map(|pos| pos[0]).max().unwrap_or(0) as f32
    } else {
        0.0
    };
    let min_y = if !water_manager.water.is_empty() {
        water_manager.water.iter().map(|pos| pos[1]).min().unwrap_or(0) as f32
    } else {
        0.0
    };
    let max_y = if !water_manager.water.is_empty() {
        water_manager.water.iter().map(|pos| pos[1]).max().unwrap_or(0) as f32
    } else {
        0.0
    };

    // Also consider plant positions for the visualization bounds
    for plant in &plant_manager.plants {
        let x = plant.position[0] as f32;
        let y = plant.position[1] as f32;
        let min_x = min_x.min(x);
        let max_x = max_x.max(x);
        let min_y = min_y.min(y);
        let max_y = max_y.max(y);
    }

    let width = (max_x - min_x + 1.0) * CELL_SIZE;
    let height = (max_y - min_y + 1.0) * CELL_SIZE;

    let offset_x = (SCREEN_WIDTH - width) / 2.0;
    let offset_y = (SCREEN_HEIGHT - height) / 2.0;

    let mut last_update = Instant::now();

    loop {
        clear_background(WHITE);

        // Draw grid lines only within the visualization bounds
        for x in 0..=(width / CELL_SIZE) as i32 {
            let x_pos = x as f32 * CELL_SIZE + offset_x;
            draw_line(
                x_pos, offset_y,
                x_pos, offset_y + height,
                1.0, Color::new(0.8, 0.8, 0.8, 1.0)
            );
        }
        for y in 0..=(height / CELL_SIZE) as i32 {
            let y_pos = y as f32 * CELL_SIZE + offset_y;
            draw_line(
                offset_x, y_pos,
                offset_x + width, y_pos,
                1.0, Color::new(0.8, 0.8, 0.8, 1.0)
            );
        }

        // Draw each water position
        for pos in &water_manager.water {
            let x = (pos[0] as f32 - min_x) * CELL_SIZE + offset_x;
            let y = (pos[1] as f32 - min_y) * CELL_SIZE + offset_y;
            
            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, WATER_COLOR);
        }

        // Draw each plant
        for plant in &plant_manager.plants {
            let x = (plant.position[0] as f32 - min_x) * CELL_SIZE + offset_x;
            let y = (plant.position[1] as f32 - min_y) * CELL_SIZE + offset_y;
            
            // Size of plant based on growth
            let size = CELL_SIZE * (0.5 + plant.growth as f32 * 0.25);
            let offset = (CELL_SIZE - size) / 2.0;
            
            draw_rectangle(
                x + offset, 
                y + offset, 
                size, 
                size, 
                PLANT_COLOR
            );
        }

        // Update plants every second
        if last_update.elapsed() >= UPDATE_INTERVAL {
            plant_manager.update();
            last_update = Instant::now();
        }

        // Check for exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
} 