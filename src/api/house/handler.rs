use crate::api::house::request::{HouseCreateRequest, HouseInviteUserRequest, HouseRevokeUserRequest, HouseUpdateRequest};
use crate::api::house::response::{HouseCreatedResponse, HouseDeleteResponse, HouseInvitation, HouseInviteResponse, HouseListInvitation, HouseListResponse, HouseResponse, HouseRevokeResponse};
use crate::api::user::response::UserListResponse;
use crate::domain::house::HouseDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, Path, ServiceConfig};
use permission::house::HousePermission;
use permission::resource::Resource;
use tracing::instrument;
use permission::expense::ExpensePermission;
use crate::domain::expense::ExpenseDomain;
use crate::extractors::pagination::Pagination;

pub struct HouseRoute;

impl HouseRoute {
    #[instrument(skip(session))]
    pub async fn get_house(session: AuthSession, db: DbReq, path: Path<String>) -> Result<HouseResponse, Error> {
        session.enforce(Resource::House(HousePermission::ReadSelf)).await?;
        let house = HouseDomain::get_house(&session, &db.clone().into_inner(), &path.into_inner().clone()).await?;
        Ok(HouseResponse::new(house.0, Some(house.1)))
    }

    #[instrument(skip(session))]
    pub async fn create_house(session: AuthSession, db: DbReq, body: Dto<HouseCreateRequest>) -> Result<HouseCreatedResponse, Error> {
        session.enforce(Resource::House(HousePermission::Create)).await?;
        let created_house =HouseDomain::create_house(session, body.into_inner(), db.into_inner()).await?;
        Ok(HouseCreatedResponse::new(created_house))
    }

    #[instrument(skip(session, db))]
    pub async fn list_house(session: AuthSession, db: DbReq, pag: Pagination) -> Result<HouseListResponse, Error> {
        session.enforce(Resource::House(HousePermission::List)).await?;
        let houses = HouseDomain::list_house(session, db.into_inner(), pag).await?;
        Ok(HouseListResponse::new(houses))
    }

    #[instrument(skip(session, db))]
    pub async fn list_users(session: AuthSession, db: DbReq, path: Path<String>, pag: Pagination) -> Result<UserListResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::ReadSelf), Resource::House(HousePermission::Read)]).await?;
        let house_with_users = HouseDomain::get_active_house_with_users(&db.into_inner(), (&path.into_inner(), pag)).await?;
        Ok(UserListResponse::new(house_with_users.1))
    }

    #[instrument(skip(session, db))]
    pub async fn invite_users(session: AuthSession, db: DbReq, path: Path<String>, body: Dto<HouseInviteUserRequest>,) -> Result<HouseInviteResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::InviteSelf), Resource::House(HousePermission::Invite)]).await?;
        HouseDomain::invite_users(session, db.into_inner(), &path.into_inner(), &body.into_inner().users).await?;
        Ok(HouseInviteResponse {})
    }

    #[instrument(skip(session, db))]
    pub async fn revoke_users(session: AuthSession, db: DbReq, path: Path<String>, body: Dto<HouseRevokeUserRequest>) -> Result<HouseRevokeResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::RevokeSelf), Resource::House(HousePermission::Revoke)]).await?;
        HouseDomain::revoke_users(session, db.into_inner(), &path.into_inner(), &body.into_inner().users).await?;
        Ok(HouseRevokeResponse {})
    }

    #[instrument(skip(_session))]
    pub async fn get_invitation(_session: AuthSession, _db: DbReq, _path: Path<(String, String)>) -> Result<String, Error> {
        Ok("".to_string())
    }

    #[instrument(skip(session))]
    pub async fn list_invitation(session: AuthSession, db: DbReq, path: Path<String>) -> Result<HouseListInvitation, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::ListInvitationSelf), Resource::House(HousePermission::ListInvitation)]).await?;
        let invitations = HouseDomain::list_users(session, db.into_inner(), &path.into_inner()).await?;
        Ok(HouseListInvitation(invitations.into_iter().map(|(i, u)| HouseInvitation::from_model(i, u)).collect()))
    }

    #[instrument(skip(session, db))]
    pub async fn accept_invitation(session: AuthSession, db: DbReq, path: Path<(String, String)>) -> Result<HouseInviteResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::AcceptInvitationSelf), Resource::House(HousePermission::AcceptInvitation)]).await?;
        HouseDomain::accept_invitation(session, db.into_inner(), &path.into_inner().0).await?;
        Ok(HouseInviteResponse {})
    }

    #[instrument(skip(session, db))]
    pub async fn decline_invitation(session: AuthSession, db: DbReq, path: Path<(String, String)>) -> Result<HouseInviteResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::DeclineInvitationSelf), Resource::House(HousePermission::DeclineInvitation)]).await?;
        HouseDomain::decline_invitation(session, db.into_inner(), &path.into_inner().0).await?;
        Ok(HouseInviteResponse {})
    }

    #[instrument(skip(session))]
    pub async fn update_house(session: AuthSession, db: DbReq, path: Path<String>, body: Dto<HouseUpdateRequest>) -> Result<HouseResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::UpdateSelf), Resource::House(HousePermission::Update)]).await?;
        let house = HouseDomain::update_house(session, db.into_inner(), path.into_inner(), body.into_inner()).await?;
        Ok(HouseResponse::new(house, None))
    }

    #[instrument(skip(session))]
    pub async fn delete_house(session: AuthSession, db: DbReq, path: Path<String>) -> Result<HouseDeleteResponse, Error> {
        session.enforce_or(vec![Resource::House(HousePermission::Delete), Resource::House(HousePermission::DeleteSelf)]).await?;
        HouseDomain::delete_house(session, db.into_inner(), path.into_inner()).await?;
        Ok(HouseDeleteResponse {})
    }

    #[instrument(skip(session, db))]
    pub async fn list_expense(session: AuthSession, db: DbReq, house_id: Path<String>) -> Result<String, Error> {
        session.enforce(Resource::Expense(ExpensePermission::List)).await?;
        ExpenseDomain::list_expense(&db.into_inner(), &house_id.into_inner()).await?;
        Ok("".to_string())
    }

    #[instrument(skip(_session))]
    pub async fn list_credentials(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for HouseRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/house")
                .route("", get().to(Self::list_house))
                .route("", post().to(Self::create_house))
                .route("{house_id}", put().to(Self::update_house))
                .route("{house_id}", get().to(Self::get_house))
                .route("{house_id}", delete().to(Self::delete_house))
                .route("{house_id}/users", post().to(Self::invite_users))
                .route("{house_id}/users", delete().to(Self::revoke_users))
                .route("{house_id}/users", get().to(Self::list_invitation))
                .route("{house_id}/users/{userId}", delete().to(Self::get_invitation))
                .route("{house_id}/users/{userId}/accept", delete().to(Self::accept_invitation))
                .route("{house_id}/users/{userId}/decline", delete().to(Self::decline_invitation))
                .route("{house_id}/expenses", delete().to(Self::list_expense))
                .route("{house_id}/credentials", delete().to(Self::list_credentials)),
        );
    }
}
