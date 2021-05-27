// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_parens)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IOError};
use std::path::Path;

use clap::{load_yaml, App, ArgMatches};
use regex::Regex;
use ron::{de::from_reader, Error as RonError};
use rustyline::error::ReadlineError;

use mips_parser::prelude::{Expr, MipsParserError, Node, Program};
use mips_simulator::prelude::{DeviceKind, ICSimulator, ICState, ICStateError, Line};
use util::impl_from_error;

type Editor = rustyline::Editor<()>;
type DeviceKinds = HashMap<String, DeviceKind>;

#[derive(Debug)]
enum CliError {
    ReadlineError(ReadlineError),
    MipsParserError(MipsParserError),
    IOError(IOError),
    RonError(RonError),
    ICStateError(ICStateError),
}

impl_from_error!(
    CliError,
    ReadlineError,
    MipsParserError,
    IOError,
    RonError,
    ICStateError
);

const WARN_DEVICE_KINDS: &'static str =
    "Warning: No device kinds file loaded, device interactions will fail
Download a Stationeers thing file from one of the following,
and compile it to Ron via the `stationeering-to-ron` tool:
    - https://data.stationeering.com/things/beta.json, or
    - https://data.stationeering.com/things/public.json";

const HELP_PROGRAM: &'static str = "    \"EOL\"             - finish
    \"help\" or \"h\"     - print this message again
    \"reinit\"          - reinitialize the simulation
    \"program\"         - display the program
    \"status\"          - display state details
    \"<n>\"             - step <n> times
    \"\"                - step once";

const HELP_DEVICE: &'static str = "    \"EOL\"             - finish
    \"help\" or \"h\"     - print this message again
    \"add <id> <kind>\" - assign device of <kind> to register <id>
    \"add n    <kind>\" - assign device of <kind> to the network
    \"rm  <id>\"        - remove device at register <id>
    \"list\"            - list device kinds
    \"status\"          - display currently set devices";

