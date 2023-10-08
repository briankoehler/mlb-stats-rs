use super::response::PeopleResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeopleRequest {
    #[serde(skip_serializing)]
    id: u32,
}

impl PeopleRequest {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub async fn send(&self) -> PeopleResponse {
        reqwest::get(format!(
            "https://statsapi.mlb.com/api/v1/people/{}",
            self.id
        ))
        .await
        .unwrap()
        .json::<PeopleResponse>()
        .await
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_test() {
        let request = PeopleRequest::new(571578);
        let response = request.send().await;
        println!("{:?}", response);
    }
}
