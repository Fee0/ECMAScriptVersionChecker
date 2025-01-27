extern crate js_version_checker_lib;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use clap::{command, Arg, ColorChoice};
use log::error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let match_result = command!()
        .version("1")
        .about("Checks the ECMAScript version of a Javascript file")
        .arg(Arg::new("path").help("path to js file"))
        .color(ColorChoice::Always)
        .get_matches();

    if let Some(path) = match_result.get_one::<String>("path") {
        if let Ok(mut f) = File::open(path) {
            let mut code = String::new();
            f.read_to_string(&mut code)?;

            let version = js_version_checker_lib::get_min_ecma_version(&code)?;
            println!("min version: {:?}", version);

            let features = js_version_checker_lib::get_ecma_features(&code)?;
            println!("language features: {:?}", features);
        } else {
            error!("Failed to open file: {}", path);
        }
    }

    Ok(())
}