fn main() -> Result<(), CliError> {
    let mut rl = Editor::new();
    // TODO: better location for history.txt (temp directory)
    rl.load_history("history.txt").ok();
    let yaml = load_yaml!("./clap.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // Deserialize device kinds
    let kinds_path = Path::new(matches.value_of("kind-file").unwrap());
    let kinds: DeviceKinds = if kinds_path.exists() {
        let file = File::open(kinds_path)?;
        from_reader(file)?
    } else {
        println!("{}", WARN_DEVICE_KINDS);
        HashMap::new()
    };

    // Get program
    let program = get_program(&matches, &mut rl)?;

    let mut state = ICState::default();

    // Configure devices
    configure_devices(&mut state, &kinds, matches.value_of("device-conf"), &mut rl)?;

    let sim = ICSimulator::new(state, program);

    // Run the simulation
    run_program(sim, &mut rl)?;

    rl.save_history("history.txt").unwrap();
    Ok(())
}

// Get program, from file or from standard input.
fn get_program(matches: &ArgMatches, rl: &mut Editor) -> Result<Program, CliError> {
    let program = if let Some(path) = matches.value_of("file") {
        Program::try_from_file(path)?
    } else {
        // Try to build the program
        println!("Build program from stdin...");
        let mut program = Program::new();
        loop {
            let source = rl.readline(">> ");
            match source {
                Ok(source) => {
                    match Expr::try_from_str(&source) {
                        Ok(expr) => program.push(expr),
                        _ => println!("Error: Parse error"),
                    };
                }
                Err(ReadlineError::Eof) => break,
                Err(e) => Err(e)?,
            }
        }
        program
    };
    Ok(program)
}

// Configure state devices, either via a configuration file or through standard input.
fn configure_devices<'dk>(
    state: &mut ICState<'dk>,
    kinds: &'dk DeviceKinds,
    conf: Option<&str>,
    rl: &mut Editor,
) -> Result<(), CliError> {
    if let Some(conf) = conf {
        let n_pattern = Regex::new(r"\s*n\s*(\d+)\s*(\w+)").unwrap();
        let i_pattern = Regex::new(r"\s*(\d+)\s*(\w+)").unwrap();

        let file = File::open(conf)?;
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if let Some(groups) = n_pattern.captures(&line) {
                let n = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let key = groups.get(2).unwrap().as_str();
                if let Some(kind) = kinds.get(key) {
                    for _ in 0..n {
                        let dev = kind.make();
                        state.dev_network_add(dev);
                    }
                } else {
                    println!("Error: device kind {} not-found, skipping", key);
                }
            } else if let Some(groups) = i_pattern.captures(&line) {
                let i = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let key = groups.get(2).unwrap().as_str();
                if let Some(kind) = kinds.get(key) {
                    let dev = kind.make();
                    state.set_dev(i, Some(dev))?;
                } else {
                    println!("Error: device kind {} not-found, skipping", key);
                }
            } else {
                println!("Error: failed to parse line {}", i);
            }
        }
    } else {
        println!("Configure devices:\n{}", HELP_DEVICE);
        let add_i_pattern = Regex::new(r"add\s+(\d+)\s+(\w+)").unwrap();
        let add_n_pattern = Regex::new(r"add\s+n\s+(\w+)").unwrap();
        let rm_pattern = Regex::new(r"rm\s+(\d+)").unwrap();
        loop {
            match rl.readline(">> ") {
                Ok(line) => {
                    let line = line.trim();
                    if line.contains("help") {
                        println!("{}", HELP_DEVICE);
                        rl.add_history_entry(line);
                    } else if line.contains("list") {
                        // List device kinds
                        for kind in kinds.keys() {
                            println!("{}", kind);
                        }
                        rl.add_history_entry(line);
                    } else if line.contains("status") {
                        println!("{}", state);
                        rl.add_history_entry(line);
                    } else if let Some(groups) = add_i_pattern.captures(line) {
                        // Add device
                        let i = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let key = groups.get(2).unwrap().as_str();
                        if let Some(kind) = kinds.get(key) {
                            let dev = kind.make();
                            state.set_dev(i, Some(dev))?;
                        } else {
                            println!("Error: device kind {} not-found, skipping", key);
                        }
                        rl.add_history_entry(line);
                    } else if let Some(groups) = add_n_pattern.captures(line) {
                        // Add device to network
                        let key = groups.get(1).unwrap().as_str();
                        if let Some(kind) = kinds.get(key) {
                            let dev = kind.make();
                            state.dev_network_add(dev);
                        } else {
                            println!("Error: device kind {} not-found, skipping", key);
                        }
                        rl.add_history_entry(line);
                    } else if let Some(groups) = rm_pattern.captures(line) {
                        // Remove device
                        let i = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        state.set_dev(i, None)?;
                        rl.add_history_entry(line);
                    } else {
                        println!("Error: unknown command");
                    }
                }
                Err(ReadlineError::Eof) => return Ok(()),
                Err(e) => Err(e)?,
            }
        }
    }
    Ok(())
}

fn run_program(sim_init: ICSimulator, rl: &mut Editor) -> Result<(), ReadlineError> {
    let mut sim = sim_init.clone();
    let mut i = 1_usize;

    println!(
        "Running simulation:\n{}\n0: {}",
        HELP_PROGRAM,
        format_next_line(&sim)
    );

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    step(&mut i, &mut sim);
                } else if line == "reinit" {
                    sim = sim_init.clone();
                    i = 1_usize;
                    println!("0: {}", format_next_line(&sim));
                    rl.add_history_entry(line);
                } else if line == "help" || line == "h" {
                    println!("{}", HELP_PROGRAM);
                    rl.add_history_entry(line);
                } else if line == "program" {
                    for line in sim.iter_lines() {
                        println!("{}", line);
                    }
                    rl.add_history_entry(line);
                } else if line == "status" {
                    println!("{}", sim.state);
                    rl.add_history_entry(line);
                } else if let Ok(n) = line.parse::<usize>() {
                    for _ in 0..n {
                        step(&mut i, &mut sim);
                    }
                    rl.add_history_entry(line);
                } else {
                    println!("Error: unknown command");
                }
            }
            Err(ReadlineError::Eof) => return Ok(()),
            Err(e) => Err(e)?,
        }
    }
}

fn format_next_line(sim: &ICSimulator) -> String {
    sim.next_line().map(Line::to_string).unwrap_or("END".into())
}

fn step(i: &mut usize, sim: &mut ICSimulator) {
    let l1 = format_next_line(&sim);
    sim.step().ok();
    let l2 = format_next_line(&sim);
    println!("{}: {} -> {} ", i, l1, l2);
    *i += 1;
}
