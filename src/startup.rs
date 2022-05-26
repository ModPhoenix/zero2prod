use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::{
    email_client::EmailClient,
    routes::{health_check, subscribe},
};

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
