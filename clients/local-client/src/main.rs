use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(move || {
        App::new()
            .service(
                actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:7778")
    .expect("Failed to bind to socket")
    .run()
    .await
    .expect("Could not start local client server");

}
