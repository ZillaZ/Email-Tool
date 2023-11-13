use google_gmail1::{api::Scope, Gmail, oauth2, hyper, hyper_rustls};

#[tokio::main]
async fn main() {
    let secret = oauth2::read_application_secret("C:/Users/lucas/Projects/email-tool/secret.json").await.unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::Interactive,
    ).build().await.unwrap();
    let hub = Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
    let messages = hub.users().messages_list("me").add_scope(Scope::Modify.as_ref().to_string()).doit().await.unwrap();
    for x in messages.1.messages.unwrap().iter() {
        let id = &x.id;
        if let Some(id) = id {
            println!("{}", id);
            let message = hub.users().messages_get("me", id).add_scope(Scope::Modify.as_ref().to_string()).doit().await.unwrap().1;
            println!("{:?}", String::from_utf8_lossy(message.payload.unwrap().body.unwrap().data.unwrap().as_slice())); 
        }
    }
}