/*
 * ================== warning! ==================
 * The original code was written by Federico from
 * the Rustfully YouTube channel, and this is a -
 * small fork of Federico's code, called dig-file
 *
 *          (sorry if my english is bad)
 * ==============================================
 */

 /* warn: The code is still full of errors */
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use colored::*;

const VERSION: &str = "v0.1";

struct Config {
    pattern: String,
    files: Vec<String>,
    case_insensitive: bool,
    help_or_version: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        /*if args.len() < 3 {
            return Err(format!("Usage: {} pattern file (or files) [-i]", args[0]));
        }*/

        let mut case_insensitive = false;
        let mut non_flag_args: Vec<String> = Vec::new();
        let mut dont_panic: bool = false;
        let mut help_or_version: bool = false;

        for arg in &args[1..] {
            if arg == "-i" {
                case_insensitive = true;
            } else if arg == "-v" {
                dont_panic = true;
                help_or_version = true;
                println!("DigFile {}\nLicensed under Apache v2.0 Licese, ©2025 Salah Al-Refaai", VERSION)
            } else if arg == "-r" {
                todo!("cannot use regex right now, but its comming soon (if this project continued)")
            } else if arg == "-h" {
                dont_panic = true;
                help_or_version = true;
                println!("----------- Dig-File Help -----------");
                println!("digfile *args");
                println!("   -h: Help,   print this message.");
                println!("   -v: Ver.,   print the Version.");
                println!("   -i: Ignore, ignore char case.");
                println!("   -r: Regex,  search using Regex instead of normal String (todo!)");
                println!("----------- Dig-File Help -----------");
            } else {
                non_flag_args.push(arg.clone());
            }
        }

        if help_or_version {
            return Ok(Config {
                pattern: String::new(),
                files: vec![],
                case_insensitive,
                help_or_version,
            });
        }

        if args.len() < 3 && !dont_panic {
            return Err(format!("Usage: {} pattern file (or files) [-i]", args[0]));
        }

        // Should be fixed:
        // Triggered in the rare case the user types: cargo run -- -i -i
        if non_flag_args.is_empty() && !dont_panic {
            return Err("Error: Pattern not provided.".to_string());
        }
        
        let pattern;

        if !help_or_version { pattern = non_flag_args[0].clone();}
        else { pattern = String::from("") }
        if non_flag_args.len() < 2 && !dont_panic {
            return Err("Error: No input files provided.".to_string());
        }

        let files = non_flag_args[1..].to_vec();

        Ok(Config {
            pattern,
            files,
            case_insensitive,
            help_or_version,
        })
    }
}

fn grep(reader: &mut BufReader<File>, pattern: &str, case_insensitive: bool) {
    let mut line = String::new();
    let mut line_count: usize = 0;

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        line_count += 1;
        let _line_count = format!("{}", line_count).green();

        let mut matched = false;
        let mut processed_line = String::new();

        if case_insensitive {
            let pattern_lower = pattern.to_lowercase();
            let mut start = 0;
            let lower_line = line.to_lowercase();

            for (idx, _) in lower_line.match_indices(&pattern_lower) {
                matched = true;
                // Push uncolored part
                processed_line.push_str(&line[start..idx]);
                // Push colored matched part
                processed_line.push_str(&line[idx..idx + pattern.len()].red().to_string());
                //line.char_indices().collect::<Vec<_>>();
                start = idx + pattern.len();
            }

            // Push the remaining part of the line
            processed_line.push_str(&line[start..]);
        } else {
            if line.contains(pattern) {
                matched = true;
                processed_line = line.replace(pattern, &pattern.red().to_string());
            }
        }

        if matched {
            print!("{}: {}", _line_count, processed_line);
        }

        line.clear();
    }
}

fn grep_file(file: &str, config: &Config) {
    match File::open(file) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            grep(&mut reader, &config.pattern, config.case_insensitive);
        }
        Err(e) => {
            eprintln!("Could not open file: '{}': {}", file, e);
        }
    }
}

fn main() -> Result<(), String> { // the whole fًunction SALAH PATCHed
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;
    //let more_than_one_file: bool = false;
    let mut repeating: bool = false;

    if config.help_or_version {
        return Ok(()); // void
    }

    for file in &config.files {
        if repeating { println!("{} \"{}\"", "...done. now grepping:".cyan(), file) }
        else         { println!("{} \"{}\"", "grepping".cyan(), file) }
        grep_file(file, &config);

        if *&config.files.len() > 1 {
            repeating = true
        }
    }
    println!("{}", "\n...Done!".cyan());

    Ok(())
}

/* warn: The code is still full of errors */
