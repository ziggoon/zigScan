use crate::util;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn description() {
    let desc = r#"
                [+]     welcome to zigScan
                [+]   nmap visualization tool 
                [+]     built by @ziggoon
                [+]      cc @secdaemons
"#;
    println!("{}", desc);
}

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

fn main_help() {
    println!("  help              this menu lol");
    println!("  scan              scan IP or CIDR");
    println!("  quit              exits the program");
}

fn scan(ip: Vec<String>) {
    let ip = ip[1].as_str();
    println!("[!] STARTING SCANS FOR: {:?}", ip);
    util::lib::basic_tcp(&ip);
    util::lib::full_tcp(&ip);
    util::lib::intense_scan(&ip);
    util::lib::basic_udp(&ip);
}

pub fn main_loop() {
    util::lib::banner();
    description();
    // initialize user_input to a mutable String
    let mut user_input: Vec<String>;
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("zigScan(): ");
        match readline {
            Ok(line) => {
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "scan" => scan(user_input),
                    "help" => main_help(),
                    "quit" => std::process::exit(0),
                    _ => continue,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                break;
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
}
