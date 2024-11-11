use crate::{error::VkApiError, error::VkError, param_grid::ParamGrid, VkApi};
const API: &str = "https://api.vk.com/method/users.";


pub async fn check_token(api: &VkApi, params: Option<ParamGrid>) -> Result<u8, VkApiError> {
    let mut params = params.unwrap_or_default();

    params.insert_if_not_exists("v", api.v);

    let response = api
        .client
        .post(format!("{}checkToken", API))
        .header("Authorization", format!("Bearer {}", api.flow_key))
        .form(&params.data)
        .send()
        .await?;

    if let Ok(error) = response.json::<VkError>().await {
        return Err(VkApiError::VkError(error));
    };

    Ok(1)
}