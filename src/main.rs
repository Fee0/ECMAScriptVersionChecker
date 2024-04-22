extern crate js_version_checker_lib;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let js_code = r#"
        async function f() {}
        x?.y
    "#;

    let version = js_version_checker_lib::get_min_ecma_version(js_code)?;
    println!("min version: {:?}", version);

    let features = js_version_checker_lib::get_ecma_features(js_code)?;
    println!("language features: {:?}", features);

    Ok(())
}
