
use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead}; // self ?
use std::path::Path;


pub struct SimpleDB {
    pub data: IndexMap<String, String>, 
    filename: String, // so the struct can be imported from a file
}

impl SimpleDB {
    pub fn find_database(file_name: &str) -> Result<Self> {
        let mut data_base = SimpleDB {
            data: IndexMap::new(),
            filename: file_name.to_string(),
        };
        data_base.load_data_from_file()?;
        Ok(data_base)
    }

    fn load_data_from_file(&mut self) -> Result<()> {
        if !Path::new(&self.filename).exists() {
            return Ok(());
        }

        let file = File::open(&self.filename)
            .with_context(|| format!("Failed to open file: {}", &self.filename))?;
        let file_reader = BufReader::new(file);

        for row in file_reader.lines() {
            let entry = row.with_context(|| "Failed to read a line from the file")?;
            let parts: Vec<&str> = entry.splitn(2, ':').collect();
            if parts.len() == 2 {
                self.data.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        Ok(())
    }
    
    pub fn save_data_to_file(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)
            .with_context(|| format!("Failed to open file for writing: {}", &self.filename))?;

        for (key, value) in &self.data {
            writeln!(file, "{}:{}", key, value)
                .with_context(|| format!("Failed to write key-value pair to file: {}:{}", key, value))?;
        }
        Ok(())
    }

    pub fn insert_into_db(&mut self, key: String, value: String) -> Result<()> {
        self.data.insert(key, value);
        self.save_data_to_file().with_context(|| "Failed to save data after insertion")?;
        Ok(())
    }

    pub fn get_value_from_db(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn sort_by_key(&mut self) -> Result<(), String> {
        if self.data.is_empty() {
            return Err("Database is empty. No sorting needed.".to_string());
        }

        let mut sorted: Vec<_> = self.data.clone().into_iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(&b.0)); 

        self.data.clear();
        for (key, value) in sorted {
            self.data.insert(key, value);
        }

        if let Err(e) = self.save_data_to_file() {
            return Err(format!("Error saving sorted data to file: {}", e));
        }

        Ok(())
    }

    pub fn sort_by_value(&mut self) -> Result<(), String> {
        if self.data.is_empty() {
            return Err("Database is empty. No sorting needed.".to_string());
        }

        let mut sorted: Vec<_> = self.data.clone().into_iter().collect();
        sorted.sort_by(|a, b| a.1.cmp(&b.1)); 

        self.data.clear();
        for (key, value) in sorted {
            self.data.insert(key, value);
        }

        if let Err(e) = self.save_data_to_file() {
            return Err(format!("Error saving sorted data to file: {}", e));
        }

        Ok(())
    }

    pub fn delete_from_db(&mut self, key: &str) -> Result<()> {
        if self.data.shift_remove(key).is_none() {
            anyhow::bail!("Key '{}' does not exist in the database", key);
        }
        self.save_data_to_file()
            .with_context(|| "Failed to save data after deletion")?;
        Ok(())
    }

    pub fn print_db(&self) {
        for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }

}

