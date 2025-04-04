use macroquad::prelude::*;
use crate::map::{Map, Elements};
use std::time::{Duration, Instant};

const WATER_COLOR: Color = Color::new(0.0, 0.5, 1.0, 1.0);
const PLANT_COLOR_MIN: Color = Color::new(0.0, 0.6, 0.0, 1.0); // Less grown plants
const PLANT_COLOR_MAX: Color = Color::new(0.0, 0.9, 0.0, 1.0); // Fully grown plants
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

pub async fn display_water_and_plants(
    map: &mut Map,
    max_width: usize,
    max_height: usize
) {
    let mut last_update = Instant::now();

    loop {
        // Get current window dimensions
        let window_width = screen_width();
        let window_height = screen_height();
        
        // Calculate cell dimensions based on window dimensions and maximum grid size
        let cell_width = window_width / max_width as f32;
        let cell_height = window_height / max_height as f32;

        clear_background(WHITE);

        // Draw grid lines for the entire grid
        for x in 0..=max_width {
            let x_pos = x as f32 * cell_width;
            draw_line(
                x_pos, 0.0,
                x_pos, window_height,
                1.0, Color::new(0.8, 0.8, 0.8, 1.0)
            );
        }
        for y in 0..=max_height {
            let y_pos = y as f32 * cell_height;
            draw_line(
                0.0, y_pos,
                window_width, y_pos,
                1.0, Color::new(0.8, 0.8, 0.8, 1.0)
            );
        }

        // Draw elements from the map_2d array
        for x in 0..max_width {
            for y in 0..max_height {
                let x_pos = x as f32 * cell_width;
                let y_pos = y as f32 * cell_height;
                
                match map.map_2d[x][y] {
                    Elements::Water => {
                        draw_rectangle(x_pos, y_pos, cell_width, cell_height, WATER_COLOR);
                    },
                    Elements::Plant => {
                        // Get the growth value for this plant
                        let growth = map.plant_growth[x][y];
                        
                        // Calculate size based on growth (0.2 to 1.5)
                        // Normalize growth to a 0.0-1.0 range for visualization
                        let normalized_growth = ((growth - 0.2) / (1.5 - 0.2)) as f32;
                        let size_factor = 0.5 + (normalized_growth * 0.5); // Size from 50% to 100% of cell
                        
                        let size_width = cell_width * size_factor;
                        let size_height = cell_height * size_factor;
                        let offset_x = (cell_width - size_width) / 2.0;
                        let offset_y = (cell_height - size_height) / 2.0;
                        
                        // Interpolate color based on growth
                        let plant_color = Color::new(
                            PLANT_COLOR_MIN.r + (PLANT_COLOR_MAX.r - PLANT_COLOR_MIN.r) * normalized_growth,
                            PLANT_COLOR_MIN.g + (PLANT_COLOR_MAX.g - PLANT_COLOR_MIN.g) * normalized_growth,
                            PLANT_COLOR_MIN.b + (PLANT_COLOR_MAX.b - PLANT_COLOR_MIN.b) * normalized_growth,
                            1.0
                        );
                        
                        draw_rectangle(
                            x_pos + offset_x, 
                            y_pos + offset_y, 
                            size_width, 
                            size_height, 
                            plant_color
                        );
                    },
                    Elements::Empty => {
                        // Empty cells don't need to be drawn
                    }
                }
            }
        }

        // Update map every second
        if last_update.elapsed() >= UPDATE_INTERVAL {
            map.update();
            last_update = Instant::now();
        }

        // Check for exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
} 