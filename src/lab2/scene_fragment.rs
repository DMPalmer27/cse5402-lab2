/*
 * Author: Daniel Palmer
 * Email: d.m.palmer@wustl.edu
 * File: scene_fragment.rs
 * Summary:
 *
 */

use std::collections::HashSet;

use super::player::Player;
use super::declarations;


type PlayConfig = Vec<(String, String)>; // (character name, associated text file)

const TITLE_INDEX: usize = 0;
const CHARACTER_NAME: usize = 0;
const CHARACTER_FILE: usize = 1;
const CONFIG_LINE_TOKENS: usize = 2;
const MIN_CONFIG_LINES: usize = 2;
const FIRST_LINE: usize = 0;
const EXPECTED_NUM_SPEAKERS: usize = 1;


pub struct SceneFragment {
    scene_title: String,
    characters: Vec<Player>,
}


impl SceneFragment {
    pub fn new(title: &str) -> Self {
        Self {
            scene_title: title.to_string();,
            characters: Vec::new(),
        }
    }

    // This function processes a passed in PlayConfig. For each item in the PlayConfig it creates a
    // Player, adds it to the Play's characters, and prepares the character with its associated
    // text file. 
    // If it fails the error is propagated out and otherwise Ok(()) is returned
    fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for tup in play_config {
            match tup {
                (name, file) => {
                    let mut character = Player::new(&name);
                    character.prepare(&file)?;
                    self.characters.push(character);
                }
            }
        }
        Ok(())
    }

    // This function splits the passed in line into two separate tokens and adds them as a tuple to
    // the passed in PlayConfig. If the tokens could not be properly extracted and whinge mode is
    // on it complains, but if there were at least two tokens (the minimum amount) it adds the
    // line.
    fn add_config(line: &str, play_config: &mut PlayConfig) {
        let delimited_tokens: Vec<&str> = line.split_whitespace().collect();
        if delimited_tokens.len() != CONFIG_LINE_TOKENS {
            use std::sync::atomic::Ordering;
            if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                eprintln!("Warning: There were not exactly two distinct tokens in the line {}", line);
            }
        }
        if delimited_tokens.len() >= CONFIG_LINE_TOKENS {
            play_config.push((
                    delimited_tokens[CHARACTER_NAME].to_string(),
                    delimited_tokens[CHARACTER_FILE].to_string()
                    ));
        }
    }



    // This function reads a given config file name and populates the passed in title and
    // play_config with the relevant information from this config file. It propagates any errors
    // out and otherwise returns Ok(())
    fn read_config(config_file_name: &str, title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();
        declarations::grab_trimmed_file_lines(config_file_name, &mut lines)?;
        if lines.len() < MIN_CONFIG_LINES {
            eprintln!("Error: the config file must contain at least one character and associated text file");
            return Err(declarations::ERR_SCRIPT_GEN);
        }
        for (i, line) in lines.iter().enumerate() {
            if i == TITLE_INDEX {
                *title = line.clone();
            } else {
                Self::add_config(line, play_config);
            }
        }
        Ok(())
    }


    // This method does the script generation for a given scene. It uses the above functions to
    // populate the self Play with associated information.
    pub fn prepare(&mut self, config_file_name: &str) -> Result<(), u8> {
        let mut play_config: PlayConfig = Default::default();
        Self::read_config(config_file_name, &mut self.scene_title, &mut play_config)?;
        self.process_config(&play_config)?;
        self.characters.sort();
        Ok(())
    }


    // This method prints the play line by line by finding the player that has the next line and
    // printing it out.
    pub fn recite(&mut self) {
        let mut next_line_number = FIRST_LINE;
        let mut cur_speaker = String::new();
        loop {
            let min_line_number = match self.characters
                .iter()
                .filter_map(|c| c.next_line())
                .min(){
                Some(n) => n,
                None => break,
            };
            
            // Skip over any missing line numbers, complaining if whinge mode is on
            while min_line_number > next_line_number {
                use std::sync::atomic::Ordering;
                if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                    eprintln!("Warning: missing line {}", next_line_number);
                }
                next_line_number += 1;
            }


            let next_characters: Vec<&mut Player> = self.characters
                .iter_mut()
                .filter(|c| c.next_line() == Some(min_line_number))
                .collect(); // Holds all characters who have a line which number is the minimum
            if next_characters.len() != EXPECTED_NUM_SPEAKERS {
                use std::sync::atomic::Ordering;
                if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                    eprintln!("Warning: there are {} characters who have a line with number {}", next_characters.len(), min_line_number);
                }
            }
            
            for c in next_characters {
                c.speak(&mut cur_speaker);
            }
            
            next_line_number += 1;
        }
    }

    pub fn enter(&self, other: &Self) {
        if !self.scene_title.trim().is_empty(){
            println!("{}", self.scene_title);
        }
        let other_names: HashSet<&str> = other.characters.iter()
            .map(|c| &c.name)
            .collect();
        for name in self.characters.iter().map(|c| c.name.as_str()) {
            if !other_names.contains(name) {
                println!("[Enter {}]", name);
            }
        }
        
    }

    pub fn enter_all(&self) {
        if !self.scene_title.trim().is_empty(){
            println!("{}", self.scene_title);
        }
        for name in self.characters.iter().map(|c| c.name.as_str()) {
            println!("[Enter {}]", name);
        }
    }

    pub fn exit(&self, other: &Self) {
        let other_names: HashSet<&str> = other.characters.iter()
            .map(|c| &c.name)
            .collect();
        for name in self.characters.iter().rev().map(|c| c.name.as_str()) {
            if !other_names.contains(name) {
                println!("[Exit {}]", name);
            }
        }
    }

    pub fn exit_all(&self) {
        for name in self.characters.iter().rev().map(|c| c.name.as_str()) {
            println!("[Exit {}]", name);
        }
    }


}
