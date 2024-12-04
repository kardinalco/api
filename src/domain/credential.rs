use entity::credentials::ActiveModel;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::JoinType::LeftJoin;
use tracing::instrument;
use crate::api::credentials::request::{CredentialCreate, CredentialUpdate};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;

pub struct CredentialDomain;

type Credential = entity::credentials::Model;

impl CredentialDomain {
    #[instrument(skip(db))]
    pub async fn list_credentials(db: &DatabaseConnection, house_id: &str) -> Result<Vec<Credential>, Error> {
        Ok(entity::credentials::Entity::find()
            .filter(entity::credentials_house::Column::HouseId.eq(house_id))
            .join(LeftJoin, entity::credentials::Relation::CredentialsHouse.def())
            .all(db)
            .await?.iter_mut().map(|x| { 
                x.password = Self::decode_passwords(&x.password);
                x.to_owned() 
            }
        ).collect::<Vec<Credential>>())
    }

    #[instrument(skip(db))]
    pub async fn get_credentials(db: &DatabaseConnection, credential_id: &str) -> Result<Credential, Error> {
        let mut entity = entity::credentials::Entity::find()
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("Credential", credential_id.to_string())))?;
        entity.password = Self::decode_passwords(&entity.password);
        Ok(entity)
    }

    #[instrument(skip(db))]
    pub async fn create_credentials(db: &DatabaseConnection, created_by: &str, body: CredentialCreate) -> Result<Credential, Error> {
        let credential = body.clone().into_model(created_by).insert(db).await?;
        entity::credentials_house::Entity::insert(entity::credentials_house::ActiveModel {
            credentials_id: Set(credential.id.clone()),
            house_id: Set(body.house_id.clone()),
            created_by: Set(created_by.to_string()),
            ..Default::default()
        }).exec(db).await?;
        Ok(credential)
    }

    #[instrument(skip(db))]
    pub async fn update_credentials(db: &DatabaseConnection, updated_by: &str, credential_id: &str, body: CredentialUpdate) -> Result<Credential, Error> {
        Ok(entity::credentials::Entity::update(body.into_model(updated_by))
                .filter(entity::credentials::Column::Id.eq(credential_id))
                .exec(db)
                .await?)
    }

    #[instrument(skip(db))]
    pub async fn delete_credentials(db: &DatabaseConnection, deleted_by: &str, credential_id: &str) -> Result<Credential, Error> {
        Ok(entity::credentials::Entity::update(ActiveModel {
            deleted_by: Set(Some(deleted_by.to_string())),
            deleted_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default() })
               .filter(entity::credentials::Column::Id.eq(credential_id))
               .exec(db)
               .await?)
    }
    
    fn decode_passwords(password: &Vec<String>) -> Vec<String> {
        password.to_owned()
    }
    
    fn encode_passwords(password: &Vec<String>) -> Vec<String> {
        password.to_owned()
    }
}