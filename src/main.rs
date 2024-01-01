use google_gmail1::{api::{Scope, Message, MessagePartHeader, MessagePart}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use std::{env, collections::HashMap};
use crate::email_utils::EmailUtils;

pub mod email_utils;

#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    let target_subject = &args[1];
    let vars = start_env();
    let hub = init_hub(&vars).await;
    let mut messages = get_ids(&hub).await;
    for message in messages.iter_mut() {
        let id = &message.clone().id;
        if id.is_none() {
            continue;
        }
        if message.get_subject(&hub).await == *target_subject {
            println!("Message: {}", message.get_message(&hub).await);
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