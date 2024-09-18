import gspread
import os

from oauth2client.service_account import ServiceAccountCredentials

# Define the scope for the API
scope = ["https://spreadsheets.google.com/feeds", "https://www.googleapis.com/auth/spreadsheets",
         "https://www.googleapis.com/auth/drive.file", "https://www.googleapis.com/auth/drive"]

# Use the service account credentials
creds = ServiceAccountCredentials.from_json_keyfile_name(os.path.join(os.path.dirname(__file__), '..', '.keys', 'key_1.json'), scope)
client = gspread.authorize(creds)

# Open the Google Sheet by its name
sheet = client.open('[rgen] Biomes sheet').worksheet('CompositionTables')

# Get all values from the sheet
all_values = sheet.get_all_values()

# Initialize an empty dictionary to hold data categorized by climate and geographic type
data_dict = {}
current_climate = None

# List of geographic types
geo_types = ["STANDARD", "RIVER", "VALLEY", "HILLS", "MOUNTAIN"]

# Parse the Google Sheet data
for row in all_values:
    if not row[0]:  # Skip empty rows
        continue

    if row[0].isupper() and row[0] not in geo_types:  # This is a climate type
        current_climate = row[0]
        data_dict[current_climate] = {geo_type: [] for geo_type in geo_types}  # Initialize all geo types for this climate
        # Debug: Print detected climate
        print(f"Detected climate: {current_climate}")

    elif row[0].isdigit() and current_climate:  # Data rows containing percentages and names
        for i, geo_type in enumerate(geo_types):  # Loop through the geographic types
            # Each geographic type has two columns: percentage and name
            percentage_index = i * 2
            name_index = percentage_index + 1

            # Ensure the indices are within bounds and that the name isn't 'total'
            if percentage_index < len(row) and name_index < len(row) and row[name_index] != 'total':
                percentage = row[percentage_index]
                name = row[name_index]

                if percentage.isdigit() and name:  # Ensure valid entries
                    data_dict[current_climate][geo_type].append(f'b!({float(percentage)}, {name})')

# Generate Rust code
rust_code = []

for climate, geo_data in data_dict.items():
    for geo_type, entries in geo_data.items():
        if entries:  # Only generate code if there are entries to add
            formatted_entries = ',\n    '.join(entries)
            rust_code.append(f'let (GeographicType::{geo_type.capitalize()}, ClimateType::{climate.capitalize()}) = &[\n    {formatted_entries},\n];\n')

# Debug: Print generated Rust code before writing to file
print("Generated Rust Code:")
print('\n'.join(rust_code))

# Write to a Rust file
output_file_path = os.path.join(os.path.dirname(__file__), 'test.rs')
with open(output_file_path, 'w') as rust_file:
    rust_file.write('\n'.join(rust_code))

print("Rust code generated and written to test.rs")