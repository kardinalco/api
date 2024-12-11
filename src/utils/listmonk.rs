use serde::{Deserialize, Serialize};
use serde_json::{json};
use crate::exceptions::error::Error;

pub struct Client {
    client: reqwest::Client,
    api_host: String,
    api_user: String,
    api_key: String,
}

#[derive(Deserialize)]
pub struct CreateSubscriber {
    pub data: CreateSubscriberData
}

#[derive(Deserialize)]
pub struct CreateSubscriberData {
    pub id: i32,
}

impl Client {
    pub fn new(api_host: String, api_user: String, api_key: String) -> Self {
        Client { 
            api_user,
            api_host,
            api_key,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn send_transactional_mail<T: Serialize>(&self, id: i32, template_id: i32, data: &T) -> Result<(), Error> {
        self.client.post(&format!("{}/tx", self.api_host))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("token {}:{}", self.api_user, self.api_key))
            .json(&json!({
                "subscriber_id": id,
                "template_id": template_id,
                "data": data,
            }))
            .send()
            .await
            .map_err(|_| Error::InternalServer("ListMonk internal error !".to_string()))?;
        Ok(())
    }
    
    pub async fn send_transactional_mail_with_email<T: Serialize>(&self, email: &str, template_id: i32, data: &T) -> Result<(), Error> {
        self.client.post(&format!("{}/tx", self.api_host))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("token {}:{}", self.api_user, self.api_key))
            .json(&json!({
                "subscriber_email": email,
                "template_id": template_id,
                "data": data,
            }))
            .send()
            .await
            .map_err(|_| Error::InternalServer("ListMonk internal error !".to_string()))?;
        Ok(())
    }
    
    pub async fn create_subscriber(&self, email: &str, name: &str) -> Result<CreateSubscriber, Error> {
        Ok(self.client.post(&format!("{}/subscribers", self.api_host))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("token {}:{}", self.api_user, self.api_key))
            .json(&json!({
                "email": email,
                "name": name,
                "status": "enabled",
            }))
            .send()
            .await
            .map_err(|_| Error::InternalServer("ListMonk internal error !".to_string()))?
            .json::<CreateSubscriber>()
            .await
            .map_err(|e| {
                println!("{:?}", e);
                Error::InternalServer("ListMonk internal error !".to_string())
            })?)
    }
}