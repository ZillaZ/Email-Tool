use google_gmail1::{Error, api::ListMessagesResponse};

use crate::{*, hub_extension::HubExtension};

pub async fn init_hub(vars: &HashMap<String, String>) -> (Gmail<HttpsConnector<HttpConnector>>, String) {
    let secret = oauth2::read_application_secret(&vars["SECRET_PATH"]).await.unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    (Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth), vars["SAVED_TOKENS_PATH"].clone())
}

pub async fn get_messages(hub: &Gmail<HttpsConnector<HttpConnector>>, path: &str) -> Vec<Message> {
    match get_ids(hub, path).await {
        Ok(val) => return val.1.messages.unwrap(),
        Err(_e) => {
            update_list_token(hub, path).await;
            return get_ids(hub, path).await.unwrap().1.messages.unwrap();
        }
    }
}

async fn get_ids(hub: &Gmail<HttpsConnector<HttpConnector>>, path: &str) -> Result<(hyper::Response<hyper::Body>, ListMessagesResponse), Error> {
    let mut calee = hub.users()
    .messages_list("me");
    calee.add_scope(Scope::Modify);
    let token = match calee.get_saved_token(&path).await {
        Some(token) => token,
        None => calee.save_token(&path).await
    };
    calee
    .doit(Some(token))
    .await
}

pub async fn read_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str, path: &str) -> Message {
    match get_message(hub, id, path).await {
        Ok(val) => return val.1,
        Err(_e) => {
            update_get_token(hub, id, path).await;
            return get_message(hub, id, path).await.unwrap().1;
        }
    }
}
    
    async fn get_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str, path: &str) -> Result<(hyper::Response<hyper::Body>, Message), Error> {
    let mut calee = hub.users().messages_get("me", id);
    calee.add_scope(Scope::Modify);
    let token = match calee.get_saved_token(path).await {
        Some(token) => token,
        None => calee.save_token(path).await
    };
    calee
    .doit(Some(token))
    .await
}

async fn update_get_token(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str, path: &str) {
    hub.users().messages_get(id, path).add_scope(Scope::Modify).save_token(path).await;
}

async fn update_list_token(hub: &Gmail<HttpsConnector<HttpConnector>>, path: &str) {
    hub.users().messages_list("me").add_scope(Scope::Modify).save_token(path).await;
}