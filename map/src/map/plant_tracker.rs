use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use serde::{Serialize, Deserialize};
use chrono::Local;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantCount {
    pub timestamp: String,
    pub count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlantHistory {
    pub records: Vec<PlantCount>,
}

pub struct PlantTracker {
    file_path: String,
    history: PlantHistory,
}

impl PlantTracker {
    pub fn new(file_path: &str) -> Self {
        let history = if Path::new(file_path).exists() {
            // Try to read existing history
            match Self::read_history(file_path) {
                Ok(h) => h,
                Err(_) => PlantHistory { records: Vec::new() },
            }
        } else {
            // Create new history if file doesn't exist
            PlantHistory { records: Vec::new() }
        };

        PlantTracker {
            file_path: file_path.to_string(),
            history,
        }
    }

    fn read_history(file_path: &str) -> io::Result<PlantHistory> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let history: PlantHistory = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(history)
    }

    pub fn record_count(&mut self, count: usize) -> io::Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let record = PlantCount { timestamp, count };
        
        self.history.records.push(record);
        
        // Write the updated history to the file
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)?;
        
        serde_json::to_writer_pretty(file, &self.history)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn get_history(&self) -> &PlantHistory {
        &self.history
    }
} 