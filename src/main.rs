use actix_files::NamedFile;
use actix_web::{App, HttpServer, Responder, web};

async fn index() -> impl Responder {
    NamedFile::open_async("./frontend/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("✅ Server running at: http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/assets", "./frontend/assets").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
