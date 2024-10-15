use actix_web::{App, HttpServer};
use tracing_subscriber::filter::EnvFilter;
use crate::api::auth::handler::AuthRoute;
use crate::utils::route::Route;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use crate::api::user::handler::UserRoute;

mod db;
mod test_helpers;
mod utils;
mod api;
mod extractors;
mod exceptions;

#[allow(warnings, unused)]
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    
    client.unwrap().user().find_many(vec![]).exec().await.unwrap();
    

    /* // Create a new OpenTelemetry trace pipeline that prints to stdout
     let provider = TracerProvider::builder()
         .with_simple_exporter(stdout::SpanExporter::default())
         .build();
     let tracer = provider.tracer("readme_example");
     let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
 
 
     tracing_subscriber::registry()
         .with(tracing_subscriber::fmt::layer().with_line_number(false).with_file(false))
         .with(EnvFilter::from_default_env())*/
    tracing::info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .configure(AuthRoute::route)
            .configure(UserRoute::route)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
