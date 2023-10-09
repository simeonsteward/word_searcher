mod file_io;

use colored::*;

fn main() -> std::io::Result<()> {
    // Read the map and words from files
    let map: Vec<Vec<char>> = file_io::get_map().expect("Map Read Error");
    let words = file_io::get_words().expect("Words Read Error");

    // Initialize directions for word search
    let directions: [(i32, i32); 8] = [
        (1, 0),   // Right
        (-1, 0),  // Left
        (0, 1),   // Down
        (0, -1),  // Up
        (1, 1),   // Diagonal (right-down)
        (-1, -1), // Diagonal (left-up)
        (1, -1),  // Diagonal (right-up)
        (-1, 1),  // Diagonal (left-down)
    ];

    // Create a separate map for highlighting
    let mut highlighted_map: Vec<Vec<ColoredString>> = map
        .iter()
        .map(|row| row.iter().map(|&c| c.to_string().white()).collect())
        .collect();

    // Iterate through the map
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, _char) in row.iter().enumerate() {
            // Check for words starting at this position in all directions
            for &(dx, dy) in &directions {
                let mut word = String::new();
                let mut x = col_idx as i32;
                let mut y = row_idx as i32;

                while x >= 0 && y >= 0 && (x as usize) < row.len() && (y as usize) < map.len() {
                    word.push(map[y as usize][x as usize]);
                    if words.contains(&word) {
                        // Highlight the word by storing individual characters as strings
                        for (i, c) in word.chars().enumerate() {
                            let char_idx = col_idx as i32 + i as i32 * dx;
                            let row_idx = row_idx as i32 + i as i32 * dy;
                            highlighted_map[row_idx as usize][char_idx as usize] =
                                c.to_string().yellow();
                        }
                        highlighted_map[row_idx as usize][col_idx as usize] =
                            word.chars().nth(0).expect("REASON").to_string().red();
                    }
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    // Print the map with highlighted words
    println!("Highlighted Map:");
    for row in &highlighted_map {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    
    println!("Words");
    for word in words{
        println!("\"{}\"",word);
    }

    Ok(())
}
