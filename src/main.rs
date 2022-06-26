use std::net::TcpListener;
use newsletter_app::startup::run;
use newsletter_app::configuration::get_configuration;
use sqlx::{Connection, PgConnection};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
