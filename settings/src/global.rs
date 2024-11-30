use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Global {
    default_role: String,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            default_role: String::from("User"),
        }
    }
}

impl Global {
    pub fn get_default_role_name(&self) -> &str {
        &self.default_role
    }
}
