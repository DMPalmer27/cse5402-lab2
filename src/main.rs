pub mod lab1;

use std::env;
use lab1::declarations;

// This function is called whenver the program is ran with improper command line arguments and it
// prints a message telling the user how to run the program
fn usage(name: &String) {
    println!("Usage: ./{name} <configuration_file_name> [whinge]");
}

// This function is used to parse the command line arguments. It takes one parameter, a mutable
// reference to a string in which it places the name of the file provided as the first command line
// argument. It also sets the whinge mode flag if "whinge" was provided as the second command line
// argument. If the program was ran improperly it calls the usage function and returns an error.
fn parse_args(name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();
    for arg in env::args() {
        args.push(arg);
    }
    
    //Check if valid input
    if args.len() < declarations::MIN_ARGS  || 
    args.len() > declarations::MAX_ARGS || 
    (args.len() == declarations::MAX_ARGS && args[declarations::WHINGE_MODE] != "whinge".to_string()){

        usage(&args[declarations::PROG_NAME]);
        return Err(declarations::ERR_CMD_LINE);
    }

    *name = args[declarations::CONFIG_FILE].clone(); 
    
    if args.len() == declarations::MAX_ARGS {
        use std::sync::atomic::Ordering;
        declarations::WHINGE_ON.store(true, Ordering::SeqCst); 
    }
    Ok(())
}


// This function prints all of the lines in a play, starting from the title including proper
// spacing when characters begin speaking
fn recite(title: &String, play: &declarations::Play) {
    println!("Play: {}", title);
    if play.len() == 0 {
        println!("Play does not contain any valid lines");
    } else {
        let mut cur_speaker: &str = "";
        for line in play {
            match line {
                (_num, name, text) => {
                    if cur_speaker != name {
                        println!();
                        println!("{}.", name);
                        cur_speaker = name;
                    }
                    println!("{}", text);
                }
            }
        }
    }

}

// The main function executes the program which includes retrieving command line arguments,
// constructing the play, and printing the play.  
fn main() -> Result<(), u8>  {
    let mut config_file: String = Default::default();

    if let Err(e) = parse_args(&mut config_file){
        return Err(e);
    }
    
    let mut play_title: String = Default::default();
    let mut play: declarations::Play = Default::default();

    if let Err(e) = lab1::script_gen::script_gen(&config_file, &mut play_title, &mut play) {
        return Err(e);
    }

    play.sort();
    recite(&play_title, &play);
    Ok(())
}
