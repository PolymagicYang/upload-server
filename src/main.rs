use actix_web::{App, HttpServer, web};
use upload_server::controllers;
// use rustls::{ServerConfig, server::AllowAnyAuthenticatedClient, RootCertStore};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: move to dotenv.
    let listened_addr = "127.0.0.1:8292";
   
    /* 
    let roots = RootCertStore::empty();
   
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_client_cert_verifier(AllowAnyAuthenticatedClient::new(roots))
        .with_single_cert();
    */
    
    let server = HttpServer::new(|| {
        App::new()
            .service(controllers::upload_controller::upload)
    }).bind(listened_addr)?;
    
    server.run().await
}
