use indexmap::IndexMap;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;

pub struct ColonDB {
    pub data: IndexMap<String, Vec<String>>, // a hash map consists of key & value pairs
    filename: String, // so the struct can be imported from a file
}

impl ColonDB {

    pub fn find_database(file_name: &str) -> Self {
        let mut data_base = ColonDB {
            data: IndexMap::new(),
            filename: file_name.to_string(),
        };
        data_base.load_data_from_file();
        data_base
    }

    fn column_name_toindex(&self, column: &str, header: &[String]) -> Option<usize> {
        header.iter().position(|col| col == column)
    }


    fn load_data_from_file(&mut self) {
        if !Path::new(&self.filename).exists() {
            return;
        }
        let file = File::open(&self.filename).expect("Couldn't open File...");
        let file_reader = BufReader::new(file); // reads file to a string. with lines

        for row in file_reader.lines() {
            if let Ok(entry) = row {
                let sep_keyvalue: Vec<&str> = entry.splitn(2, ':').collect(); // separating key and value by :
                
                if sep_keyvalue.len() == 2 {
                    let key = sep_keyvalue[0].to_string();
                    let values_str = sep_keyvalue[1];
                    // convert values string to vec by splitting at comma
                    let values: Vec<String> = values_str.split(',').map(|s| s.to_string()).collect();
                    
                    self.data.insert(key, values);
                }
            }
        }
    }


    
    pub fn save_data_to_file(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;
        
        for (key, values) in &self.data {
            let values_str = values.join(",");
            writeln!(file, "{}:{}", key, values_str)?;
        }
        Ok(())
    }

//###################################################################################################################################
// IT WORKS DONT TOUCH ##############################################################################################################
//###################################################################################################################################
    pub fn insert_item_into_db(&mut self, key: String, column: String, value: String) {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        if let Some(index) = self.column_name_toindex(&column, &header) {
            // Get or create the row for the key
            let row = self.data.entry(key).or_insert_with(|| vec![String::new(); header.len()]);

            // Update the value at the corresponding index
            if index < row.len() {
                row[index] = value;
            } else {
                eprintln!("Index out of bounds: column '{}'", column);
            }

            // Save changes to the file
            self.save_data_to_file().expect("Failed to save Data...");
        } else {
            eprintln!("Column '{}' not found in header!", column);
        }
    }

    pub fn insert_row_into_db(&mut self, key: String, entry_vec: Vec<String>) {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();

        // Ensure the header and entry_vec have the same length
        if entry_vec.len() != header.len() {
            eprintln!(
                "Entry vector length ({}) does not match header length ({}).",
                entry_vec.len(),
                header.len()
            );
            return;
        }


        // Insert or overwrite the entire row for the given key
        self.data.insert(key, entry_vec);

        // Save changes to the file
        self.save_data_to_file().expect("Failed to save Data...");

    }
//####################################################################################################################################
//####################################################################################################################################



    pub fn delete_item(&mut self, key: &str, column: String) {
        // Get the header (the first row's keys)
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        
        // Get the index of the column in the header
        if let Some(index) = self.column_name_toindex(&column, &header) {
            if let Some(row) = self.data.get_mut(key) {
                // Set the value to an empty string (or any other placeholder)
                row[index] = String::new();
                
                // Save changes to the file
                self.save_data_to_file().expect("Failed to save Data to File.");
            } else {
                eprintln!("Key '{}' not found in database!", key);
            }
        } else {
            eprintln!("Column '{}' not found in header!", column);
        }
    }

    pub fn delete_column(&mut self, column: String) {
        // Get the header (the first row's keys)
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        
        // Get the index of the column in the header
        if let Some(index) = self.column_name_toindex(&column, &header) {
            // Iterate through all rows (keys)
            for (_, row) in self.data.iter_mut() {
                // Remove the value at the specified index
                if index < row.len() {
                    row[index] = String::new(); // You can replace with an empty value or something else
                }
            }
    
            // Save the changes to the file
            self.save_data_to_file().expect("Failed to save Data to File.");
        } else {
            eprintln!("Column '{}' not found in header!", column);
        }
    }
    
    
    pub fn delete_row(&mut self, key: &str) {
        if self.data.remove(key).is_some() {
            // Successfully removed the row
            self.save_data_to_file().expect("Failed to save Data to File.");
        } else {
            eprintln!("Key '{}' not found in database!", key);
        }
    }
    
    pub fn select_item(
        &self, 
        key: &str, // ID (key) of the row
        column: &str // Column name to filter
    ) -> Option<String> {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        
        // Find the index of the column
        if let Some(index) = self.column_name_toindex(column, &header) {
            // Check if the specified key exists in the data
            if let Some(row) = self.data.get(key) {
                // Get the value at the specified column index
                if index < row.len() {
                    return Some(row[index].clone()); // Return the value at the specified index
                }
            } else {
                eprintln!("Key '{}' not found in database!", key);
            }
        } else {
            eprintln!("Column '{}' not found in header!", column);
        }
    
        // Return None if key or column not found, or index is out of bounds
        None
    }
    
    
    pub fn select_data(
        &self, 
        row_range: Option<std::ops::Range<usize>>, // A range of indices to select rows
        column_range: Option<Vec<String>> // A list of column names to select
    ) -> ColonDB {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        let mut result = Vec::new();
        
        // If column_range is provided, map column names to their indices
        let column_indices: Vec<usize> = if let Some(columns) = column_range {
            columns.iter()
                .filter_map(|column| self.column_name_toindex(column, &header))
                .collect()
        } else {
            (0..header.len()).collect() // If no columns are specified, select all columns
        };
        
        // Convert the data into a Vec of (key, row) so we can access rows by index
        let rows: Vec<(String, Vec<String>)> = self.data.iter()
            .map(|(key, row)| (key.clone(), row.clone()))
            .collect();
        
        // Get the row range, defaulting to all rows if row_range is None
        let row_range = row_range.unwrap_or(0..rows.len());
        
        // Iterate through the rows within the row_range
        for (key, row) in rows.iter().skip(row_range.start).take(row_range.len()) {
            // Slice the row based on the column indices
            let selected_columns: Vec<String> = column_indices.iter()
                .filter_map(|&index| row.get(index).cloned()) // Get values for each specified column index
                .collect();
            
            // Add the selected row to the result
            result.push((key.clone(), selected_columns));
        }
        
        // Create a new SimpleDB object from the selected data
        let mut new_db = ColonDB {
            data: IndexMap::new(),
            filename: self.filename.clone(), // You may want to keep the same filename or modify as needed
        };
    
        // Populate the new SimpleDB with the selected rows
        for (key, row) in result {
            new_db.data.insert(key, row);
        }
        
        // Return the new SimpleDB object
        new_db
    }
    
    
    

    pub fn print_database(&self) {
        // Loop through the data in the database
        for (key, row) in &self.data {
            let mut line = format!("{key} || "); // Initialize line with the key
            
            // Iterate over each value in the row
            for value in row.iter() {
                if value.is_empty() {
                    line.push_str("empty | "); // Append "empty |" if value is empty
                } else {
                    line.push_str(&format!("{value} | ")); // Append the actual value
                }
            }
            
            // Print the line for this row
            println!("{}", line);
            
            // Print a separator line based on the length of the key and values
            let separator = "-".repeat(line.len()); // Create a separator line of the same length
            println!("{}", separator);
        }
    }
    
    

}

