#!/usr/bin/env python3
import json
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime
import argparse

def parse_arguments():
    parser = argparse.ArgumentParser(description='Visualize plant count over time')
    parser.add_argument('--file', type=str, default='plant_history.json',
                        help='Path to the JSON file containing plant count data')
    parser.add_argument('--output', type=str, default='plant_count.png',
                        help='Path to save the output image')
    return parser.parse_args()

def load_plant_data(file_path):
    try:
        with open(file_path, 'r') as f:
            data = json.load(f)
        return data
    except FileNotFoundError:
        print(f"Error: File '{file_path}' not found.")
        return None
    except json.JSONDecodeError:
        print(f"Error: File '{file_path}' is not a valid JSON file.")
        return None

def visualize_plant_count(data, output_file):
    if not data or 'records' not in data:
        print("Error: Invalid data format.")
        return
    
    # Extract timestamps and counts
    timestamps = []
    counts = []
    
    for record in data['records']:
        try:
            # Parse the timestamp string to datetime object
            timestamp = datetime.strptime(record['timestamp'], '%Y-%m-%d %H:%M:%S')
            timestamps.append(timestamp)
            counts.append(record['count'])
        except (KeyError, ValueError) as e:
            print(f"Error parsing record: {e}")
            continue
    
    if not timestamps:
        print("No valid data points found.")
        return
    
    # Create the plot
    plt.figure(figsize=(12, 6))
    plt.plot(timestamps, counts, marker='o', linestyle='-', linewidth=2, markersize=4)
    
    # Format the x-axis to show dates nicely
    plt.gca().xaxis.set_major_formatter(mdates.DateFormatter('%H:%M:%S'))
    plt.gca().xaxis.set_major_locator(mdates.AutoDateLocator())
    plt.gcf().autofmt_xdate()  # Rotate and align the tick labels
    
    # Add labels and title
    plt.xlabel('Time')
    plt.ylabel('Number of Plants')
    plt.title('Plant Count Over Time')
    plt.grid(True, linestyle='--', alpha=0.7)
    
    # Add statistics
    if len(counts) > 0:
        avg_count = sum(counts) / len(counts)
        max_count = max(counts)
        min_count = min(counts)
        plt.text(0.02, 0.98, f'Average: {avg_count:.1f}\nMax: {max_count}\nMin: {min_count}',
                 transform=plt.gca().transAxes, verticalalignment='top',
                 bbox=dict(boxstyle='round', facecolor='white', alpha=0.8))
    
    # Save the plot
    plt.tight_layout()
    plt.savefig(output_file, dpi=300)
    print(f"Plot saved to {output_file}")
    
    # Show the plot
    plt.show()

def main():
    args = parse_arguments()
    data = load_plant_data(args.file)
    if data:
        visualize_plant_count(data, args.output)

if __name__ == "__main__":
    main() 