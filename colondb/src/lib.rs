use indexmap::IndexMap;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, BufReader, BufRead};
use std::path::Path;
use thiserror::Error;
use anyhow::{Context, Result};




#[derive(Error, Debug)]
pub enum DataStoreError {

    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),

    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("header length doesnt match row length")]
    InvalidRowLength,

    #[error("unknown data store error")]
    Unknown,
     
    #[error("could not find column and/or key in database")]
    CoordinatesNotFound,   

    #[error("invalid selection range (expected {expected:?}, found {found:?})")]
    InvalidSelectionRange {
        expected: String,
        found: String,
    },

    #[error("tried to load data from a filename that does not exists")]
    Filename,

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    #[error("duplicate column name `{0}`")]
    DuplicateColumn(String),
}




pub struct ColonDB {
    pub data: IndexMap<String, Vec<String>>, 
    filename: String, 
}




impl ColonDB {

    pub fn find_database(file_name: &str) -> Self {
        let mut data_base = ColonDB {
            data: IndexMap::new(),
            filename: file_name.to_string(),
        };
        let _ = data_base.load_data_from_file();
        data_base
    }


    fn column_name_toindex(&self, column: &str, header: &[String]) -> Option<usize> {
        if column == "key" {
            return Some(0); 
        }

        header.iter().position(|col| col == column).map(|idx| idx + 1)
    }


    fn load_data_from_file(&mut self) -> Result<(), DataStoreError> {
        if !Path::new(&self.filename).exists() {
            return Err(DataStoreError::Filename);
        }
        let file = File::open(&self.filename).expect("Couldn't open File...");
        let file_reader = BufReader::new(file); 

        for row in file_reader.lines() {
            if let Ok(entry) = row {
                let sep_keyvalue: Vec<&str> = entry.splitn(2, ':').collect(); 
                
                if sep_keyvalue.len() == 2 {
                    let key = sep_keyvalue[0].to_string();
                    let values_str = sep_keyvalue[1];
                    let values: Vec<String> = values_str.split(',').map(|s| s.to_string()).collect();
                    
                    self.data.insert(key, values);
                }
            }
        }
    return Ok(())
    }

    
    pub fn save_data_to_file(&self) -> Result<()> {
       
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)
            .with_context(|| format!("Failed to open file for writing: {}", &self.filename))?;
        
