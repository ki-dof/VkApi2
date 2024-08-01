use std::collections::HashMap;

use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, VkApi};
use serde::Deserialize;
use serde_json::Value;

const API: &str = "https://api.vk.com/method/groups.";

pub struct Token(String);

#[derive(Debug, Deserialize)]
pub struct ResponseInfo {
    #[serde(alias = "2fa_required")]
    pub twofa_required: u8,
    pub country: String,
    pub https_required: u8,
    pub intro: u8,
    pub community_comments: bool,
    pub link_redirects: HashMap<String, String>,
    pub lang: u8,
    pub no_wall_replies: u8,
    pub own_posts_default: u8,
    pub vk_pay_endpoint_v2: String,
    pub messages_translation_language_pairs: Vec<String>,
    pub obscene_text_filter: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResponseBanned {
    pub count: u32,
    pub items: Vec<Inner>,
}

#[derive(Debug, Deserialize)]
pub struct Inner {
    pub ban_info: BanInfo,
    pub profile: Option<Profile>,
    pub group: Option<Group>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct BanInfo {
    pub admin_id: u32,
    pub comment: String,
    pub comment_visible: bool,
    pub date: i64,
    pub reason: i64,
    pub end_date: i64,
}
#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub can_access_closed: bool,
    pub is_closed: bool,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub screen_name: String,
    pub is_closed: u32,
    pub r#type: String,
    pub photo_100: String,
}

pub async fn ban(api: &VkApi, params: Option<ParamGrid>) -> Result<u8, VkApiError> {
    let mut params = params.unwrap_or_else(ParamGrid::new);

    params.insert_if_not_exists("v", api.v);

    let response = api
        .client
        .post(format!("{}ban", API))
        .header("Authorization", format!("Bearer {}", api.flow_key))
        .form(&params.data)
        .send()
        .await?;

    if let Ok(error) = response.json::<VkError>().await {
        return Err(VkApiError::VkError(error));
    };

    Ok(1)
}

pub async fn unban(api: &VkApi, params: Option<ParamGrid>) -> Result<u8, VkApiError> {
    let mut params = match params {
        Some(params) => params,
        None => ParamGrid::new(),
    };

    params.insert_if_not_exists("v", api.v);

    let response = api
        .client
        .post(format!("{}unban", API))
        .header("Authorization", format!("Bearer {}", api.flow_key))
        .form(&params.data)
        .send()
        .await?;

    if let Ok(error) = response.json::<VkError>().await {
        return Err(VkApiError::VkError(error));
    };

    Ok(1)
}

pub async fn get_banned(
    api: &VkApi,
    params: Option<ParamGrid>,
) -> Result<ResponseBanned, VkApiError> {
    let mut params = match params {
        Some(params) => params,
        None => ParamGrid::new(),
    };

    params.insert_if_not_exists("v", api.v);

    let response = api
        .client
        .post(format!("{}getBanned", API))
        .header("Authorization", format!("Bearer {}", api.flow_key))
        .form(&params.data)
        .send()
        .await?;

    let response_text = response.text().await.unwrap();
    if let Ok(error) = serde_json::from_str::<VkError>(&response_text) {
        return Err(VkApiError::VkError(error));
    } else {
        let json: Value = serde_json::from_str(&response_text)?;
        let data: ResponseBanned = serde_json::from_value(json["response"].clone())?;
        return Ok(data);
    }
}
