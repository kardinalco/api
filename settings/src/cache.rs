use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cache {
    refresh: CacheRefresh,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheRefresh {
    permission: i64,
    google: i64,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            refresh: CacheRefresh {
                permission: 30,
                google: 3600,
            }
        }
    }
}

impl Cache {
    pub fn get_permission_ttl(&self) -> i64 {
        self.refresh.permission
    }
}
