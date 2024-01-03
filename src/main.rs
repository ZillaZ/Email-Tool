#![feature(iter_advance_by)]

use google_gmail1::{api::{Scope, Message, MessagePartHeader, MessagePart}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use message_interpreter::*;
use std::{env, collections::HashMap};
use gmail::*;
use std::sync::Arc;

pub mod email_extension;
pub mod message_interpreter;
pub mod gmail;

#[tokio::main]
async fn main() {
    let vars = start_env();
    let hub = init_hub(&vars).await; 
    process_messages(&hub, &vars).await;
}

fn start_env() -> HashMap<String, String> {
    let _ = dotenv().expect("Error while initializing .env");
    env::vars().collect::<HashMap<String, String>>()
}

fn get_args() -> HashMap<String, String> {
    let args : Vec<String> = env::args().collect();
    let mut args_map = HashMap::<String, String>::new();
    if args.len() < 3 { panic!("Not enough arguments for execution."); }
    for i in (1..args.len()).filter(|x| x % 2 != 0) {
        let arg = args[i].to_owned();
        let value = args[i+1].to_owned();
        args_map.insert(arg, value);
    }
    if !args_map.contains_key("-s") { panic!("Missing argument: -s"); }
    args_map
}