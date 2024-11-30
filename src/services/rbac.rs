use crate::domain::role::RoleDomain;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use casbin::{Adapter, Filter, Model};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub sub: String,
    pub obj: String,
    pub act: String,
}

pub struct RbacAdapter {
    db: Arc<DatabaseConnection>,
    redis: Arc<Pool<RedisConnectionManager>>,
    is_filtered: bool,
}

impl RbacAdapter {
    pub fn new(db: Arc<DatabaseConnection>, redis: Arc<Pool<RedisConnectionManager>>) -> Self {
        Self {
            db,
            redis,
            is_filtered: false,
        }
    }

    pub async fn load_policies_from_db(
        &mut self,
    ) -> Vec<(entity::role::Model, Vec<entity::permission::Model>)> {
        RoleDomain::get_roles_and_permissions(&self.db)
            .await
            .unwrap_or(vec![])
    }

    pub async fn load_policies_from_cache(&mut self) -> Vec<Policy> {
        todo!()
    }

    pub async fn save_to_cache(&mut self) {}
}

#[async_trait]
impl Adapter for RbacAdapter {
    async fn load_policy(&mut self, m: &mut dyn Model) -> casbin::Result<()> {
        self.load_policies_from_db()
            .await
            .iter()
            .for_each(|(role, permissions)| {
                let role_name = role.name.clone();
                permissions.iter().for_each(|permission| {
                    println!(
                        "{} {} {}",
                        &role_name, &permission.resource, &permission.action
                    );
                    m.add_policy(
                        "",
                        "p",
                        vec![
                            "user".to_string(),
                            permission.resource.clone(),
                            permission.action.clone(),
                        ],
                    );
                    m.add_policy(
                        &role_name,
                        &permission.resource,
                        vec![permission.action.clone()],
                    );
                });
            });
        Ok(())
    }

    async fn load_filtered_policy<'a>(
        &mut self,
        m: &mut dyn Model,
        _f: Filter<'a>,
    ) -> casbin::Result<()> {
        println!("def");
        self.load_policies_from_db()
            .await
            .iter()
            .for_each(|(role, permissions)| {
                let role_name = role.name.clone();
                permissions.iter().for_each(|permission| {
                    m.add_policy(
                        &role_name,
                        &permission.resource,
                        vec![permission.action.clone()],
                    );
                });
            });
        todo!()
    }

    async fn save_policy(&mut self, _m: &mut dyn Model) -> casbin::Result<()> {
        println!("save policy");
        todo!()
    }

    async fn clear_policy(&mut self) -> casbin::Result<()> {
        println!("clear policy");
        //let a = self.redis.get().await.map_err(|e| casbin::Error::AdapterError(e.to_string()))?;
        //del("casbin_policy").await?;
        Ok(())
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    async fn add_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rule: Vec<String>,
    ) -> casbin::Result<bool> {
        println!("add policy");
        todo!()
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rules: Vec<Vec<String>>,
    ) -> casbin::Result<bool> {
        println!("add policy");
        todo!()
    }

    async fn remove_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rule: Vec<String>,
    ) -> casbin::Result<bool> {
        println!("add policy");
        todo!()
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rules: Vec<Vec<String>>,
    ) -> casbin::Result<bool> {
        println!("add policy");
        todo!()
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _field_index: usize,
        _field_values: Vec<String>,
    ) -> casbin::Result<bool> {
        println!("add policy");
        todo!()
    }
}
