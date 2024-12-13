use crate::api::house::request::{HouseCreateRequest, HouseUpdateRequest};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use entity::house::Model;
use entity::prelude::*;
use entity::sea_orm_active_enums::HouseUserStatus;
use entity::{house_user, user};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, JoinType, QueryFilter, QuerySelect, QueryTrait, RelationTrait};
use permission::house::HousePermission;
use permission::resource::Resource;
use crate::extractors::pagination::Pagination;
use crate::services::entity::WithPagination;

pub struct HouseDomain;

type HouseWithUsers = (Model, Vec<user::Model>);

impl HouseDomain {
    pub async fn find_active_house_by_id(house_id: &str, db: &DatabaseConnection) -> Result<Model, Error> {
        House::find_by_id(house_id)
            .filter(entity::house::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("House", house_id.to_owned())))
    }

    pub async fn get_active_house_with_users(db: &DatabaseConnection, (house_id, pagination): (&str, Pagination)) -> Result<HouseWithUsers, Error> {
        let house_with_users = House::find_by_id(house_id)
            .filter(entity::house::Column::DeletedAt.is_null())
            .with_pagination(pagination)
            .find_with_related(user::Entity)
            .all(db)
            .await?;
        let (house, users) = match house_with_users.get(0) {
            Some(house) => Ok(house.to_owned()),
            None => Err(Error::Entity(EntityError::NotFound("House", house_id.to_owned()))),
        }?;
        Ok((house, users))
    }

    pub async fn create_house(session: &AuthSession, body: HouseCreateRequest) -> Result<Model, Error> {
        let mut house = body.into_model();
        house.created_by = Set(session.user.id.clone());
        Ok(house.insert(&session.db).await?)
    }

    pub async fn list_house(session: &AuthSession, pag: Pagination) -> Result<Vec<Model>, Error> {
        let house = entity::house::Entity::find()
            .with_pagination(pag)
            .join(JoinType::LeftJoin, entity::house::Relation::HouseUser.def())
            .filter(entity::house::Column::DeletedAt.is_null())
            .filter(Condition::any()
                .add(entity::house::Column::CreatedBy.eq(&session.user.id))
                .add(house_user::Column::UserId.eq(&session.user.id)))
            .all(&session.db).await?;
        Ok(house)
    }

    pub async fn invite_users(session: &AuthSession, house_id: &String, users: &Vec<String>) -> Result<(), Error> {
        let house = Self::find_active_house_by_id(house_id, &session.db).await?;
        let users = user::Entity::find()
            .filter(user::Column::Email.is_in(users))
            .filter(user::Column::Id.ne(house.created_by))
            .filter(
                user::Column::Id.not_in_subquery(
                    house_user::Entity::find()
                        .select_only()
                        .column(house_user::Column::UserId)
                        .filter(house_user::Column::HouseId.eq(house_id))
                        .into_query(),
                ),
            )
            .all(&session.db)
            .await?;
        HouseUser::insert_many(
            users
                .iter()
                .map(|user| house_user::ActiveModel {
                    user_id: Set(user.id.clone()),
                    house_id: Set(house_id.clone()),
                    invited_at: Set(chrono::Utc::now().naive_utc()),
                    status: Set(HouseUserStatus::Pending.into()),
                    ..Default::default()
                }).collect::<Vec<house_user::ActiveModel>>(),
        )
        .on_empty_do_nothing()
        .exec(&session.db)
        .await?;
        Ok(())
    }

    pub async fn revoke_users(session: AuthSession, house_id: &String, users: &Vec<String>) -> Result<(), Error> {
        Self::find_active_house_by_id(house_id, &session.db).await?;
        house_user::Entity::delete_many()
            .filter(house_user::Column::HouseId.eq(house_id))
            .filter(
                house_user::Column::UserId.in_subquery(
                    user::Entity::find()
                        .filter(user::Column::Email.is_in(users))
                        .select_only()
                        .column(user::Column::Id)
                        .into_query(),
                ),
            )
            .exec(&session.db)
            .await?;
        Ok(())
    }
    
    pub async fn accept_invitation(session: AuthSession, house_id: &str) -> Result<(), Error> {
        Self::update_invitation(session, (house_id, HousePermission::AcceptInvitation, HouseUserStatus::Accepted)).await
    }
    
    pub async fn decline_invitation(session: AuthSession, house_id: &str) -> Result<(), Error>{
        Self::update_invitation(session, (house_id, HousePermission::DeclineInvitation, HouseUserStatus::Declined)).await
    }
    
    async fn update_invitation(session: AuthSession, (house_id, perm, status): (&str, HousePermission, HouseUserStatus)) -> Result<(), Error> {
        let house_user = house_user::Entity::find()
            .filter(house_user::Column::HouseId.eq(house_id).and(house_user::Column::UserId.eq(&session.user.id)))
            .one(&session.db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("HouseUser", house_id.to_owned())))?;
        match house_user.status == HouseUserStatus::Pending {
            true => (),
            false => session.enforce(Resource::House(perm)).await?
        }
        let mut model = house_user.into_active_model();
        model.accepted_at = Set(Some(chrono::Utc::now().naive_utc()));
        model.status = Set(status.into());
        model.update(&session.db).await?;
        Ok(())
    }
    
    pub async fn list_users(session: AuthSession, house_id: &str) -> Result<Vec<(house_user::Model, user::Model)>, Error> {
        Self::_get_house(&session, house_id, &session.user.id, HousePermission::ListInvitation).await?;
        Ok(house_user::Entity::find()
            .find_also_related(user::Entity)
            .filter(house_user::Column::HouseId.eq(house_id))
            .all(&session.db)
            .await?
            .into_iter().filter_map(|(invitation, user)| {
            match user {
                Some(user) => Some((invitation, user)),
                None => None
            }
        }).collect())
    }
    
    async fn _get_house(session: &AuthSession, house_id: &str, _user_id: &str, _perm: HousePermission) -> Result<Model, Error> {
        match Self::is_in_house(&session.db, &session.user.id, house_id).await? {
            (h, false) => {
                session.enforce(Resource::House(HousePermission::ListInvitation)).await?;
                Ok(h)
            },
            (h, _) => Ok(h),
        }
    }
    
    pub async fn get_invitation(db: DatabaseConnection, (house_id, user_id): (&str, &str)) -> Result<house_user::Model, Error> {
        Ok(house_user::Entity::find()
            .filter(house_user::Column::HouseId.eq(house_id).and(house_user::Column::UserId.eq(user_id)))
            .one(&db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("HouseUser", house_id.to_owned())))?)
    }

    pub async fn update_house(session: AuthSession, house_id: String, body: HouseUpdateRequest) -> Result<Model, Error> {
        let house_with_users = Self::get_active_house_with_users(&session.db, (&house_id, Pagination::all())).await?;
        Self::need_to_be_members(&house_with_users, &session)?;
        let mut model = house_with_users.0.into_active_model();
        model.address = body.address.map_or(model.address, |address| Set(Some(address)));
        model.name = body.name.map_or(model.name, |name| Set(name));
        model.description = body.description.map_or(model.description, |description| Set(Some(description)));
        model.location_type = body.location_type.map_or(model.location_type, |location_type| Set(location_type.into()));
        model.owner_name = body.owner_name.map_or(model.owner_name, |owner_name| Set(Some(owner_name)));
        model.owner_contact_info = body.owner_contact_info.map_or(model.owner_contact_info, |owner_contact_info| Set(Some(owner_contact_info)));
        model.owner_email = body.owner_email.map_or(model.owner_email, |owner_email| Set(Some(owner_email)));
        model.owner_phone = body.owner_phone.map_or(model.owner_phone, |owner_phone| Set(Some(owner_phone)));
        model.built_year = body.built_year.map_or(model.built_year, |built_year| Set(Some(built_year)));
        model.acquired_at = body.acquired_at.map_or(model.acquired_at, |acquired_at| Set(Some(acquired_at)));
        model.updated_by = Set(Some(session.user.id));
        model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(model.update(&session.db).await?)
    }

    pub async fn delete_house(session: AuthSession, house_id: String) -> Result<(), Error> {
        let house = Self::find_active_house_by_id(&house_id, &session.db).await?;
        match house.created_by == session.user.id {
            true => (),
            false => session.enforce(Resource::House(HousePermission::Delete)).await?
        }
        let mut active_house = house.into_active_model();
        active_house.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
        active_house.deleted_by = Set(Some(session.user.id));
        active_house.update(&session.db).await?;
        Ok(())
    }

    pub fn need_to_be_owner(house: &Model, session: &AuthSession) -> Result<(), Error> {
        if house.created_by != session.user.id {
            return Err(Error::Entity(EntityError::NeedToBeOwner(
                "House".to_string(),
            )));
        }
        Ok(())
    }

    pub fn need_to_be_members(house_with_user: &(Model, Vec<user::Model>), session: &AuthSession) -> Result<(), Error> {
        let (_, users) = house_with_user;
        let user_ids: Vec<String> = users.iter().map(|user| user.id.clone()).collect();
        if !user_ids.contains(&session.user.id) && house_with_user.0.created_by != session.user.id {
            return Err(Error::Entity(EntityError::NoPermission(
                "House".to_string(),
            )));
        }
        Ok(())
    }

    pub async fn get_house(session: &AuthSession, house_id: &str) -> Result<HouseWithUsers, Error> {
        let (house, users) = Self::get_active_house_with_users(&session.db, (&house_id, Pagination::all())).await?;
        if house.created_by == session.user.id || users.iter().find(|user| user.id == session.user.id).is_some() {
            Ok((house, users))
        } else {
            session.enforce(Resource::House(HousePermission::Read)).await?;
            Ok((house, users))
        }
    }
    
    pub async fn is_in_house(db: &DatabaseConnection, user_id: &str, house_id: &str) -> Result<(Model, bool), Error> {
        let house = Self::find_active_house_by_id(house_id, db).await?;
        match house.created_by == user_id {
            true => Ok((house, true)),
            false => {
                match house_user::Entity::find()
                    .filter(house_user::Column::HouseId.eq(house_id).and(house_user::Column::UserId.eq(user_id)).and(house_user::Column::Status.eq(HouseUserStatus::Accepted)))
                    .one(db)
                    .await {
                    Ok(Some(_)) => Ok((house, true)),
                    _ => Ok((house, false))
                }
            }
        }
    }
}
