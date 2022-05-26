extern crate clap;

use clap::{Arg, Command};

#[derive(Debug)]
pub struct CommandArgs  {
    pub filename: String,
    pub start_vertex: u32,
}

impl CommandArgs  {
    pub fn new() -> Self {
        // basic app information
        let app = Command::new("prim")
            .version("1.0")
            .about("Calculates MST using Prim Algo")
            .author("Marvin Mednick");

        // Define the name command line option
        let filename_option = Arg::new("file")
            .takes_value(true)
            .help("Input file name")
            .required(true);

        let starting_option = Arg::new("start")
            .takes_value(true)
            .help("Starting Vertex")
            .required(true);

        // now add in the argument we want to parse
        let mut app = app.arg(filename_option);
        app = app.arg(starting_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let filename = matches.value_of("file")
            .expect("Filename can't be None, we said it was required");

        let num_str = matches.value_of("start");

        let start = match num_str {
            None => { println!("Start is None..."); 0},
            Some(s) => {
                match s.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {println!("That's not a number! {}", s); 0},
                }
            }
        };

        println!("clap args: {} {}",filename, start);

        CommandArgs { filename: filename.to_string(), start_vertex : start}
    }   
}
