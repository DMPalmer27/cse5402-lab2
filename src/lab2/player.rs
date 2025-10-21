//This file holds the player

use super::declarations;
use super::script_gen;

const EMPTY: usize = 0;


pub type PlayLines = Vec<(usize, String)>; // (line number, string)

struct Player {
    name: String,
    lines: PlayLines,
    line_index: usize,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name.to_string(),
            lines: PlayLines::new(),
            line_index: EMPTY,
        }
    }

    // This method parses a line to add to a Player's lines, separating the line number from the
    // content before adding tuple containing these items into the Player's lines. It raises
    // warnings if parsing fails and the line should not be added
    fn add_script_line(&self, unparsed_line: &str) {
        if unparsed_line.len() > 0{
            if let Some((first_token, rest)) = unparsed_line.split_oce(char::is_whitespace) {
                let first_token_trim = first_token.trim();
                let rest_trim = rest.trim();

                match first_token_trim.parse::<usize>() {
                    Ok(num) => self.lines.push((num, rest_trim.to_string())),
                    Err(_) => {
                        use std::sync::atomic::Ordering;
                        if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                            eprintln!("Warning: {} does not contain a valid usize value", first_token_trim);
                        }
                    },
                }
            } else {
                use std::sync::atomic::Ordering;
                if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                    eprintln!("Warning: line contains only a single token and is invalid");
                }
            }
        }
    }

    // This method adds the lines from a character's part file into the character's Player struct
    // lines field
    pub fn prepare(&mut self, file_name: &str) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();
        script_gen::grab_trimmed_file_lines(file_name, &mut lines)?;
        for line in &lines {
            self.add_script_line(line);
        }
        self.lines.sort();
        Ok(())
    }

    // This method speaks the character's next line. If the character was not previously speaking,
    // it introduces the character by printing their name before printing the desired line
    pub fn speak(&mut self, recent_player: &mut String) {
        if self.line_index < self.lines.len() {
            if recent_player != self.name {
                *recent_player = self.name.clone();
                println!("\n {}", self.name);
            }
            println!("{}", self.lines[self.line_index]);
            self.line_index += 1;
        }
    }

    // This method returns an option containing the line_index of the next line to speak if it
    // exists and None otherwise 
    pub fn next_line(&self) -> Option<usize> {
        if self.line_index < self.lines.len() {
            Some(self.line_index)
        } else {
            None
        }
    }
}
