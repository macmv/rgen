import json
import os

# Construct the path relative to the current script's location
json_file_path = os.path.join(os.path.dirname(__file__), '..', '.keys', 'key_1.json')

# Read and print the JSON content
with open(json_file_path, 'r') as file:
    data = json.load(file)
    print(json.dumps(data, indent=4))
