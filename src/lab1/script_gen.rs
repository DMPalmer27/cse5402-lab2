//This file includes functions, types, and constants used for generating the play's script

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use super::declarations;

type PlayConfig = Vec<(String, String)>; // (character name, associated text file)
const TITLE_INDEX: usize = 0;
//const FIRST_CHARACTER_INDEX: usize = 1; The document told us to use this, but I just used a
//loop and after tracking the title index this becomes unnecessary
const CHARACTER_NAME: usize = 0;
const CHARACTER_FILE: usize = 1;
const CONFIG_LINE_TOKENS: usize = 2;
const MIN_CONFIG_LINES: usize = 2;
const TEST_APPEND: &str = "test_files/";

// This function takes as parameters a mutable reference to a play object, a reference to the
// line that will be added to the play, and a reference to a string that contains the character who
// is speaking.
// It splits the unparsed line into its line number and content which, if successful, then results
// in the line being added to the play. Otherwise, if the first token is not a number and whinge
// mode is on the program prints a complaint message without adding the line. If any other call fails it is due to the
// presence of whitespace so the line is not added and the function runs successfully. 
fn add_script_line(play: &mut declarations::Play, unparsed_line: &String, char_part_name: &String) {
    if unparsed_line.len() > 0 {
        if let Some((first_token, rest)) = unparsed_line.split_once(char::is_whitespace) {
            let first_token_trim = first_token.trim();
            let rest_trim = rest.trim();

            match first_token_trim.parse::<usize>() {
                Ok(num) => play.push((num, char_part_name.clone(), rest_trim.to_string())),
                Err(_) => {
                    use std::sync::atomic::Ordering;
                    if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                        println!("Warning: {} does not contain a valid usize value", first_token_trim);
                    }
                },
            }

        } else {
            use std::sync::atomic::Ordering;
            if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                println!("Warning: line contains only a single token and is invalid");
            }
        }
    }
}

// This function is used to open and read lines from a file. It takes as parameters a string
// reference that
// is the final name to be opened, and a mutable reference to a vector of strings in which it will
// store the lines read from the file. 
// For my specific program, I put all test files within a specific subdirectory so I prepend the
// file name with a path to this directory so that it can be successfully ran from the project's
// main directory. 
// This function returns a Result type that is an error if a file could not be opened or read from,
// and success otherwise. 
fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    let appended_path = format!("{}{}", TEST_APPEND, file_name);
    match File::open(appended_path) {
        Err(_) => {
            println!("Error: script generation failed because the file {} could not be opened", file_name);
            return Err(declarations::ERR_SCRIPT_GEN);
        },
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut s = String::new();
            loop {
                s.clear();
                match reader.read_line(&mut s) {
                    Err(_) => {
                        println!("Error: script generation failed because line could not be read");
                        return Err(declarations::ERR_SCRIPT_GEN);
                    },
                    Ok(bytes_read) => {
                        if bytes_read == 0 { //done reading
                            return Ok(())
                        }
                        file_lines.push(s.trim().to_string());
                    },
                }

            }
        },
    }
}

// This function processes a PlayConfig which contains character names and associated text files.
// It takes as paramaters a mutable declaration to a play and a reference to a play config. 
// For each tuple in the play config, it opens the character text file, calls grab trimmed lines, and puts
// each line along with the character name into the play while propagating errors it finds.
fn process_config(play: &mut declarations::Play, play_config: &PlayConfig) -> Result<(), u8> {
    for tup in play_config {
        match tup {
            (name, file) => {
                let mut lines: Vec<String> = Vec::new();
                grab_trimmed_file_lines(file, &mut lines)?; 
                for line in &lines {
                    add_script_line(play, line, name);
                }
            }
        }
    }
    Ok(())
}

// This function takes as parameters a string reference of a line containing a character name and a character file and a mutable reference to a PlayConfig.
// It splits this line into two separate tokens, and adds them as a tuple to the PlayConfig. If
// the tokens could not be properly extracted and whinge mode is on the program prints out a line
// complaining and otherwise just does not add the line. 
fn add_config(line: &String, play_config: &mut PlayConfig) {
    let delimited_tokens: Vec<&str> = line.split_whitespace().collect();
    if delimited_tokens.len() != CONFIG_LINE_TOKENS {
        use std::sync::atomic::Ordering;
        if declarations::WHINGE_ON.load(Ordering::SeqCst) {
            println!("Warning: There were not exactly two distinct tokens in the line {}", line);
        }
    } else {
        play_config.push((
                delimited_tokens[CHARACTER_NAME].to_string(), 
                delimited_tokens[CHARACTER_FILE].to_string()
                ));
    }
}

// This function takes a reference to a string that holds the name of the config file, a mutable
// reference to a string that will be filled with the play's title, and a mutable reference to a
// PlayConfig that will be filled with tuples of characters and their associated files. It goes
// line by line in the config file, adding each to the PlayConfig type. If there aren't enough
// lines in the config file to have a title and at least one character or if the files could not be
// propperly opend it returns an error and otherwise succeeds. 
fn read_config(config_file_name: &String, title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();
    grab_trimmed_file_lines(config_file_name, &mut lines)?;
    if lines.len() < MIN_CONFIG_LINES {
        println!("Error: the config file must contain at least one character and associated text file");
        return Err(declarations::ERR_SCRIPT_GEN);
    }
    for (i, line) in lines.iter().enumerate() {
        if i == TITLE_INDEX {
            *title = line.clone();
        } else {
            add_config(line, play_config);
        }
    }
    Ok(())
}

// This function takes a reference to a string containing the name of the config file, a mutable
// reference to a string that will contain the play title, and a mutable reference to a Play that
// will contain the play's lines. It uses the above functions to create and process the play
// config which fills the play with every line for each character while propagating errors. If
// everything suceeds, the play is full of lines. 
pub fn script_gen(config_file_name: &String, title: &mut String, play: &mut declarations::Play) -> Result<(), u8> {
    let mut play_config: PlayConfig = Default::default();
    read_config(config_file_name, title, &mut play_config)?;
    process_config(play, &play_config)?;
    Ok(())
}
