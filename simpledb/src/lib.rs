use::std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;

pub struct SimpleDB {
    data: HashMap<String, String>, // a hash map consists of key & value pairs
    filename: String, // so the struct can be imported from a file
}

impl SimpleDB {
    pub fn find_database(file_name: &str) -> Self {
        let mut data_base = SimpleDB {
            data: HashMap::new(),
            filename: file_name.to_string(),
        };
        data_base.load_data_from_file();
        data_base
    }

    fn load_data_from_file(&mut self) {
        if !Path::new(&self.filename).exists() {
            return;
        }
        let file = File::open(&self.filename).expect("Couldn't open File...");
        let file_reader = BufReader::new(file);

        for row in file_reader.lines() {
            if let Ok(entry) = row {
                let parts: Vec<&str> = entry.splitn(2, ':').collect();
                if parts.len() == 2 {
                    self.data.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    }
    
    /// Saves the current state of the HashMap to the file
    pub fn save_data_to_file(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;

        for (key, value) in &self.data {
            writeln!(file, "{}:{}", key, value)?;
        }
        Ok(())
    }

    pub fn insert_into_db(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.save_data_to_file().expect("Failed to save Data...");
    }

    pub fn get_value_from_db(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete_from_db(&mut self, key: &str) {
        self.data.remove(key);
        self.save_data_to_file().expect("Failed to save Data to File.");
    }
}
