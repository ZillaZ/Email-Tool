use google_gmail1::{api::{Scope, Message}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use std::{env, collections::HashMap};

#[tokio::main]
async fn main() {
    let vars = start_env();
    let hub = init_hub(&vars).await;
    let messages = get_ids(&hub).await;
    for message in messages.iter() {
        let id = &message.id;
        if id.is_none() {
            continue;
        }
        read_message(&hub, &id.as_ref().unwrap()).await;
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

async fn read_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str) {
    let message = hub.users()
    .messages_get("me", id)
    .add_scope(Scope::Modify)
    .doit()
    .await
    .unwrap()
    .1;
    println!("{:?}", String::from_utf8_lossy(
        message.
        payload
        .unwrap_or_default()
        .body
        .unwrap_or_default()
        .data
        .unwrap_or_default()
        .as_slice()
    )); 
}