use crate::*;

pub trait HubExtension {
    fn get_saved_token(&self, path: &str) -> impl std::future::Future<Output = Option<String>>;
    fn save_token(&self, path: &str) -> impl std::future::Future<Output = String>;
}

impl HubExtension for google_gmail1::api::UserMessageListCall<'_, HttpsConnector<HttpConnector>> {
    async fn get_saved_token(&self, path: &str) -> Option<String> {
        let token = tokio::fs::read(path).await.unwrap();
        if token.len() > 0 {
            return Some(String::from_utf8(token).unwrap())
        }
        None
    }

    async fn save_token(&self, path: &str) -> String {
        let token = self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await.unwrap().unwrap();
        tokio::fs::write(path, token.as_bytes()).await.unwrap();
        token
    }
}

impl HubExtension for google_gmail1::api::UserMessageGetCall<'_, HttpsConnector<HttpConnector>> {
    async fn get_saved_token(&self, path: &str) -> Option<String> {
        let token = tokio::fs::read(path).await.unwrap();
        if token.len() > 0 {
            return Some(String::from_utf8(token).unwrap())
        }
        None
    }
    async fn save_token(&self, path: &str) -> String {
        let token = self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await.unwrap().unwrap();
        tokio::fs::write(path, token.as_bytes()).await.unwrap();
        token
    }
}