        for (key, values) in &self.data {
            let values_str = values.join(",");
            writeln!(file, "{}:{}", key, values_str)
                .with_context(|| format!("Failed to write key-values pair to file: {}:{}", key, values_str))?;
        }
        return Ok(())
    }


    pub fn set_header(&mut self, key: String, values: Vec<String>) -> Result<(), DataStoreError> {
        if values.is_empty() {
            return Err(DataStoreError::InvalidHeader {
                expected: "non-empty values".to_string(),
                found: "empty vector".to_string(),
            });
        }

        if self.data.contains_key(&key) {
            self.data.remove(&key);
        }

        let mut new_data = IndexMap::new();
        new_data.insert(key.clone(), values);

        for (existing_key, existing_values) in &self.data {
            new_data.insert(existing_key.clone(), existing_values.clone());
        }

        self.data = new_data;

        self.save_data_to_file();
        Ok(())
    }


    pub fn insert_item_into_db(
        &mut self, 
        key: String, 
        column: String, 
        value: String
    ) -> Result<(), DataStoreError> {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        let index = match self.column_name_toindex(&column, &header) {
            Some(idx) => idx,
            None => {
                println!("Column '{}' not found in header!", column);
                return Err(DataStoreError::CoordinatesNotFound);
            }
        };

        let row = self.data.entry(key).or_insert_with(|| vec![String::new(); header.len()]);

        if index-1 < row.len() {
            row[index-1] = value;
        } else {
            println!("Index out of bounds: column '{}'", column);
            return Err(DataStoreError::CoordinatesNotFound);
        }

        self.save_data_to_file();
        Ok(())
    }


    pub fn append_row_to_db(
        &mut self, 
        key: String, 
        entry_vec: Vec<String>
        ) -> Result<(), DataStoreError> {

        self.data.insert(key, entry_vec);
        let _ = self.save_data_to_file();
        return Ok(())
    }

    
    pub fn delete_row(&mut self, key: &str) -> Result<(), DataStoreError>  {
        if self.data.remove(key).is_some() {
            let _ = self.save_data_to_file();
        } else {
            eprintln!("Key '{}' not found in database!", key);
            return Err(DataStoreError::CoordinatesNotFound)
        }
        return Ok(())
    }
    

    pub fn get_item(
        &self, 
        key: &str, 
        column: &str 
    ) -> Result<String, DataStoreError> {

        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        
        if let Some(index) = self.column_name_toindex(column, &header) {
            if let Some(row) = self.data.get(key) {
                if index < row.len() {
                    return Ok(row[index].clone());
                } else {
                    eprintln!("Key '{}' not found in database!", key);
                    return Err(DataStoreError::CoordinatesNotFound)
                }

            } else {
                eprintln!("Column '{}' not found in header!", column);
                return Err(DataStoreError::CoordinatesNotFound)
            }
        }
        Ok("".to_string())
    }
    

    pub fn append_column(&mut self, column_name: String, default_value: String) -> Result<(), DataStoreError> {
        let mut header = self.data.values().next().cloned().unwrap_or_default();
        
        if header.contains(&column_name) {
            return Err(DataStoreError::DuplicateColumn(column_name));
        }

        header.push(column_name.clone());
        
        for row in self.data.values_mut() {
            row.push(default_value.clone());
        }

        let mut comp_data = self.data.clone();

        if let Some((key, _)) = comp_data.iter_mut().next() {
            self.data.insert(key.clone(), header);
        }

        self.save_data_to_file();

        Ok(())
    }


    pub fn select_data(
        &self,
        row_range: Option<std::ops::Range<usize>>,
        column_range: Option<Vec<String>>,
    ) -> Result<ColonDB, DataStoreError> {
        let header: Vec<String> = self.data.values().next().cloned().unwrap_or_default();
        let mut result = Vec::new();

        let col_range = column_range.clone();
        let column_indices: Vec<Option<usize>> = if let Some(columns) = column_range {
            columns
                .iter()
                .map(|column| self.column_name_toindex(column, &header))
                .collect()
        } else {
            (0..header.len()).map(Some).collect()
        };

        if column_indices.contains(&None) {
            let missing_columns: Vec<String> = col_range
                .unwrap_or_default()
                .into_iter()
                .filter(|column| self.column_name_toindex(column, &header).is_none())
                .collect();

            return Err(DataStoreError::InvalidSelectionRange {
                expected: format!("Valid column names from header: {:?}", header),
                found: format!("Missing columns: {:?}", missing_columns),
            });
        }

        let column_indices: Vec<usize> = column_indices
            .into_iter()
            .flatten()
            .filter(|&index| index > 0) 
            .map(|index| index - 1)
            .collect();


        let rows: Vec<(String, Vec<String>)> = self
            .data
            .iter()
            .map(|(key, row)| (key.clone(), row.clone()))
            .collect();

        let row_range = row_range.unwrap_or(0..rows.len());

        if row_range.end > rows.len() {
            return Err(DataStoreError::InvalidSelectionRange {
                expected: format!("row range within 0..{}", rows.len()),
                found: format!("end of range {}", row_range.end),
            });
        }

        for (key, row) in rows.iter().skip(row_range.start).take(row_range.len()) {
            let selected_columns: Vec<String> = column_indices
                .iter()
                .filter_map(|&index| row.get(index).cloned())
                .collect();

            result.push((key.clone(), selected_columns));
        }

        let mut new_db = ColonDB {
            data: IndexMap::new(),
            filename: self.filename.clone(),
        };

        for (key, row) in result {
            new_db.data.insert(key, row);
        }

        Ok(new_db)
    }

    
    pub fn print_database(&self) {
        for (key, row) in &self.data {
            let mut line = format!("{key} || "); 
            
            for value in row.iter() {
                if value.is_empty() {
                    line.push_str("empty | "); 
                } else {
                    line.push_str(&format!("{value} | ")); 
                }
            }
            
            println!("{}", line);
            
            let separator = "-".repeat(line.len()); 
            println!("{}", separator);
        }
    }
}

