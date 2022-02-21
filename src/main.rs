use actix_web::{App, HttpServer, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: move to dotenv.
    let listened_addr = "127.0.0.1:8292";
    
    let mut builder = 
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("ssl cannot be initialized.");
    
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get())
            .route("/user", web::post())
    }).bind_openssl(listened_addr, builder)?;
    
    server.run().await
}
