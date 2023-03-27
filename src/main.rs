use actix_web::web::scope;
use actix_web::{App, HttpServer};
use libs::posicube::sample::agent_super_configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(scope("/api").configure(|cfg| agent_super_configure(cfg)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
