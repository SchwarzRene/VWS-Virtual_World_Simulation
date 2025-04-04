pub struct Config {
    pub max_width: usize,
    pub max_height: usize,
    pub initial_water_level: i32,
    pub water_spread_factor: i32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            max_width: 130,
            max_height: 80,
            initial_water_level: 15,
            water_spread_factor: 30,
        }
    }
} 