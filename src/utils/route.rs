pub trait Route {
    fn route(cfg: &mut actix_web::web::ServiceConfig);
}
