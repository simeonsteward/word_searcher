use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn get_words() -> Result<Vec<String>, io::Error> {
    let words_path = "res/words.txt";
    let mut words = Vec::new();

    // Open and parse the words.txt file
    if let Ok(words_file) = File::open(&words_path) {
        let words_reader = BufReader::new(words_file);
        for line in words_reader.lines() {
            if let Ok(line) = line {
                // Trim whitespace from the line and ignore empty lines
                let trimmed_line = line.replace(" ", "").replace("\t", "").to_lowercase();
                if !trimmed_line.is_empty() {
                    words.push(trimmed_line.to_string());
                }
            }
        }
        Ok(words)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to open words.txt",
        ))
    }
}

pub fn get_map() -> Result<Vec<Vec<char>>, io::Error> {
    let map_path = "res/map.txt";
    let mut map = Vec::new();

    // Open and parse the map.txt file
    if let Ok(map_file) = File::open(&map_path) {
        let map_reader = BufReader::new(map_file);
        let mut current_row = Vec::new();

        for line in map_reader.lines() {
            if let Ok(line) = line {
                let cleaned_line = line.replace(" ", "").replace("\t", "").to_lowercase();
                if !cleaned_line.is_empty() {
                    // Collect characters into the current row
                    current_row.extend(cleaned_line.chars());

                    // If the current row is 24 characters long, add it to the map
                    if current_row.len() == 24 {
                        map.push(current_row.clone());
                        current_row.clear();
                    }
                }
            }
        }

        // Check if there's an incomplete row at the end and add it to the map
        if !current_row.is_empty() {
            map.push(current_row);
        }

        Ok(map)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to open map.txt",
        ))
    }
}