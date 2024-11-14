use std::fs;
use inline_colorization::*;

// all import for web server 
use actix_cors::Cors;
use actix_web::{HttpServer,App};

use edda::api;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the welcome message from the file
    println!("{}", fs::read_to_string("./utils/ascii.art").unwrap());


    let port: u16 = 27997;
    let _ = HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new().wrap(cors).service(api::init::init_api())
    })
    .bind(("0.0.0.0",port))?
    .workers(1)
    .run().await;

    Ok(())
}



fn get_config_file() -> String {
    // read the config file passed as argument after --config
    let args: Vec<String> = std::env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        if arg == "--config" {
            return args[i + 1].clone();
        }
    }
    panic!("\nPlease provide a config file using --config flag");
}