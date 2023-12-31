
use crate::*;

#[async_trait]
pub trait EmailExtension {
    async fn get_subject(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> Arc<String>;
    async fn get_message(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> Arc<String>;
}

#[async_trait]
impl EmailExtension for Message {
    async fn get_subject(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> Arc<String> {
        if self.payload.is_none() {
            *self = get_message(hub, self.id.as_ref().unwrap()).await;
        }
        let headers = self.payload.clone().unwrap_or_default().headers.unwrap_or_default();
        let subject: Vec<&MessagePartHeader> = headers.iter().filter(|x| x.name.is_some() && x.name.as_ref().unwrap().as_str() == "Subject").collect();
        Arc::new(subject[0].value.clone().unwrap())
    }
    async fn get_message(&mut self, hub: &Gmail<HttpsConnector<HttpConnector>>) -> Arc<String> {
        if self.payload.is_none() {
            *self = get_message(hub, self.id.as_ref().unwrap()).await;
        }
        let headers = self.payload.clone().unwrap_or_default().parts.unwrap_or_default();
        let message : Vec<&MessagePart> = headers.iter()
        .filter(|x| x.headers.is_some() && x.headers.as_ref().unwrap().iter().map(|y| y.name.clone().unwrap()).collect::<Vec<String>>().contains(&"Content-Transfer-Encoding".to_string()))
        .collect();
        Arc::new(String::from_utf8(message[0].body.clone().unwrap().data.unwrap()).unwrap())
    }
}