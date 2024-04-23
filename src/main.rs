extern crate js_version_checker_lib;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use clap::{Arg, ColorChoice, command};

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let match_result = command!()
        .version("1")
        .about("check js version")
        .styles(get_styles())
        .arg(
            Arg::new("path")
                .help("path to js file")
        ).color(ColorChoice::Always).get_matches();


    if let Some(path) = match_result.get_one::<String>("path") {
        if let Ok(mut f) = File::open(path) {
            let mut code = String::new();
            f.read_to_string(&mut code)?;

            let version = js_version_checker_lib::get_min_ecma_version(&code)?;
            println!("min version: {:?}", version);

            let features = js_version_checker_lib::get_ecma_features(&code)?;
            println!("language features: {:?}", features);
        } else {
            println!("Failed to open File")
        }
    }

    Ok(())
}
