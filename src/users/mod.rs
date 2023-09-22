use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, VkApi};
use serde::Deserialize;
use serde_json::Value;

const API: &str = "https://api.vk.com/method/users.";

pub struct Token(String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Response {
    pub response: Vec<Profile>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub can_access_closed: bool,
    pub is_closed: bool,
}
pub async fn get(api: &VkApi, params: Option<ParamGrid>) -> Result<Vec<Profile>, VkApiError> {
    let mut params = match params {
        Some(params) => params,
        None => ParamGrid::new(),
    };

    params.insert_if_not_exists("v", api.v);

    let response = api
        .client
        .post(format!("{}get", API))
        .header("Authorization", format!("Bearer {}", api.flow_key))
        .form(&params.data)
        .send()
        .await?;

    let response_text = response.text().await.unwrap();

    return if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        Err(VkApiError::VkError(error))
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: Vec<Profile> = serde_json::from_value(json["response"].clone())?;
        Ok(data)
    }
}
