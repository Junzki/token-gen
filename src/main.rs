extern crate argparse;
extern crate jsonwebtoken as jwt;
extern crate yaml_rust;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use argparse::{ArgumentParser, Store};
use jwt::{encode, Header};
use yaml_rust::YamlLoader;

mod models;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut spec_from: String = String::new();
    {
        let mut parser = ArgumentParser::new();
        
        parser.set_description("JSON Web Token (JWT) generator.");
        parser.refer(&mut spec_from).add_argument("spec_from", Store, "Path of JWT spec file (toml).");

        parser.parse_args_or_exit();
    }

    println!("Load spec file from `{}`", spec_from);
    let mut f = File::open(spec_from)?;
    let mut spec: String = String::new();

    f.read_to_string(&mut spec)?;

    // let spec = YamlLoader::load_from_str(&spec).unwrap();
    // println!("{:?}", &(spec[0]));

    // let token = encode(&Header::default(), &(spec.payload.unwrap()), spec.generic.secret.as_ref());
    // println!("Bearer {}", token.unwrap());
    let config = models::config_from_string(&spec)?;
    println!("{:?}", &config.default_context);

    Ok(())
}
