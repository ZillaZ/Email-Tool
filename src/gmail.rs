use crate::{*, hub_extension::HubExtension};

pub async fn init_hub(vars: &HashMap<String, String>) -> (Gmail<HttpsConnector<HttpConnector>>, String) {
    let secret = oauth2::read_application_secret(&vars["SECRET_PATH"]).await.unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();
    (Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth), vars["SAVED_TOKENS_PATH"].clone())
}

pub async fn get_ids(hub: &Gmail<HttpsConnector<HttpConnector>>, path: &str) -> Vec<Message> {
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
    .unwrap()
    .1
    .messages
    .unwrap()
}

pub async fn get_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str, path: &str) -> Message {
    let mut calee = hub.users().messages_get("me", id);
    calee.add_scope(Scope::Modify);
    let token = match calee.get_saved_token(path).await {
        Some(token) => token,
        None => calee.save_token(path).await
    };
    calee
    .doit(Some(token))
    .await
    .unwrap_or_default()
    .1
}