use google_gmail1::{api::{Scope, Message, MessagePartHeader, MessagePart}, Gmail, oauth2, hyper, hyper_rustls::{self, HttpsConnector}};
use dotenvy::dotenv;
use hyper::client::HttpConnector;
use std::{env, collections::HashMap};

pub trait EmailUtils {
    fn get_subject(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> impl std::future::Future<Output = String> + Send;
    fn get_message(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> impl std::future::Future<Output = String> + Send;
}

impl EmailUtils for Message {
    async fn get_subject(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> String {
        if self.payload.is_none() {
            *self = get_message(hub, self.id.as_ref().unwrap()).await;
        }
        let headers = self.payload.clone().unwrap_or_default().headers.unwrap_or_default();
        let subject: Vec<&MessagePartHeader> = headers.iter().filter(|x| x.name.is_some() && x.name.as_ref().unwrap().as_str() == "Subject").collect();
        subject[0].value.clone().unwrap()
    }
    async fn get_message(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> String {
        if self.payload.is_none() {
            *self = get_message(hub, self.id.as_ref().unwrap()).await;
        }
        let headers = self.payload.clone().unwrap_or_default().parts.unwrap_or_default();
        let message : Vec<&MessagePart> = headers.iter()
        .filter(|x| x.headers.is_some() && x.headers.as_ref().unwrap().iter().map(|y| y.name.clone().unwrap()).collect::<Vec<String>>().contains(&"Content-Transfer-Encoding".to_string()))
        .collect();
        String::from_utf8(message[0].body.clone().unwrap().data.unwrap()).unwrap()
    }
}

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