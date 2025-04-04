# Plant Tracking System

This system allows you to track the number of plants in the virtual world simulation over time and visualize the data.

## How It Works

1. The Rust code records the number of plants at each update cycle in a JSON file.
2. The Python script reads this JSON file and creates a visualization of the plant count over time.

## Usage

### Running the Simulation

The plant tracking is automatically enabled in the main application. When you run the simulation, it will create a file called `plant_history.json` in the project directory.

```bash
cargo run
```

### Visualizing the Data

To visualize the plant count data, use the provided Python script:

```bash
python visualize_plant_count.py
```

By default, it will read from `plant_history.json` and save the visualization to `plant_count.png`.

You can specify different input and output files:

```bash
python visualize_plant_count.py --file my_plant_data.json --output my_visualization.png
```

## Requirements for Visualization

The Python script requires the following packages:
- matplotlib
- argparse

You can install them using pip:

```bash
pip install matplotlib
```

## JSON Data Format

The plant history is stored in a JSON file with the following structure:

```json
{
  "records": [
    {
      "timestamp": "2023-04-04 12:34:56",
      "count": 10
    },
    {
      "timestamp": "2023-04-04 12:35:00",
      "count": 12
    },
    ...
  ]
}
```

Each record contains:
- `timestamp`: The date and time when the count was recorded
- `count`: The number of plants at that time 