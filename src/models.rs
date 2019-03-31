use std::error::Error;
use yaml_rust::{Yaml, YamlLoader};


#[derive(Serialize, Deserialize)]
pub struct Fields {
    pub uid: String,
    pub scopes: Vec<String>
}

pub struct Context {
    pub name: String,
    pub algorithm: String,
    pub secret: String,
    pub fields: Option<Fields>
}

pub struct Config {
    pub contexts: Vec<Context>,
    pub default_context: String
}

impl Default for Config {
    fn default() -> Config {
        Config {
            default_context: String::new(),
            contexts: Vec::new()
        }
    }
}

impl Default for Context {
    fn default() -> Context {
        Context {
            name: String::new(),
            algorithm: String::from("HS256"),
            secret: String::new(),
            fields: None
        }
    }
}

pub fn config_from_string(config_str: &String) -> Result<Config, Box<dyn Error>> {
    let spec = YamlLoader::load_from_str(config_str)?;
    let spec = &spec[0];

    let mut config = Config { ..Default::default() };

    let default_context = spec["default_context"].as_str().unwrap();
    config.default_context = default_context.to_string();

    let contexts = spec["contexts"].as_vec().unwrap();
    for original in contexts {
        let mut cxt = Context { ..Default::default() };
        cxt.name = original["name"].as_str().unwrap().to_string();

        let algorithm = original["algorithm"].as_str().unwrap().to_string();
    }


    Ok(config)
}