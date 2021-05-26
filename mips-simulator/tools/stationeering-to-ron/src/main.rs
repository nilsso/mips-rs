#![feature(map_into_keys_values)]
use std::fs::File;
use std::io::Error as IoError;

use clap::{load_yaml, App};
use ron::Error as RonError;
use serde_json::Error as SerdeError;

use util::impl_from_error;

mod device;
use device::DeviceKinds;

#[derive(Debug)]
enum CliError {
    IoError(IoError),
    SerdeError(SerdeError),
    RonError(RonError),
}

impl_from_error!(CliError, IoError, SerdeError, RonError);

fn main() {
    if let Err(e) = cli() {
        println!("Error: {:?}", e);
    }
}

fn cli() -> Result<(), CliError> {
    let yaml = load_yaml!("./clap.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let in_file = File::open(matches.value_of("file").unwrap())?;
    let device_kinds: DeviceKinds = serde_json::de::from_reader(in_file)?;
    let device_kinds = device_kinds.0;
    if matches.is_present("pretty") {
        let config = ron::ser::PrettyConfig::new();
        println!("{}", ron::ser::to_string_pretty(&device_kinds, config)?);
    } else {
        println!("{}", ron::to_string(&device_kinds)?);
    }
    Ok(())
}
