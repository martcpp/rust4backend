use actix_web::{web, App, HttpServer, Responder,HttpResponse,HttpRequest};


async fn index(req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(index))
    })
    .bind("localhost:3000")?
    .run()
    .await
   
}
