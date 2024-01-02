#![feature(iter_advance_by)]

use google_gmail1::{api::{Scope, Message, MessagePartHeader, MessagePart}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use message_interpreter::*;
use std::{env, collections::HashMap};
use gmail::*;
use email_extension::*;
use std::sync::Arc;

pub mod email_extension;
pub mod message_interpreter;
pub mod gmail;
pub mod hub_extension;

#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    let target_subject = &args[1];
    let vars = start_env();
    let template = load_template(&vars).await;
    let beg = &vars["BEG"];
    let end = &vars["END"];
    let template_vars = init_template_vars(&template, beg, end);
    let answer_template = load_answer_template(&vars).await;
    let (hub, path) = init_hub(&vars).await;
    let mut messages = get_messages(&hub, &path).await;

    for message in messages.iter_mut() {
        if *message.get_subject(&hub, &path).await != *target_subject { continue; }

        let message = message.get_message(&hub, &path).await;
        let vals = get_values(&message, &template_vars);
        let mut answer = answer_template.clone();

        for pair in vals.iter() {
            answer = answer.replace(pair.0, pair.1);
        }

        println!("Final Message: {}", answer);
    }
}

fn start_env() -> HashMap<String, String> {
    let _ = dotenv().expect("bruh.");
    env::vars().collect::<HashMap<String, String>>()
}