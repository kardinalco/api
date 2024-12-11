use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub region: Region,
    pub info: Vec<BucketInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketInfo {
    pub name: String,
    pub user: String,
    pub key: String
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket {
            region: Region {
                name: "eu-west-3".to_string(),
                endpoint: "http://localhost:9000".to_string(),
            },
            info: vec![
                BucketInfo {
                    name: "user-profiles".to_string(),
                    user: Default::default(),
                    key: Default::default(),
                },
                BucketInfo {
                    name: "documents".to_string(),
                    user: Default::default(),
                    key: Default::default(),
                },
                BucketInfo {
                    name: "house-pictures".to_string(),
                    user: Default::default(),
                    key: Default::default(),
                },
                BucketInfo {
                    name: "expenses".to_string(),
                    user: Default::default(),
                    key: Default::default(),
                },
            ]
        }
    }
}