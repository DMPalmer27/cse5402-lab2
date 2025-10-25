/*
 * Author: Daniel Palmer
 * Email: d.m.palmer@wustl.edu
 * File: play.rs
 * Summary: This file contains the Play struct and its implementation. A Play is the
 * type used for coordinating the script generation of a scene. It handles the 
 * individual characters as instances of Player structs and is primarily used to 
 * recite the play.
 *
 */


use super::player::Player;
use super::scene_fragment::SceneFragment;
use super::declarations;


type ScriptConfig = Vec<(bool, String)>;

const SCENE_INDICATOR: &str = "[SCENE]";
const EMPTY: usize = 0;
const SINGLE_TOKEN: usize = 1;
const FIRST_TOKEN: usize = 0;
const SECOND_TOKEN: usize = 1;
const NEW_SCENE_BOOL: bool = true;
const CONFIG_FILE_BOOL: bool = false;


const TITLE_INDEX: usize = 0;
const CHARACTER_NAME: usize = 0;
const CHARACTER_FILE: usize = 1;
const CONFIG_LINE_TOKENS: usize = 2;
const MIN_CONFIG_LINES: usize = 2;
const FIRST_LINE: usize = 0;
const EXPECTED_NUM_SPEAKERS: usize = 1;


pub struct Play {
    fragments: Vec<SceneFragment>,
}


impl Play {
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }

    // This function processes a passed in ScriptConfig. For each item in the ScriptConfig if it contains a scene title it updates the title and otherwise creates a new SceneFragment, adds it to the Play's fragments, and prepares the fragment with its associated file. If it fails, the error is propagated out and otherwise Ok(()) is returned
    fn process_config(&mut self, script_config: &ScriptConfig) -> Result<(), u8> {
        let mut title  = String::new();
        for tup in script_config {
            match tup {
                (true, text) => { //Text is a new title
                    title = text.clone();
                },
                (false, text) => {
                    let mut frag = SceneFragment::new(&title);
                    frag.prepare(&text)?;
                    self.fragments.push(frag);
                    title = "".to_string();
                }
            }
        }
        Ok(())
    }

    // This function separates the tokens in the passed in line, creating a new scene if the first
    // token is [scene] and there is a scene title after. Otherwise, treats the first token as a
    // config file. In either success case an element containing the info is pushed to the passed
    // in ScriptConfig, and in the event of an empty line or [scene] is the first token with
    // nothing after nothing is pushed.
    fn add_config(line: &str, script_config: &mut ScriptConfig) {
        let trimmed = line.trim();
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() == EMPTY {
            return;
        }
        if tokens.len() == SINGLE_TOKEN && tokens[FIRST_TOKEN] == SCENE_INDICATOR {
            use std::sync::atomic::Ordering;
            if declarations::WHINGE_ON.load(Ordering::SeqCst){
                eprintln!("Warning: scene identified but has no title so has not been added");
            }
            return;
        }
        if tokens[FIRST_TOKEN] == SCENE_INDICATOR {
            let rest = tokens[SECOND_TOKEN..].join(" ");
            script_config.push((NEW_SCENE_BOOL, rest));
        } else {
            script_config.push((CONFIG_FILE_BOOL, tokens[FIRST_TOKEN].to_string()));
            if tokens.len() != SINGLE_TOKEN{
                use std::sync::atomic::Ordering;
                if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                    eprintln!("Warning: there are additional tokens in the line \"{}\" that is being treated as a config file name", line);
                }
            }
        }
            
    }



    // This function reads a given config file name and populates the passed in title and
    // play_config with the relevant information from this config file. It propagates any errors
    // out and otherwise returns Ok(())
    fn read_config(config_file_name: &str, title: &mut String, play_config: &mut ScriptConfig) -> Result<(), u8> {
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
        let mut play_config: ScriptConfig = Default::default();
        Self::read_config(config_file_name, &mut self.scene_title, &mut play_config)?;
        self.process_config(&play_config)?;
        Ok(())
    }


    // This method prints the play line by line by finding the player that has the next line and
    // printing it out.
    pub fn recite(&mut self) {
        println!("{}", self.scene_title);
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

}
