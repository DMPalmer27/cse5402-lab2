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
const FIRST_FRAGMENT: bool = 0;
const ONE_FRAGMENT: usize = 1;




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



    // This function reads a given script file name and populates the passed in 
    // script_config with the relevant information from this config file. It propagates any errors
    // out and otherwise returns Ok(())
    fn read_config(script_file_name: &str, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines: Vec<String> = Vec::new();
        declarations::grab_trimmed_file_lines(script_file_name, &mut lines)?;
        if lines.len() == EMPTY {
            eprintln!("Error: the script gen file must contain at least 1 line");
            return Err(declarations::ERR_SCRIPT_GEN);
        }
        for line in &lines {
            Self::add_config(line, script_config);
        }
        Ok(())
    }


    // This method does the script generation for a given play. It uses the above functions to
    // populate the self Play with associated information.
    pub fn prepare(&mut self, script_file_name: &str) -> Result<(), u8> {
        let mut script_config: ScriptConfig = Default::default();
        Self::read_config(script_file_name, &mut script_config)?;
        self.process_config(&script_config)?;
        if self.fragments.len() != EMPTY && !self.fragments[FIRST_FRAGMENT].title.is_empty() {
            Ok(())
        } else {
            eprintln!("Error: script generation failed");
            Err(declarations::ERR_SCRIPT_GEN)
        }
    }


    // This function prints the script by iterating over each scene fragment and printing
    // everything required for it, including character entrances, exits, and lines.
    pub fn recite(&mut self) { 
        for (i, frag) in self.fragments.iter().enumerate(){
            if i == FIRST_FRAGMENT {
                frag.enter_all();
                frag.recite();
                if self.fragments.len() > ONE_FRAGMENT {
                    frag.exit(&self.fragments[i+1]);
                } else {
                    frag.exit_all();
                }
            } else if i == self.fragments.len()-1 {
                frag.enter(&self.fragments[i-1]);
                frag.recite();
                frag.exit_all();
            } else {
                frag.enter(&self.fragments[i-1]);
                frag.recite();
                frag.exit(&self.fragments[i+1]);
            }
        }
    }

}
