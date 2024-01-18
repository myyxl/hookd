use std::process::{self, Command};

use log::error;
use rocket::State;
use serde::{Deserialize, Serialize};

#[macro_use] extern crate rocket;

#[derive(Deserialize, Serialize)]
pub struct Hook {
    pub id: String,
    pub secret: String,
    pub execute: String
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub hooks: Vec<Hook>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hooks: vec![
                Hook {
                    id: String::from("blog"),
                    secret: String::from("b0b4d001-da79-4db8-bcaa-3130774abcea"),
                    execute: String::from("sleep 1")
                },
                Hook {
                    id: String::from("website"),
                    secret: String::from("af322f5c-4586-4ad8-80bc-23e132e85f9f"),
                    execute: String::from("sleep 1")
                }
            ]
        }
    }
}

pub fn load_config() -> Config {
    match confy::load_path("./hookd-config.toml") {
        Ok(config) => config,
        Err(error) => {
            error!("{}", error);
            process::exit(2);
        }
    }
}

#[get("/<secret>")]
fn hook(secret: &str, config: &State<Config>) -> &'static str {
    match config.hooks.iter().find(|&hook| hook.secret == secret) {
        None => return "404 NOT FOUND",
        Some(hook) => {
            let result = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/C", &hook.execute])
                    .spawn()
            } else {
                Command::new("sh")
                    .args(["-c", &hook.execute])
                    .spawn()
            };
            match result {
                Err(e) => {
                    error!("{}", e);
                    return "ERROR"
                },
                Ok(_) => {
                    return "OK"
                }
            }
        }
    }
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let config = load_config();

    rocket::build()
        .manage(config)
        .mount("/", routes![hook])
}
