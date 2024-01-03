use crate::*;

pub async fn init_hub(vars: &HashMap<String, String>) -> Gmail<HttpsConnector<HttpConnector>> {
    let secret = oauth2::read_application_secret(&vars["SECRET_PATH"]).await.unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).persist_tokens_to_disk("tokenscache.json")
    .build()
    .await
    .unwrap();
    Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth)
}

pub async fn get_ids(hub: &Gmail<HttpsConnector<HttpConnector>>) -> Vec<Message> {
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

pub async fn get_message(hub: &Gmail<HttpsConnector<HttpConnector>>, id: &str) -> Message {
    hub.users()
    .messages_get("me", id)
    .add_scope(Scope::Modify)
    .doit()
    .await
    .unwrap()
    .1
}