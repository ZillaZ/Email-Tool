use google_gmail1::{api::{Scope, Message, MessagePartHeader, MessagePart}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use message_interpreter::*;
use std::{env, collections::HashMap};
use crate::email_utils::EmailUtils;

pub mod email_utils;
pub mod message_interpreter;

#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    let target_subject = &args[1];
    let vars = start_env();
    let template = load_template(&vars).await;
    let template_vars = init_template_vars(&template, '{', '}');
    let answer_template = load_answer_template(&vars).await;
    println!("{:?}", template_vars);
    let hub = init_hub(&vars).await;
    let mut messages = get_ids(&hub).await;
    
    for message in messages.iter_mut() {
        let id = &message.clone().id;
        if id.is_none() {
            continue;
        }
        if message.get_subject(&hub).await == *target_subject {
            let message = message.get_message(&hub).await;
            println!("Message: {}", &message);
            let vals = get_values(&message, &template_vars);
            println!("Values: {:?}", vals);
            let mut answer = answer_template.clone();
            for pair in vals.iter() {
                answer = answer.replace(pair.0, pair.1);
                println!("Temporary value: {}", answer);
            }
            println!("Final Message: {}", answer);
        }
    }
}

fn start_env() -> HashMap<String, String> {
    let _ = dotenv().expect("bruh.");
    env::vars().collect::<HashMap<String, String>>()
}

async fn init_hub(vars: &HashMap<String, String>) -> Gmail<HttpsConnector<HttpConnector>> {
    let secret = oauth2::read_application_secret(&vars["SECRET_PATH"]).await.unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth)
}

async fn get_ids(hub: &Gmail<HttpsConnector<HttpConnector>>) -> Vec<Message> {
    hub.users()
    .messages_list("me")
    .add_scope(Scope::Modify)
    .doit()
    .await
    .unwrap()
    .1
    .messages
    .unwrap()
}

async fn get_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str) -> Message {
    hub.users()
    .messages_get("me", id)
     .add_scope(Scope::Modify)
    .doit()
    .await
    .unwrap_or_default()
    .1
}