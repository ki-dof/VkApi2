mod error;
use reqwest::Client;
pub mod account;
pub mod groups;
pub mod users;

mod param_grid;

pub use param_grid::ParamGrid;

pub struct VkApi {
    service_key: String,
    group_key: String,
    flow_key: String,
    client: Client,
    v: f32,
}

impl VkApi {
    pub fn new(
        service_key: Option<String>,
        group_key: Option<String>,
        flow_key: Option<String>,
        v: Option<f32>,
    ) -> Self {
        let service_key = match service_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let group_key = match group_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let flow_key = match flow_key {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };
        let v = v.unwrap_or(5.131);

        if group_key == flow_key && flow_key == service_key && service_key.is_empty() {
            //Err(Error::new("Not valid email adress"))
        }

        Self {
            service_key,
            group_key,
            flow_key,
            client: Client::new(),
            v,
        }
    }

    pub fn get_group_key(self) -> Option<String> {
        if self.group_key.is_empty() {
            None
        } else {
            Some(self.group_key)
        }
    }

    pub fn get_service_key(self) -> Option<String> {
        if self.service_key.is_empty() {
            None
        } else {
            Some(self.service_key)
        }
    }

    pub fn get_flow_key(self) -> Option<String> {
        if self.flow_key.is_empty() {
            None
        } else {
            Some(self.flow_key)
        }
    }

    pub fn get_version(self) -> f32 {
        self.v
    }

    pub fn set_group_key(mut self, group_key: String) {
        self.group_key = group_key;
    }

    pub fn set_service_key(mut self, service_key: String) {
        self.service_key = service_key;
    }

    pub fn set_flow_key(mut self, flow_key: String) {
        self.flow_key = flow_key;
    }

    pub fn set_version(mut self, v: f32) {
        self.v = v;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
